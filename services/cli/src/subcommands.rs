pub enum Command {
    Init,
    Analyze,
}

impl ToString for Command {
    fn to_string(&self) -> String {
        match self {
            Command::Init => String::from("init"),
            Command::Analyze => String::from("analyse"),
        }
    }
}

pub struct CommandDescriptions {}

// Get Command Descriptions
impl CommandDescriptions {
    pub fn from(command: Command) -> String {
        match command {
            Command::Init => {
                String::from("Initialize a new git ventures project for a local git repository")
            }
            Command::Analyze => String::from("Analyze a repository or individual commit"),
        }
    }
}
