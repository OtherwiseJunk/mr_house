mod commands;
mod services;

use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use once_cell::sync::Lazy;
//use crate::commands::slot_machine::{generate_gore_slots, PlayResult, SlotMachine};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub static PANOPTICON_TOKEN: Lazy<String> = Lazy::new(|| {
    std::env::var("PANOPTICON_TOKEN")
        .expect("Expected PANOPTICON_TOKEN environment variable")
});

pub static PREVIOUS_ROLLING_JACKPOT: Lazy<f64> = Lazy::new(|| {
    std::env::var("PREVIOUS_ROLLING_JACKPOT")
        .ok()
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0)
});

pub struct Data {}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN environment variable");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::get_commands(),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands)
                .await
                .map_err(Error::from)?;
                Ok(Data {})
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .expect("Error creating client");

    client.start().await.unwrap();
}

/*fn test_gore_slots() {
    let mut slot_machine = generate_gore_slots(*PREVIOUS_ROLLING_JACKPOT);
    // Simulate 100000 spins to calculate RTP
    let mut plays: Vec<PlayResult> = Vec::new();
    let mut jackpot_count = 0;
    let mut jackpot_winnings = 0;
    for _ in 0..100000 {
        let play_result = slot_machine.play();
        if play_result.payout >= 500 {
            jackpot_winnings += play_result.payout;
            jackpot_count += 1;
        }
        // println!("Play Result: {:?}", play_result.symbols);
        // println!("Payout: {}", play_result.payout);
        // println!(
        //     "Current Jackpot Value: {}",
        //     play_result.current_jackpot_value
        // );
        plays.push(play_result);
    }
    // Calculate RTP
    let total_payout: u32 = plays.iter().map(|play| play.payout).sum();
    let total_cost: u32 = plays.len() as u32 * 10; // Assuming each play costs 10 libcoins;
    let rtp = (total_payout as f64 / total_cost as f64) * 100.0; // RTP in percentage
    println!("Total Payout: {}", total_payout);
    println!("Total Cost: {}", total_cost);
    println!("RTP: {:.2}%", rtp);
    println!("Jackpot Count: {}", jackpot_count);
    println!("Jackpot Winnings: {}", jackpot_winnings);
    println!(
        "Average Jackpot Winnings: {:.2}",
        jackpot_winnings as f64 / jackpot_count as f64
    );
}*/
