extern crate libc;
// extern crate euclid;

#[cfg(not(target_os="windows"))]
extern crate termios;

pub mod platform;

use platform::NativeConnection;
use platform::{connect};
use std::io;

pub struct SerialPort {
    connection: NativeConnection,
}

pub struct SerialPortParams {
    pub baud_rate: BaudRate,
    pub char_size: CharSize,
    pub parity: Parity
}

impl SerialPort {
    #[inline]
    pub fn new(connection: NativeConnection) -> Self {
        SerialPort {
            connection: connection,
        }
    }

    pub fn from_port(name: &str) -> io::Result<Self> {
        let connection = try!(NativeConnection::new(name));
        Ok(SerialPort::new(connection))
    }

    pub fn set_params(&mut self, params: SerialPortParams) -> io::Result<()> {

    }

    pub fn native_connection(&self) -> &NativeConnection {
        &self.connection
    }
}

impl io::Read for SerialPort {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.connection.read(buf)
    }
}

impl io::Write for SerialPort {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.connection.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.connection.flush()
    }
}
