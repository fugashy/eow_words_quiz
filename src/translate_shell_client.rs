use std::error::Error;
extern crate subprocess;
use subprocess::{Popen, PopenConfig, Redirection};

pub struct TranslateShellClient {
    client: Popen,
}

impl TranslateShellClient {
    pub fn new() -> Result<TranslateShellClient, Box<dyn Error>> {
        let client = Popen::create(
            &["trans", "{en=ja}"],
            PopenConfig {
                stdin: Redirection::Pipe,
                stdout: Redirection::Pipe,
                ..Default::default()
            },
        )?;
        Ok(TranslateShellClient { client })
    }

    pub fn trans(
        &mut self,
        statement: &str,
    ) -> Result<(Option<String>, Option<String>), &'static str> {
        let s = format!("'{}'", statement.to_string());
        match self.client.communicate(Some(&s)) {
            Ok(v) => Ok(v),
            Err(_) => Err("Some thing goes wrong"),
        }
    }
}
