use crate::{Result, FrontendService, BackendService, History};

pub enum Service {
    Frontend(FrontendService),
    Backend(BackendService),
}

impl ServiceExt for Service {
  fn has_history(&self) -> bool{
      match *self {
          Self::Frontend(ref x) => x.has_history(),
          Self::Backend(ref x) => x.has_history(),
      }
  }
  fn get_history(&self) -> Option<History>{
      match *self {
          Self::Frontend(ref x) => x.get_history(),
          Self::Backend(ref x) => x.get_history(),
      }
  }
  fn needs_auth(&self) -> bool{
      match *self {
          Self::Frontend(ref x) => x.needs_auth(),
          Self::Backend(ref x) => x.needs_auth(),
      }
  }
  fn try_auth(&self) -> Result<String>{
      match *self {
          Self::Frontend(ref x) => x.try_auth(),
          Self::Backend(ref x) => x.try_auth(),
      }
  }
  fn test_request(&self) -> Result<()>{
      match *self {
          Self::Frontend(ref x) => x.test_request(),
          Self::Backend(ref x) => x.test_request(),
      }
  }
}


pub trait ServiceExt {
  fn has_history(&self) -> bool {
      todo!();
  }
  fn get_history(&self) -> Option<History> {
      todo!();
  }
  fn needs_auth(&self) -> bool{
      todo!();
  }
  fn try_auth(&self) -> Result<String>{
      todo!();
  }
  fn test_request(&self) -> Result<()>{
      todo!();
  }
}

pub trait FrontendServiceExt {
    fn post(&self, reply: String) -> Result<()> {
        todo!();
    }
}

pub trait BackendServiceExt {
    fn request(&self, history: History, input: String) -> Result<String> {
        todo!();
    }
}


