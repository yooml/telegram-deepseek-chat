use std::sync::Arc;

use rig::{completion::Prompt, providers};
use teloxide::{prelude::*, RequestError};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	dotenv().ok();
	let client = providers::deepseek::Client::from_env();
	let agent = client.agent("deepseek-chat").preamble("You are a helpful assistant.").build();

	// Wrap `agent` in an Arc for shared ownership
	let agent = Arc::new(agent);

	pretty_env_logger::init();
	log::info!("Starting bot...");

	let bot = Bot::from_env();

	teloxide::repl(bot.clone(), {
		move |bot: Bot, msg: Message| {
			let agent = Arc::clone(&agent);
			async move {
				if let Some(text) = msg.text() {
					match agent.prompt(text).await {
						Ok(answer) => {
							bot.send_message(msg.chat.id, answer).await?;
						},
						Err(err) => {
							log::error!("Error occurred while prompting the agent: {}", err);
							bot.send_message(
								msg.chat.id,
								"An error occurred while processing your request.",
							)
							.await?;
						},
					}
				} else {
					bot.send_message(msg.chat.id, "Send me plain text.").await?;
				}

				Ok(())
			}
		}
	})
	.await;

	Ok(())
}
