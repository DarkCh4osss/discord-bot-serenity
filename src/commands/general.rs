use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
        println!("Errore durante l'invio del messaggio: {:?}", why);
    }

    Ok(())
}

#[command]
pub async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let sos = msg.channel_id.send_message(&ctx, |m| {
        m.embed(|e| e
            .title("Comandi | TS-Bot")
            .field("Generale", "`drk!help`\n`drk!ping`", false)
            .field("Moderazione", "`drk!ban <utente> <motivo>`\n`drk!kick <utente> <motivo>`\n`drk!unban <id>`", false)
            .color(0xff0000)
            .timestamp("")
            .footer(|f| f
                .text("Comandi | TS-Bot")
            )
        );

        m
    }).await;

    if let Err(why) = sos {
        println!("Errore durante l'invio dell'embed: {:?}", why);
    }

    Ok(())
}