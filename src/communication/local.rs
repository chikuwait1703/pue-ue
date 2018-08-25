use std::fs::remove_file;
use std::path::Path;
use std::io::Error as io_Error;

use futures::{Future, Poll};
use tokio::prelude::*;
use tokio_uds::{UnixListener, UnixStream};

use settings::Settings;
use communication::message::MessageType;

/// Create a new unix listener.
/// In case a socket already exists it will be removed
pub fn get_unix_listener(settings: &Settings) -> UnixListener {
    let socket_path = get_socket_path(&settings);

    // Remove old socket
    if Path::new(&socket_path).exists() {
        remove_file(&socket_path).expect("Failed to remove old socket.");
        println!("Remove old socket.");
    }

    println!("Creating socket at {}", socket_path);

    UnixListener::bind(socket_path).expect("Failed to create unix socket.")
}

/// Helper function to create the socket path used by clients and the daemon.
pub fn get_socket_path(settings: &Settings) -> String {
    let path = Path::new(settings.common.local_socket_dir.as_str())
        .join(format!("pueue_{}.sock", settings.common.group_id));

    path.as_path()
        .to_str()
        .expect("Unable to create socket path.")
        .to_string()
}

pub struct ReceiveInstruction {
    pub instruction_type: MessageType,
    pub read_instruction_future: Box<Future<Item = (UnixStream, Vec<u8>), Error = io_Error> + Send>,
}

impl Future for ReceiveInstruction {
    type Item = (MessageType, String, UnixStream);
    type Error = String;

    /// Poll for a received instruction
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {

        // Check if we received the instruction
        let result = self.read_instruction_future.poll();

        // The socket errored, return the error
        if result.is_err() {
            println!("{:?}", result.err());
            return Err("Socket errored during read".to_string());
        }

        match result.unwrap() {
            // We received an instruction from a client. Handle it
            Async::Ready((stream, instruction_bytes)) => {
                // Extract instruction and handle invalid utf8
                let instruction_result = String::from_utf8(instruction_bytes);

                let instruction = if let Ok(instruction) = instruction_result {
                    instruction
                } else {
                    return Err(String::from("Didn't receive valid utf8."));
                };

                return Ok(Async::Ready((self.instruction_type.clone(), instruction, stream)))
            },
            // Wait
            Async::NotReady => Ok(Async::NotReady)
        }
    }
}
