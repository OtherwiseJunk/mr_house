use super::{generate_gore_slots, SlotMachine};
use crate::{Context, Error};
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static GORE_SLOT_MACHINE: Lazy<Mutex<SlotMachine>> =
    Lazy::new(|| Mutex::new(generate_gore_slots()));

#[poise::command(slash_command)]
pub async fn slots(ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
