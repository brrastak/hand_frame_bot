
use handframe::hand_frame::{*, self};
use rand::Rng;
use std::error::Error;
use teloxide::{
    prelude::*,
    types::Me,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let bot = Bot::from_env();

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler));

    Dispatcher::builder(bot, handler).enable_ctrlc_handler().build().dispatch().await;
    Ok(())
}


async fn message_handler(
    bot: Bot,
    msg: Message,
    _me: Me,
) -> Result<(), Box<dyn Error + Send + Sync>> {

    if let Some(text) = msg.text() {

        if text.chars().count() > 2 {

            return Ok(())
        }

        let style: FrameStyle;
        let layers: usize;
        {
            // rng is not compatible with await
            let mut rng = rand::thread_rng();
            style = if rng.gen() {
                FrameStyle::LightCenter
            }
            else {
                FrameStyle::DarkCenter
            };
            layers = rng.gen_range(1..=hand_frame::max_layers_number());
        }

        let frame = hand_frame::new(text, layers, style).unwrap();
        let mut answer = String::new();

        for line in &frame {

            answer.push_str(line);
            answer.push_str("\n");
        }
        
        bot.send_message(msg.chat.id, answer).await?;
    }

    Ok(())
}
