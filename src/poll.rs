use crate::ffi;
use std::{
    io::{self, Result},
    net::TcpStream, os::fd::AsRawFd,
};

type Events = Vec<ffi::Event>;

pub struct Poll {
    registry: Registry,
}

impl Poll {
    pub fn new() -> Result<Self> {
        let efd = unsafe { ffi::epoll_create(1) };
        if efd < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(Self {
            registry: Registry { raw_fd: efd },
        })
    }

    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    pub fn poll(&mut self, events: &mut Events, timeout: Option<i32>) -> Result<()> {
        let read = unsafe {
            ffi::epoll_wait(
                self.registry.raw_fd,
                events.as_mut_ptr(),
                events.capacity() as i32,
                timeout.unwrap_or(-1),
            )
        };
        if read < 0 {
            return Err(io::Error::last_os_error());
        }
        unsafe { events.set_len(read as usize) };
        Ok(())
    }
}

pub struct Registry {
    raw_fd: i32,
}

impl Registry {
    pub fn register(&self, source: &TcpStream, token: usize, interests: i32) -> Result<()> {
        let mut event = ffi::Event {
            events: interests as u32,
            epoll_data: token,
        };
        let op = ffi::EPOLL_CTL_ADD;
        let res = unsafe {
            ffi::epoll_ctl(self.raw_fd, op, source.as_raw_fd(), &mut event)
        };

        if res < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        
        let res = unsafe {
            ffi::close(self.raw_fd)
        };
        if res < 0 {
            let err = io::Error::last_os_error();
            eprintln!("ERROR: {err:?}");
        }
    }
}
