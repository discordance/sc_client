mod alloc_buffer_responder;
use self::alloc_buffer_responder::AllocBufferResponder;
use crate::{types::NodeValue, types::OscType, ScClientResult, Server};
use uid::Id;

pub struct Buffer<'a> {
  id: i32,
  server: &'a Server,
}

impl<'a> Buffer<'a> {
  // nothing get allocated for now
  pub fn new(server: &'a Server) -> ScClientResult<Self> {
    let id = Buffer::init_id();
    let buff = Buffer { id, server };
    Ok(buff)
  }

  pub fn allocate<F>(&self, num_frames: i32, num_channels: i32, on_reply: F) -> ScClientResult<()>
  where
    F: Fn(i32) + Send + Sync + 'static,
  {
    // sets the responder
    let responder = AllocBufferResponder::new(self.id, on_reply);
    self
      .server
      .osc_server
      .borrow_mut()
      .add_responder(responder)?;

    let send_args: Vec<OscType> = vec![self.id.into(), num_frames.into(), num_channels.into()];
    self
      .server
      .osc_server
      .borrow()
      .send_message("/b_alloc", Some(send_args))?;
    Ok(())
  }

  fn init_id() -> i32 {
    let id = Id::<i32>::new();
    id.get() as i32
  }

  pub fn get_id(&self) -> i32 {
    self.id
  }
}
