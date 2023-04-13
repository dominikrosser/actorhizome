use async_trait::async_trait;
use tokio::task;
//use std::error::Error;
use anyhow::Error;
use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};

use crate::Message;
use crate::Actor;

pub async fn call_gpt(model: String, msg: Message) -> Result<Message, Error> {
    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        //.model("gpt-3.5-turbo")
        .model(model)
        .messages([
                  ChatCompletionRequestMessageArgs::default()
                  .role(Role::System)
                  .content("You are a helpful assistant answering solely in a bash executable line. You provide the user with one line executable in bash terminal.")
                  .build()?,
                  ChatCompletionRequestMessageArgs::default()
                  .role(Role::User)
                  .content("CODE ONLY! NO EXPLANATIONS! NO TEXT!")
                  .build()?,
                  ChatCompletionRequestMessageArgs::default()
                  .role(Role::Assistant)
                  //.content("The Los Angeles Dodgers won the World Series in 2020.")
                  .content("The Anwser must be exactly one line and correspond to a bash executable command.")
                  .build()?,
                  ChatCompletionRequestMessageArgs::default()
                  .role(Role::User)
                  //.content("Where was it played?")
                  .content(msg)
                  .build()?,
        ])
        .build()?;

    let response = client.chat().create(request).await?;

    println!("\nResponse:\n");
    for choice in response.choices {
        /*println!(*/
            /*"{}: Role: {}  Content: {}",*/
            /*choice.index, choice.message.role, choice.message.content*/
        /*    );*/
        let result_message  = format!("{}", choice.message.content);
        return Ok(result_message);
    }

    Ok("No  response".to_string())
}

pub struct GPTActor {
    model: String,
}

#[async_trait]
impl Actor for GPTActor {
    fn new() -> Self {
        Self {
            model: "gpt-3.5-turbo".to_string(),
            //model: "gpt-4".to_string(),
        }
    }

    async fn receive(&self, msg: Message) -> Option<Message> {
        println!("Received: {}", msg);

        let msg = msg.clone();
        let model = self.model.clone();
        match task::spawn(call_gpt(model, msg)).await {
            Ok(result) => match result {
                Ok(message) => Some(message),
                Err(e) => {
                    println!("Error: {:?}", e);
                    None
                }
            },
            Err(e) => {
                println!("Error: {:?}", e);
                None
            }
        }
    }
}
