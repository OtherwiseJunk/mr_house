use super::{generate_gore_slots, PlayResult, SlotMachine};
use crate::services::libcoin::{
    deduct_libcoin, get_libcoin_balance, get_user_transactions, grant_libcoin,
};
use crate::{Context, Error, PREVIOUS_ROLLING_JACKPOT};
use once_cell::sync::Lazy;
use poise::serenity_prelude as serenity;
use poise::CreateReply;
use serenity::builder::{CreateEmbed, CreateEmbedFooter};
use std::sync::Mutex;

pub static GORE_SLOT_MACHINE: Lazy<Mutex<SlotMachine>> =
    Lazy::new(|| Mutex::new(generate_gore_slots(*PREVIOUS_ROLLING_JACKPOT)));

const MR_HOUSE_ID: u64 = 1382600478206066769;
const DEDUCT_MESSAGE: &str = "Playing the slot machine";
const GRANT_MESSAGE: &str = "Winning from the slot machine";

#[poise::command(
    slash_command,
    description_localized("en-US", "Spin the slot machine for a chance to win Libcoin! Costs 10 Libcoin per spin."),
    description_localized("fr", "Faites tourner la machine à sous pour tenter de gagner des Libcoins! Coût : 10 Libcoins par tour."),
    description_localized("es-ES", "Gira la ruleta para ganar Libcoin! 10 Libcoin por giro!")
)]
pub async fn slots(ctx: Context<'_>) -> Result<(), Error> {
    let user_id = ctx.author().id.get();
    let play_cost = {
        let slot_machine = GORE_SLOT_MACHINE.lock().unwrap();
        slot_machine.cost_per_play
    };

    if get_libcoin_balance(user_id).await? < play_cost as f64 {
        return Err(Error::from(
            "You don't have enough libcoin to play the slot machine!",
        ));
    }

    deduct_libcoin(user_id, play_cost as f64, DEDUCT_MESSAGE)
        .await
        .map_err(|_| Error::from("Sorry, looks like I'm having trouble contacting the bank."))?;
    grant_libcoin(
        MR_HOUSE_ID,
        play_cost as f64,
        &format!(
            "Payment from {} playing the slot machine",
            ctx.author().name
        ),
    )
    .await
    .map_err(|_| Error::from("Sorry, looks like I'm having trouble contacting the bank."))?;

    let play_result = {
        let mut slot_machine = GORE_SLOT_MACHINE.lock().unwrap();
        slot_machine.play()
    };

    let embed = build_result_embed(&play_result);

    ctx.send(CreateReply {
        embeds: vec![embed],
        ..Default::default()
    })
    .await?;

    grant_libcoin(user_id, play_result.payout as f64, GRANT_MESSAGE).await
        .map_err(|_| Error::from("Well this is embarassing. I wanted to give you your winnings but it looks like I'm having trouble contacting the bank."))?;

    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "View the slot machine's paytable."),
    description_localized("fr", "Consultez la table des gains de la machine à sous."),
    description_localized("es-ES", "Mira la lista de pagos de la ruleta.")
)]
pub async fn paytable(ctx: Context<'_>) -> Result<(), Error> {
    let embed = {
        let slot_machine = GORE_SLOT_MACHINE.lock().unwrap();
        slot_machine.get_pay_table_embed()
    };

    ctx.send(CreateReply {
        embeds: vec![embed],
        ..Default::default()
    })
    .await?;

    Ok(())
}

#[poise::command(
    slash_command,
    description_localized(
        "en-US",
        "View how much Libcoin you've spent and won playing the slot machine."
    ),
    description_localized(
        "fr",
        "Voyez combien de Libcoin vous avez dépensés et gagnés en jouant à la machine à sous."
    ),
    description_localized(
        "es-ES",
        "Revisa cuantos Libcoins has invertido y cuantos has ganado en la ruleta."
    )
)]
pub async fn stats(ctx: Context<'_>) -> Result<(), Error> {
    let transactions = get_user_transactions(ctx.author().id.get())
        .await
        .map_err(|_| Error::from("Sorry, looks like I'm having trouble contacting the bank."))?;

    if transactions.is_empty() {
        ctx.send(CreateReply {
            content: Some("You haven't played the slot machine yet.".to_string()),
            ..Default::default()
        })
        .await?;
        return Ok(());
    }

    let total_spent: f64 = transactions
        .iter()
        .filter(|t| t.transaction_message == DEDUCT_MESSAGE)
        .map(|t| t.amount)
        .sum();
    let total_won: f64 = transactions
        .iter()
        .filter(|t| t.transaction_message == GRANT_MESSAGE)
        .map(|t| t.amount)
        .sum();
    let net_gain = total_won - total_spent;
    let embed = CreateEmbed::new()
        .color(0x5b9e48)
        .title("🎰 Slot Machine Stats")
        .fields([
            ("Total Spent", format!("{:.2} libcoin", total_spent), true),
            ("Total Won", format!("{:.2} libcoin", total_won), true),
            ("Net Gain/Loss", format!("{:.2} libcoin", net_gain), true),
        ]);

    ctx.send(CreateReply {
        embeds: vec![embed],
        ..Default::default()
    })
    .await?;
    Ok(())
}

fn build_result_embed(play_result: &PlayResult) -> CreateEmbed {
    let symbols: String = play_result
        .symbols
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join("");
    let footer_message = if play_result.payout >= 500 {
        "🎉 Jackpot! 🎉".to_string()
    } else if play_result.payout > 0 {
        format!(
            "You won! Maybe next time you'll hit the jackpot!\nCurrent Jackpot:{}",
            play_result.current_jackpot_value
        )
    } else {
        "Better luck next time!".to_string()
    };

    let embed = CreateEmbed::new()
        .color(0x5b9e48)
        .title("🎰 Slot Machine Results")
        .footer(CreateEmbedFooter::new(footer_message))
        .fields([
            ("Spin Result", symbols, false),
            ("Payout", play_result.payout.to_string(), true),
        ]);

    embed
}
