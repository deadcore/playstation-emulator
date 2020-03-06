extern crate env_logger;
extern crate gdb_remote_protocol;
#[macro_use]
extern crate log;

use std::{borrow::Cow, net::TcpListener};
use std::iter;

use gdb_remote_protocol::{
    MemoryRegion, ThreadId, VCont, VContFeature,
};
use gdb_remote_protocol::{Error, Handler, process_packets_from, ProcessType, StopReason};

pub type Result<T, E = Error> = std::result::Result<T, E>;

struct NoopHandler {
    pub rax: Option<u64>,
    pub rbx: Option<u64>,
    pub rcx: Option<u64>,
    pub rdx: Option<u64>,
    pub rsi: Option<u64>,
    pub rdi: Option<u64>,
    pub rbp: Option<u64>,
    pub rsp: Option<u64>,
}

impl Handler for NoopHandler {
    fn attached(&self, _pid: Option<u64>) -> Result<ProcessType, Error> {
        Ok(ProcessType::Created)
    }

    fn halt_reason(&self) -> Result<StopReason, Error> {
        Ok(StopReason::Signal(0))
    }

    fn read_general_registers(&self) -> Result<Vec<u8>, Error> {
        let mut output = Vec::new();

        let mut write = |slice: Option<&[u8]>, len: usize| {
            output.reserve(len);
            if let Some(slice) = slice {
                assert_eq!(slice.len(), len);
                output.extend_from_slice(slice);
            } else {
                output.extend(iter::repeat(0).take(len));
            }
        };

        write(self.rax.map(u64::to_le_bytes).as_ref().map(|s| &s[..]), 8);
        write(self.rbx.map(u64::to_le_bytes).as_ref().map(|s| &s[..]), 8);
        write(self.rcx.map(u64::to_le_bytes).as_ref().map(|s| &s[..]), 8);
        write(self.rdx.map(u64::to_le_bytes).as_ref().map(|s| &s[..]), 8);
        write(self.rsi.map(u64::to_le_bytes).as_ref().map(|s| &s[..]), 8);
        write(self.rdi.map(u64::to_le_bytes).as_ref().map(|s| &s[..]), 8);
        write(self.rbp.map(u64::to_le_bytes).as_ref().map(|s| &s[..]), 8);
        write(self.rsp.map(u64::to_le_bytes).as_ref().map(|s| &s[..]), 8);

        Ok(output)
    }
}

#[cfg_attr(test, allow(dead_code))]
fn main() {
    env_logger::builder()
//        .default_format_level(false)
//        .default_format_module_path(false)
//        .default_format_timestamp(false)
        .init();

    let listener = TcpListener::bind("0.0.0.0:2424").unwrap();
    info!("Listening on port 2424");
    for res in listener.incoming() {
        info!("Got connection");
        if let Ok(stream) = res {
            process_packets_from(stream.try_clone().unwrap(), stream, NoopHandler {
                rax: Some(0xbfc00000),
                rbx: Some(0xbfc00000),
                rcx: Some(0xbfc00000),
                rdx: Some(0xbfc00000),
                rsi: Some(0xbfc00000),
                rdi: Some(0xbfc00000),
                rbp: Some(0xbfc00000),
                rsp: Some(0xbfc00000),
            });
        }
        info!("Connection closed");
    }
}