use std::{
    net::TcpStream,
    ops::{Deref, DerefMut},
};

#[derive(Debug)]
pub struct Transport(TcpStream);

impl Transport {
    /// 新建一个传输
    pub fn new(socket: TcpStream) -> Self {
        Self(socket)
    }
}

impl Deref for Transport {
    type Target = TcpStream;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Transport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
