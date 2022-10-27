use std::fs::File;
use std::io::{Write, BufWriter};
use serde::{Serialize, Deserialize};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Serialize, Deserialize)]
pub struct Command {
    msg: String,
}

#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum CommandID {
    Command
}
pub trait GetCommandID<'a> : Serialize {
    fn get_command_id(&self) -> CommandID;
}
impl<'a> GetCommandID<'a> for Command {
    fn get_command_id(&self) -> CommandID {
        CommandID::Command
    }
}

pub fn serialize<'a, T: GetCommandID<'a>>(command: &T) -> Vec<u8> {
    let mut data = bincode::serialize(&command).unwrap().to_vec();
    let num: u16 = command.get_command_id().into();
    data.extend(num.to_be_bytes().into_iter());
    data
}

#[cfg(feature = "server")]
pub trait ServerCommand<'a>: Deserialize<'a> {
    fn execute(self, server: &mut Server);
}

#[cfg(feature = "server")]
pub struct Server {
}

#[cfg(feature = "server")]
impl<'a> ServerCommand<'a> for Command {
    fn execute(self, _: &mut Server) {
        println!("Message: {}", self.msg);
    }
}

#[cfg(feature = "server")]
fn drun<'a, T: ServerCommand<'a>>(data: &'a [u8], context: &mut Server) -> Result<(), bincode::Error> {
    let deserialized: T = bincode::deserialize::<'a>(data)?;
    T::execute(deserialized, context);
    Ok(())
}

#[cfg(feature = "server")]
pub fn deserialize_execute(data: Vec<u8>, server: &mut Server) {
    if data.len() >= 2 {
        let slice = [data[data.len() - 2], data[data.len() - 1]];
        let num: u16 = u16::from_be_bytes(slice);
        match CommandID::try_from(num) {
            Ok(id) => match id {
                CommandID::Command => drun::<Command>(&data, server).unwrap(),
            },
            Err(err) => panic!("Error converting: {}", err),
        }
    }
}

fn main() {
    if cfg!(feature = "server") {
        #[cfg(feature = "server")] {
            use std::io::Read;
            let mut server = Server {};
            let file = File::open("file.bin").unwrap();
            let mut buffered = std::io::BufReader::new(file);
            let mut data = vec![];
            buffered.read_to_end(&mut data).unwrap();
            deserialize_execute(data, &mut server);
        }
    } else {
        let command = Command {
            msg: "hello world".to_string(),
        };
        let file = File::create("file.bin").unwrap();
        let mut buffered = BufWriter::new(file);
        let data = serialize(&command);
        buffered.write_all(&data).unwrap();
    }
}
