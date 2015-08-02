use libc::{open, close, read, write, O_RDWR, O_NOCTTY, c_void};

use std::io;
use std::os::unix::io::{RawFd, AsRawFd};
use std::ffi::CString;

#[cfg(target_os="macos")]
use termios::os::macos::*;
#[cfg(not(target_os="macos"))]
use termios::os::linux::*;

use termios::{cfsetispeed, cfsetospeed, tcsetattr, tcdrain, Termios};

pub struct NativeConnection(RawFd);

impl NativeConnection {
    #[inline]
    pub fn from_fd(fd: RawFd) -> NativeConnection {
        NativeConnection(fd)
    }

    pub fn new(device_name: &str) -> Result<NativeConnection, io::Error> {
        // TODO: (use O_NONBLOCK if necessary)
        let port = unsafe {
            open(CString::new(device_name).unwrap().as_ptr(), O_NOCTTY, O_RDWR as u32)
        };

        if port == -1 {
            return Err(io::Error::last_os_error());
        }

        // This ensures the file descriptor is closed if not returned
        let connection = NativeConnection(port);

        let mut device_data = try!(Termios::from_fd(port));

        // setup TTY for binary serial port access
        termios.c_cflag |= CREAD | CLOCAL;
        termios.c_lflag &= !(ICANON | ECHO | ECHOE | ECHOK | ECHONL | ISIG | IEXTEN);
        termios.c_oflag &= !OPOST;
        termios.c_iflag &= !(INLCR | IGNCR | ICRNL | IGNBRK);

        // Make the changes and flush the buffers
        try!(tcsetattr(port, TCSAFLUSH, &device_data));

        Ok(connection)
    }
}

impl Drop for NativeConnection {
    fn drop(&mut self) {
        unsafe {
            close(self.0);
        }
    }
}

impl AsRawFd for NativeConnection {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

impl io::Read for NativeConnection {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let result = unsafe {
            read(self.as_raw_fd(), buf.as_mut_ptr() as *mut c_void, buf.len() as u64)
        };

        if result == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(result as usize)
        }
    }
}

impl io::Write for NativeConnection {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let result = unsafe {
            write(self.as_raw_fd(), buf.as_ptr() as *const c_void, buf.len() as u64)
        };

        if result == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(result as usize)
        }
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        tcdrain(self.as_raw_fd())
    }
}

pub fn connect(device_name: &str) -> Result<NativeConnection, io::Error> {
    // TODO: (use O_NONBLOCK if necessary)
    let port = unsafe {
        open(CString::new(device_name).unwrap().as_ptr(), O_NOCTTY, O_RDWR as u32)
    };

    if port == -1 {
        return Err(io::Error::last_os_error());
    }

    // This ensures the file descriptor is closed if not returned
    let connection = NativeConnection(port);

    let mut device_data = try!(Termios::from_fd(port));

    // setup TTY for binary serial port access
    termios.c_cflag |= CREAD | CLOCAL;
    termios.c_lflag &= !(ICANON | ECHO | ECHOE | ECHOK | ECHONL | ISIG | IEXTEN);
    termios.c_oflag &= !OPOST;
    termios.c_iflag &= !(INLCR | IGNCR | ICRNL | IGNBRK);

    // Make the changes and flush the buffers
    try!(tcsetattr(port, TCSAFLUSH, &device_data));

    Ok(connection)
}
