use serde::{Deserialize, Serialize};
use sha256::digest;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    id: String,
    creation_date: String,
    name: String,
    login: Login,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    username: String,
    password: String,
    uri: String,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Informations for {}:\nUsername: {}\nPassword: {}\nURI: {}",
            self.name,
            self.login.username,
            get_hidden_password(decrypt(self.login.password.as_str()).as_str()),
            self.login.uri
        )
    }
}
impl Item {
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

pub fn hash(password: &str) -> String {
    digest(String::from(password))
}

pub fn decrypt(password: &str) -> String {
    String::from(password)
}

pub fn get_hidden_password(password: &str) -> String {
    password.chars().map(|_| '*').collect::<String>()
}

pub enum PasswordType {
    PassWord,
    PassPhrase,
}

pub struct GeneratorConfig {
    password_type: PasswordType,
    length: u8,
    has_uppercase: bool,
    has_lowercase: bool,
    has_number: bool,
    has_special_char: bool,
}

fn generate_password(config: &GeneratorConfig) -> String {
    String::new()
}

#[derive(PartialEq, Debug)]
pub enum PasswordStrength {
    Weak,
    Good,
    Strong,
}

fn get_password_strength(password: &str) -> PasswordStrength {
    PasswordStrength::Weak
}

#[cfg(test)]
mod tests {
    use crate::*;
    use regex::Regex;

    #[test]
    fn test_hash() {
        assert_eq!(
            hash("hello"),
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_decrypt() {
        assert_eq!(true, false);
    }

    #[test]
    fn test_generate() {
        let config = GeneratorConfig {
            password_type: PasswordType::PassWord,
            length: 8,
            has_uppercase: true,
            has_lowercase: true,
            has_number: true,
            has_special_char: true,
        };
        let generated = generate_password(&config);
        assert_eq!(generated.len(), 8);

        let re = Regex::new(r"[A-Z]").unwrap();
        assert!(re.is_match(generated.as_str()));

        let re = Regex::new(r"[a-z]").unwrap();
        assert!(re.is_match(generated.as_str()));

        let re = Regex::new(r"[0-9]").unwrap();
        assert!(re.is_match(generated.as_str()));

        let re = Regex::new(r"[!@#$%^＆+]").unwrap();
        assert!(re.is_match(generated.as_str()));
    }

    #[test]
    fn test_strength() {
        let password = "azerty";
        assert_eq!(get_password_strength(password), PasswordStrength::Weak);

        let password = "azertyazertyazertyazerty";
        assert_eq!(get_password_strength(password), PasswordStrength::Weak);

        let password = "&G3MkwMhx23J#";
        assert_eq!(get_password_strength(password), PasswordStrength::Strong);
    }

    #[test]
    fn test_hidden() {
        let password = "azerty";
        assert_eq!(get_hidden_password(password), "******");
        let password = "1234";
        assert_eq!(get_hidden_password(password), "****");
    }
}
