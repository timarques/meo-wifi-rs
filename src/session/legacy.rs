use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyIvInit};
use base64::prelude::{BASE64_STANDARD as base64, Engine};
use std::net::UdpSocket;
use lazy_regex::regex;
use super::error::Error;

#[derive(Debug, Clone)]
pub struct Legacy {
    username: String,
    password: String,
}

impl Legacy {

    fn validate_credentials(username: &str, password: &str) -> Result<(), Error> {

        let email_regex = regex!(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$");
        if !email_regex.is_match(username) {
            return Err(Error::InvalidEmail);
        }
    
        let has_lowercase = || password.chars().any(|c| c.is_ascii_lowercase());
        let has_uppercase = || password.chars().any(|c| c.is_ascii_uppercase());
        let has_number = || password.chars().any(|c| c.is_ascii_digit());
        if password.len() < 8 || !has_lowercase() || !has_uppercase() || !has_number() {
            return Err(Error::InvalidPassword);
        }
    
        Ok(())
    }

    pub fn new(username: &str, password: &str) -> Result<Self, Error> {
        Self::validate_credentials(username, password)?;
        Ok(Self { 
            username: username.to_string(), 
            password: password.to_string(),
        })
    }

    pub fn get_local_ip() -> Option<String> {
        let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
        socket.connect("8.8.8.8:80").ok()?;
        let local_addr = socket.local_addr().ok()?;
        Some(local_addr.ip().to_string())
    }

    fn encrypt_password(&self) -> String {
        const SALT: [u8; 19] = [
            0x77, 0x23, 0x24, 0x69, 0x66, 0x69, 0x31, 0x32, 0x34, 0x29,
            0x39, 0x6D, 0x65, 0x6F, 0x39, 0x38, 0x57, 0x49, 0x46
        ];

        const IV: [u8; 16] = [
            0x72, 0xc4, 0x72, 0x1a, 0xe0, 0x1a, 0xe0, 0xe8,
            0xe8, 0x4b, 0xd6, 0x4a, 0xd6, 0x60, 0x60, 0xc4
        ];
        let ip = Self::get_local_ip().expect("Failed to get local IP address");
        let key = pbkdf2::pbkdf2_hmac_array::<sha1::Sha1, 32>(ip.as_bytes(), &SALT, 100);

        let mut buffer = [0u8; 48];
        let cipher = cbc::Encryptor::<aes::Aes256>::new(&key.into(),&IV.into());
        let ciphertext = cipher
            .encrypt_padded_b2b_mut::<Pkcs7>(self.password.as_bytes(), &mut buffer)
            .expect("Failed to encrypt password");

        base64.encode(&ciphertext)
    }

    fn send_request(&self, url: &str) -> Result<ureq::Response, Error> {
        const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64)";
        const CONTENT_TYPE: &str = "application/x-www-form-urlencoded";
        let res = ureq::get(url)
            .set("Content-Type", CONTENT_TYPE)
            .set("User-Agent", USER_AGENT)
            .timeout(std::time::Duration::from_secs(2))
            .send_form(&[])?;
        Ok(res)
    }

}

impl super::Trait for Legacy {

    fn is_logged(&self) -> bool {
        const URL: &str = "https://servicoswifi.apps.meo.pt/HotspotConnection.svc/GetState?mobile=false";
        self
            .send_request(URL)
            .ok()
            .and_then(|r| {
                r
                    .into_json::<serde_json::Value>()
                    .ok()
                    .and_then(|v| {
                        println!("{:?}", v);
                        v.get("LoggedOn").and_then(|v| v.as_bool())
                    })
            }).unwrap_or(false)
    }

    fn login(&self) -> Result<(), Error> {
        let url = format!(
            "https://servicoswifi.apps.meo.pt/HotspotConnection.svc/Login?username={}&password={}&navigatorLang=en&callback=", 
            urlencoding::encode(&self.username), 
            urlencoding::encode(&self.encrypt_password())
        );

        let json: serde_json::Value = self
            .send_request(&url)
            .and_then(|r| r.into_json().map_err(Error::from))?;

        let is_successful = json
            .get("result")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if is_successful {
            return Ok(())
        }

        let error = json
            .get("error")
            .and_then(|e| e.as_str())
            .map(|e| e.to_lowercase())
            .map(|e| {
                if e.contains("out of reach") {
                    Error::NetworkUnreachable
                } else if e.contains("invalid credentials") {
                    Error::CredentialsMismatch
                } else if e.contains("frammedip") {
                    Error::InvalidIp
                } else if e.contains("already logged") {
                    Error::AlreadyLoggedIn
                } else {
                    Error::Custom(e)
                }
            }).unwrap_or(Error::from("Unknown error occurred"));

        Err(error)
    }

    fn logout(&self) -> Result<(), Error> {
        const URL: &str = "https://servicoswifi.apps.meo.pt/HotspotConnection.svc/Logoff?callback=";
        let result = self
            .send_request(URL)
            .and_then(|r| r.into_string().map_err(Error::from))
            .map(|r| r.contains("true"));

        match result {
            Ok(true) => Ok(()),
            Ok(false) => Err(Error::from("failed to logout")),
            Err(e) => Err(Error::from(e))
        }
    }

}