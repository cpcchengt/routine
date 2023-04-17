use dotenvy::dotenv;
use std::sync::Arc;
use teloxide::{
    dispatching::dialogue::InMemStorage,
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup, BotCommand},
};

#[derive(Debug, Clone)]
pub struct Statex {
    pub count: i64
}
#[derive(Debug, Clone)]
pub struct Statey {
    pub count: i64
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let bot = Bot::from_env();

    bot.set_my_commands(vec![BotCommand{
        command: "xxxxxxxxxx".to_string(),
        description: "xxx".to_string()
    }]).await.unwrap();

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(answer))
        .branch(Update::filter_callback_query().endpoint(answer_callback));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![Statex {count:1}, Statey {count:-1}])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn answer(bot: Bot, statex: Statex, statey: Statey, msg: Message) -> ResponseResult<()> {
    println!("{:?}", msg);
    println!("State: {:?}", statex);
    println!("State: {:?}", statey);
    // state.count += 1;
    let markup = ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup {
        inline_keyboard: vec![vec![InlineKeyboardButton::callback("test", "test")]],
    });
    bot.send_message(msg.chat.id, "test".to_string())
        .reply_markup(markup)
        .await
        .unwrap();
    Ok(())
}

async fn answer_callback(bot: Bot, msg: CallbackQuery) -> ResponseResult<()> {
    println!("{:?}", msg);
    // let markup = ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup {
    //     inline_keyboard: vec![vec![InlineKeyboardButton::callback("test", "test")]]
    // });
    // bot.send_message(msg.chat.id, "test".to_string()).reply_markup(markup).await.unwrap();
    Ok(())
}
