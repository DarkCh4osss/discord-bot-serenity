use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args,
};

#[command]
#[required_permissions("KICK_MEMBERS")]
#[num_args(2)]
#[usage = "membro motivo"]
pub async fn kick(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    let member = args.single::<id::UserId>();
    match member {
        Err(_) => {
            msg.channel_id
                .say(&ctx, "Impossibile trovare il membro ")
                .await.unwrap();
        }
        Ok(mem) => match args.single::<String>() {
            Err(_) => {
                msg.channel_id
                    .say(&ctx, "Bisogna fornire una motivazione ")
                    .await.unwrap();
            }
            Ok(res) => match msg.guild_id {
                None => {
                    msg.channel_id
                        .say(&ctx, "Il membro necessita di essere nel server")
                        .await.unwrap();
                }
                Some(g) => {
                    let member = g.member(&ctx, mem).await.unwrap();
                    member.kick_with_reason(&ctx, &res).await.unwrap();
                    let sos = msg.channel_id.send_message(&ctx, |m| {
                        m.embed(|e| e
                            .title("Utente Espulso | TS-Bot")
                            .field("Membro Espulso", &member.user, false)
                            .field("Motivo", &res, false)
                            .thumbnail(member.user.avatar_url().unwrap())
                            .color(0xff0000)
                            .timestamp("")
                            .footer(|f| f
                                .text("Membro Espulso | TS-Bot")
                            )
                        );
            
                        m
                    }).await;

                    if let Err(why) = sos {
                        println!("Errore durante l'invio dell'embed: {:?}", why);
                    }
                }
            },
        },
    };

    Ok(())
}

#[command]
#[required_permissions("BAN_MEMBERS")]
#[num_args(2)]
#[usage = "membro motivo"]
pub async fn ban(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    let member = args.single::<id::UserId>();
    match member {
        Err(_) => {
            msg.channel_id
                .say(&ctx, "Impossibile trovare il membro ")
                .await.unwrap();
        }
        Ok(mem) => match args.single::<String>() {
            Err(_) => {
                msg.channel_id
                    .say(&ctx, "Bisogna fornire una motivazione ")
                    .await.unwrap();
            }
            Ok(res) => match msg.guild_id {
                None => {
                    msg.channel_id
                        .say(&ctx, "Il membro necessita di essere nel server")
                        .await.unwrap();
                }
                Some(g) => {
                    let member = g.member(&ctx, mem).await.unwrap();
                    member.ban_with_reason(&ctx, 0, &res).await.unwrap();
                    let sos = msg.channel_id.send_message(&ctx, |m| {
                        m.embed(|e| e
                            .title("Utente Bannato | TS-Bot")
                            .field("Membro Bannato", &member.user, false)
                            .field("Motivo", &res, false)
                            .thumbnail(member.user.avatar_url().unwrap())
                            .color(0xff0000)
                            .timestamp("")
                            .footer(|f| f
                                .text("Membro Bannato | TS-Bot")
                            )
                        );
            
                        m
                    }).await;

                    if let Err(why) = sos {
                        println!("Errore durante l'invio dell'embed: {:?}", why);
                    }
                }
            },
        },
    };

    Ok(())
}

#[command]
#[required_permissions("BAN_MEMBERS")]
#[num_args(1)]
#[usage = "membro"]
async fn unban(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let member = args.single::<id::UserId>();
    match member {
        Err(_) => {
            msg.channel_id
                .say(&ctx, "Impossibile trovare il membro ")
                .await
                .unwrap();
        }
        Ok(mem) => {
            let mut banned = false;
            match msg.guild_id {
                None => {
                    msg.channel_id
                        .say(&ctx, "L'utente deve essere nel server")
                        .await
                        .unwrap();
                }
                Some(g) => {
                    let us = mem.to_user(&ctx).await.unwrap();
                    let _bad = g
                        .bans(&ctx)
                        .await
                        .unwrap()
                        .iter()
                        .map(|ban| {
                            if ban.user.id == us.clone().id {
                                banned = true;
                            }
                            true
                        })
                        .collect::<Vec<bool>>();
                    if banned {
                        g.unban(&ctx, us.id).await.unwrap();
                        let sos = msg.channel_id.send_message(&ctx, |m| {
                            m.embed(|e| e
                                .title("Utente Sbannato | TS-Bot")
                                .field("Membro Sbannato", &us, false)
                                .thumbnail(us.avatar_url().unwrap())
                                .color(0xff0000)
                                .timestamp("")
                                .footer(|f| f
                                    .text("Membro Sbannato | TS-Bot")
                                )
                            );
                
                            m
                        }).await;

                        if let Err(why) = sos {
                            println!("Errore durante l'invio dell'embed: {:?}", why);
                        }
                    } else {
                        msg.channel_id
                            .say(&ctx, format!("{} non era bannato. Non è stato sbannato", us.name))
                            .await
                            .unwrap();
                    }
                }
            }
        }
    };
    Ok(())
}