use std::cmp::{max, min};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

use futures::Future;
use futures::Stream;
use futures::{Async, Poll};
use tokio::timer::Delay;

use crate::error::Error;
use crate::responses::Update;

pub struct UpdatesPoolStream<Fut, Sender> {
    pub send_request: Sender,
    pub buffer: VecDeque<Update>,
    pub executing_request: Fut,
    pub is_canceled: bool,
    pub last_id: Option<i64>,
    pub retry_delay: Option<Delay>,
    pub max_retry_delay_sec: u8,
    pub retries: u16,
}

impl<Fut, Sender> Stream for UpdatesPoolStream<Fut, Sender>
where
    Fut: Future<Item = Vec<Update>, Error = Error>,
    Sender: Fn(Option<i64>) -> Fut,
{
    type Item = Update;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if self.is_canceled {
            return Ok(Async::Ready(None));
        }
        if let Some(update) = self.buffer.pop_front() {
            return Ok(Async::Ready(Some(update)));
        }
        if let Some(retry_delay) = &mut self.retry_delay {
            match retry_delay.poll() {
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                _ => {
                    self.retry_delay = None;
                    self.executing_request = (self.send_request)(self.last_id);
                }
            };
        }
        match self.executing_request.poll() {
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Ok(Async::Ready(updates)) => {
                let last_id = self.last_id.unwrap_or(-1);
                for update in updates {
                    self.last_id = Some(max(update.update_id, last_id) + 1);
                    self.buffer.push_back(update)
                }
                self.retries = 0;
                self.executing_request = (self.send_request)(self.last_id);
                self.poll()
            }
            Err(err) => {
                let retry_delay = min(self.max_retry_delay_sec, 2u8.pow(self.retries.into()));
                let deadline = Instant::now() + Duration::from_secs(retry_delay.into());

                self.retry_delay = Some(Delay::new(deadline));
                self.retries += 1;

                Err(err)
            }
        }
    }
}

impl<Fut, Sender> Drop for UpdatesPoolStream<Fut, Sender> {
    fn drop(&mut self) {
        self.is_canceled = true;
    }
}
