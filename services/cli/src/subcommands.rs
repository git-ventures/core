pub enum Command {
    Init,
}

impl ToString for Command {
    fn to_string(&self) -> String {
        match self {
            Command::Init => String::from("init"),
        }
    }
}

pub struct CommandDescriptions {}

// Get Command Descriptions
impl CommandDescriptions {
    pub fn from(command: Command) -> String {
        match command {
            Command::Init => {
                String::from("Initialize a new git ventures project for local git repository")
            }
        }
    }
}
