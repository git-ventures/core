pub enum Command {
    Init,
    Analyse,
}

impl ToString for Command {
    fn to_string(&self) -> String {
        match self {
            Command::Init => String::from("init"),
            Command::Analyse => String::from("analyse"),
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
            Command::Analyse => String::from("Analyse a repository or individual commit"),
        }
    }
}
