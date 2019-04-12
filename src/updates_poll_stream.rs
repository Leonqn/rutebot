use std::cmp::max;
use std::collections::VecDeque;

use futures::Async;
use futures::Future;
use futures::Stream;

use std::i64;
use crate::error::Error;
use crate::responses::Update;

pub struct UpdatesPoolStream<Fut, Sender> {
    pub send_request: Sender,
    pub buffer: VecDeque<Update>,
    pub executing_request: Fut,
    pub is_canceled: bool,
    pub last_id: Option<i64>,
    pub has_error: bool,
}

impl<Fut, Sender> Stream for UpdatesPoolStream<Fut, Sender>
    where Fut: Future<Item=Vec<Update>, Error=Error>,
          Sender: Fn(Option<i64>) -> Fut {
    type Item = Update;
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
        if self.is_canceled {
            return Ok(Async::Ready(None));
        }
        if let Some(update) = self.buffer.pop_front() {
            return Ok(Async::Ready(Some(update)));
        }
        if self.has_error {
            self.has_error = false;
            self.executing_request = (self.send_request)(self.last_id)
        }
        match self.executing_request.poll() {
            Ok(Async::NotReady) =>
                Ok(Async::NotReady),

            Ok(Async::Ready(updates)) => {
                let last_id = self.last_id.unwrap_or(-1);
                for update in updates {
                    self.last_id = Some(max(update.update_id, last_id) + 1);
                    self.buffer.push_back(update)
                }
                self.executing_request = (self.send_request)(self.last_id);
                self.poll()
            }
            Err(err) => {
                self.has_error = true;
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