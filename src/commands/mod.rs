use crate::{Data, Error};
use poise::Command;

pub mod info;
pub mod slot_machine;
pub mod libcoin;

pub fn get_commands() -> Vec<Command<Data, Error>> {
    vec![
        info::info(), 
        slot_machine::slots::slots(), 
        slot_machine::slots::paytable(),
        libcoin::balance(), 
        libcoin::deduct(), 
        libcoin::grant(),
        slot_machine::slots::stats(),
    ]
}
