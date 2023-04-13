use async_trait::async_trait;

use dotenv::dotenv;


use tokio::io::{stdin, BufReader, AsyncBufReadExt};

mod os_actor;
mod gpt_actor;

use crate::os_actor::OSActor;
use crate::gpt_actor::GPTActor;

type Message = String;

#[async_trait]
trait Actor {
    fn new() -> Self;
    async fn receive(&self, msg: Message) -> Option<Message>;
}

struct TerminalUserActor  {}

#[async_trait]
impl Actor for TerminalUserActor {
    fn new() -> Self {
        Self {}
    }

    async fn receive(&self, msg: Message) -> Option<Message> {
        println!("User: {}", msg);
        let mut input = String::new();
        let mut stdin = BufReader::new(stdin());
        let _ = stdin.read_line(&mut input).await;
        let input = input.trim();
        Some(input.to_string())
    }
}

use async_recursion::async_recursion;

#[async_recursion]
async fn querky_user_gpt_os_loop_step() -> () {
    let terminal_user_actor = TerminalUserActor::new();
    match terminal_user_actor.receive("What do you want to do in the folder?".to_string()).await {
        Some(user_input) => {
            println!("User Input {}", user_input);

            let gpt_input_msg = user_input;
            let gpt_actor = GPTActor::new();
            match gpt_actor.receive(gpt_input_msg).await {
                Some(gpt_output) => {
                    println!("GPT Output: {}", gpt_output);
                    println!("OS Actor Executing command: {}", gpt_output);

                    let os_actor = OSActor::new();
                    match os_actor.receive(gpt_output).await {
                        Some(output) => {
                            println!("{}", output);
                            querky_user_gpt_os_loop_step().await;
                        },
                        None => println!("Error executing command"),
                    }
                },
                None => println!("Error calling GPT4"),
            }
        },
        None => println!("Error calling TerminalUserActor"),
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let os_actor = OSActor::new();

    match os_actor.receive("touch test.txt".to_string()).await {
        Some(output) => println!("{}", output),
        None => println!("Error executing command"),
    }

    match os_actor.receive("ls -la".to_string()).await {
        Some(output) => println!("{}", output),
        None => println!("Error executing command"),
    }

    querky_user_gpt_os_loop_step().await;
}

