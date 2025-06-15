use super::{generate_gore_slots, SlotMachine};
use crate::{Context, Error};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use serenity::builder::{CreateEmbed, CreateEmbedFooter};
use poise::CreateReply;
use poise::serenity_prelude as serenity;

pub static GORE_SLOT_MACHINE: Lazy<Mutex<SlotMachine>> =
    Lazy::new(|| Mutex::new(generate_gore_slots()));

#[poise::command(slash_command)]
pub async fn slots(ctx: Context<'_>) -> Result<(), Error> {
    println!("{} used the slots command", ctx.author().name);
    println!("author user id: {}", ctx.author().id);
    let play_result = {
        let mut slot_machine = GORE_SLOT_MACHINE.lock().unwrap();
        slot_machine.play()
    };
    let symbols: String = play_result.symbols.iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join("");
    let footer_message = if play_result.payout >= 500 {
        "🎉 Jackpot! 🎉".to_string()
    } else if play_result.payout > 0 {
        format!("You won! Maybe next time you'll hit the jackpot!\nCurrent Jackpot:{}", play_result.current_jackpot_value)
    } else {
        "Better luck next time!".to_string()
    };

    let embed = CreateEmbed::new()
        .color(0x5b9e48)
        .title("🎰 Slot Machine Results")
        .footer(CreateEmbedFooter::new(footer_message))
        .fields([
            ("Spin Result", symbols, false),
            ("Payout", play_result.payout.to_string(), true)
        ]);


    ctx.send(CreateReply {
        embeds: vec![embed],
        ..Default::default()
    })
    .await?;

    Ok(())
}
