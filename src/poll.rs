use crate::ffi;
use std::{io::Result, net::TcpStream};

type Events = Vec<ffi::Event>;

pub struct Poll {
    registry: Registry
}

impl Poll {
    pub fn new() -> Result<Self> {
        todo!()
    }

    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    pub fn poll(&mut self, events: &mut Events, timeout: Option<i32>) -> Result<()> {
        todo!()
    }
}
 
pub struct Registry {
    raw_fd: i32
}

impl Registry {
    pub fn registry(&self, source: TcpStream, token: usize, interests: i32) -> Result<()> {
        todo!()
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        todo!();
    }
}