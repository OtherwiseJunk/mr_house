use crate::{Data, Error};
use poise::Command;

pub mod info;
pub mod slot_machine;

pub use self::info::info;
pub use self::slot_machine::*;

pub fn get_commands() -> Vec<Command<Data, Error>> {
    vec![info::info(), slots::slots()]
}
