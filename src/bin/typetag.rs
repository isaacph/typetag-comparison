use uuid::Uuid;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error, Read, BufWriter};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Command {
    msg: String,
}

#[typetag::serde(tag = "type")]
pub trait ServerCommand {
    #[cfg(feature = "server")]
    fn execute(&self, server: &mut Server);
}

#[cfg(feature = "server")]
pub struct Server {
}

#[typetag::serde]
impl ServerCommand for Command {
    #[cfg(feature = "server")]
    fn execute(&self, _: &mut Server) {
        println!("Message: {}", self.msg);
    }
}

fn main() {
    if cfg!(feature = "server") {
        #[cfg(feature = "server")] {
            let mut server = Server {};
            let file = File::open("file.bin").unwrap();
            let mut buffered = BufReader::new(file);
            let mut data = vec![];
            buffered.read_to_end(&mut data).unwrap();
            let command: Box<dyn ServerCommand> = bincode::deserialize(&data).unwrap();
            command.execute(&mut server);
        }
    } else {
        let command = Command {
            msg: "hello world".to_string(),
        };
        let poly: Box<dyn ServerCommand> = Box::new(command);
        let file = File::create("file.bin").unwrap();
        let mut buffered = BufWriter::new(file);
        let data = bincode::serialize(&poly).unwrap();
        buffered.write_all(&data).unwrap();
    }
}
