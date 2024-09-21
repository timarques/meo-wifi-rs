use std::env;

#[derive(Debug)]
pub enum Output {
    Args(Args),
    Info(String),
}

impl Output {
    pub fn unwrap(self) -> (Option<Args>, Option<String>) {
        match self {
            Output::Args(args) => (Some(args), None),
            Output::Info(info) => (None, Some(info))
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Mode {
    OneShot,
    Continuous
}

#[derive(Debug)]
pub struct Args {
    username: String,
    password: String,
    mode: Mode
}

impl Args {

    #[allow(dead_code)]
    pub fn user(&self) -> &str {
        &self.username
    }

    #[allow(dead_code)]
    pub fn username(&self) -> &str {
        &self.username
    }

    #[allow(dead_code)]
    pub fn pass(&self) -> &str {
        &self.password
    }

    #[allow(dead_code)]
    pub fn password(&self) -> &str {
        &self.password
    }

    #[allow(dead_code)]
    pub fn is_continuous(&self) -> bool {
        self.mode == Mode::Continuous
    }

    #[allow(dead_code)]
    pub fn is_one_shot(&self) -> bool {
        self.mode == Mode::OneShot
    }

}

fn usage_instructions() -> String {
    format!(
        "{}{}{}{}{}{}{}",
        "Usage: program -u <username> -p <password> [-c]\n\n",
        "Options:\n",
        "\t-u, --username    Specify the username\n",
        "\t-p, --password    Specify the password\n",
        "\t-c, --continuous  Run in continuous mode (default is one-shot)\n",
        "\t-h, --help        Display this help message\n",
        "\t-v, --version     Display the version number"
    )
}

pub fn new() -> Result<Output, String> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut username = None;
    let mut password = None;
    let mut connection_mode = Mode::OneShot;

    let mut args_iter = args.iter().peekable();
    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "-u" | "--username" => {
                username = args_iter.next().map(|s| s.to_string());
            }
            "-p" | "--password" => {
                password = args_iter.next().map(|s| s.to_string());
            }
            "-c" | "--continuous" => {
                connection_mode = Mode::Continuous;
            }
            "-h" | "--help" => {
                return Ok(Output::Info(usage_instructions()));
            }
            "-v" | "--version" => {
                return Ok(Output::Info(format!("v{}", env!("CARGO_PKG_VERSION"))));
            }
            _ => return Err(format!("Unknown argument: {}", arg)),
        }
    }

    match (username, password) {
        (Some(u), Some(p)) => {
            Ok(Output::Args(Args { 
                username: u, 
                password: p, 
                mode: connection_mode 
            }))
        },
        (None, Some(_)) => Err("Missing username".to_string()),
        (Some(_), None) => Err("Missing password".to_string()),
        (None, None) => Err("Missing username and password".to_string()),
    }
}