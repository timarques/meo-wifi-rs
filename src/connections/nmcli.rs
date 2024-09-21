use std::process::Command;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use super::error::Error;
use crate::log;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Type {
    Loopback,
    Wifi,
    Ethernet,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Connection {
    name: String,
    r#type: Type,
    active: bool
}

#[derive(Debug, Clone)]
pub struct Nmcli {}

impl Nmcli {

    pub const fn new() -> Self {
        Self {}
    }

    fn execute(args: &[&str]) -> Result<String, IoError> {
        let output = Command::new("nmcli")
            .args(args)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(IoError::new(IoErrorKind::Other, format!("nmcli command failed: {}", stderr)));
        }

        String::from_utf8(output.stdout)
            .map(|s| s.trim().to_string())
            .map_err(|e| IoError::new(IoErrorKind::InvalidData, e))
    }

    fn parse_network_type(type_str: &str) -> Type {
        if type_str.contains("ethernet") {
            Type::Ethernet
        } else if type_str.contains("wireless") {
            Type::Wifi
        } else if type_str.contains("loopback") {
            Type::Loopback
        } else {
            Type::Unknown
        }
    }

    fn connections(&self) -> Result<Vec<Connection>, IoError> {
        let output = Self::execute(&["-t", "-f", "NAME,TYPE,ACTIVE,DEVICE", "connection", "show"])?;
        let connections: Vec<_> = output.lines().filter_map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() < 3 {
                log::warn(&format!("Skipping invalid connection line: {}", line));
                None
            } else {
                Some(Connection {
                    name: parts[0].to_string(),
                    r#type: Self::parse_network_type(parts[1]),
                    active: parts[2] == "yes"
                })
            }
        }).collect();
        Ok(connections)
    }

}

impl super::Trait for Nmcli {

    fn active(&self) -> Option<String> {
        self
            .connections()
            .ok()
            .and_then(|c| {
                let mut iter = c
                    .into_iter()
                    .filter(|conn| conn.active);
                iter
                    .find(|conn| conn.r#type == Type::Ethernet)
                    .or_else(|| iter.find(|conn| conn.r#type == Type::Wifi))
                    .map(|conn| conn.name.clone())
        })
    }

    fn is_connected(&self, name: &str) -> bool {
        let name = name.to_lowercase();
        self
            .connections()
            .ok()
            .and_then(|c| {
                c
                    .into_iter()
                    .find(|conn| conn.name.to_lowercase().eq(&name))
                    .map(|conn| conn.active)
            })
            .unwrap_or(false)
    }

    fn connect(&self, connection: &str) -> Result<(), Error> {
        let connections = self.connections()?;
        let connection_name = connection.to_lowercase();
        let mut connection = None;
        for current_connection in &connections {
            if current_connection.name.to_lowercase() == connection_name {
                if current_connection.active {
                    return Err(Error::AlreadyActive)
                }
                connection = Some(current_connection);
            } else if current_connection.active {
                Self::execute(&["connection", "down", &current_connection.name])?;
            }
        }

        let connection = connection.ok_or(Error::Unavailable)?;
        if connection.r#type == Type::Wifi {
            Self::execute(&["radio", "wifi", "on"])?;
        }
        Self::execute(&["connection", "up", &connection.name])?;

        Ok(())
    }

    fn disconnect(&self, connection: &str) -> Result<(), Error> {
        let connection_name = connection.to_lowercase();
        let connections = self.connections()?;
        let connection = connections
            .iter()
            .find(|conn| conn.name.to_lowercase().eq(&connection_name))
            .ok_or(Error::Unavailable)?;
        Self::execute(&["connection", "down", &connection.name])
            .map(|_| ())
            .map_err(|e| Error::from(e))
    }

}
