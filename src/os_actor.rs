use std::fs;
use std::process::{Command, Output};
use std::str;
use tokio::task;
use async_trait::async_trait;

use crate::Actor;
use crate::Message;

pub struct OSActor {
    cwd: String,
}
#[async_trait]
impl Actor for OSActor {
    fn new() -> Self {
        let cwd = "./actor_box_env/".to_string();
        match fs::create_dir_all(&cwd) {
            Ok(_) => (),
            Err(e) => println!("Error creating directory: {}", e),
        }
        OSActor { cwd }
    }

    async fn receive(&self, msg: Message) -> Option<Message> {
        println!("Received: {}", msg);
        let output = self.execute_command(&msg).await;
        match output {
            Ok(output) => {
                let output_str = str::from_utf8(&output.stdout).unwrap_or("");
                Some(output_str.to_string())
            }
            Err(e) => {
                println!("Error: {}", e);
                None
            }
        }
    }
}

impl OSActor {
    async fn execute_command(&self, cmd: &str) -> std::io::Result<Output> {
        let cwd = self.cwd.clone();
        let cmd = cmd.to_string().clone();
        task::spawn_blocking(move || {
            let mut parts = cmd.split_whitespace();
            let command = parts.next().unwrap_or("");
            let args = parts.collect::<Vec<&str>>();
            Command::new(command)
                .args(args)
                .current_dir(&cwd)
                .output()
        })
        .await
        .unwrap()
    }
}
