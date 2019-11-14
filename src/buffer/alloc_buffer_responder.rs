use crate::{types::{OscMessage, OscType, NodeValue}, AfterCallAction, OscResponder, ScClientResult};
use std::sync::Mutex;

pub struct AllocBufferResponder<F: Fn(i32) + Send + Sync + 'static> {
    on_reply_callback: F,
    buffer_id: i32,
    after_call_action: Mutex<AfterCallAction>,
}

impl<F: Fn(i32) + Send + Sync + 'static> AllocBufferResponder<F> {
    pub fn new(buffer_id: i32, on_reply_callback: F) -> Self {
        AllocBufferResponder {
            on_reply_callback,
            buffer_id,
            after_call_action: Mutex::new(AfterCallAction::Reschedule)
        }
    }
}

impl<F: Fn(i32) + Send + Sync + 'static> OscResponder for AllocBufferResponder<F> {
    fn callback(&self, message: &OscMessage) -> ScClientResult<()> {
        if let Some(ref args) = message.args {
            if args[1] == OscType::Int(self.buffer_id) {
                (self.on_reply_callback)(self.buffer_id);
                *self.after_call_action
                    .lock()
                    .unwrap() = AfterCallAction::None;
            }
        }
        Ok(())
    }

    fn get_address(&self) -> String {
        String::from("/b_alloc")
    }

    fn get_after_call_action(&self, message: &OscMessage) -> AfterCallAction {
        (*self.after_call_action.lock().unwrap()).clone()
    }
}
