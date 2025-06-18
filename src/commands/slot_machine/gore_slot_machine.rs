use super::slot_machine::*;
use std::collections::HashMap;

fn gore_slots_paytable() -> Vec<PayRule> {
    vec![
        PayRule {
            pattern: PayPattern::FiveOfAKind(Symbol::Gore),
            payout: 500,
            is_jackpot: true,
        },
        PayRule {
            pattern: PayPattern::FiveOfAKind(Symbol::Mean),
            payout: 250,
            is_jackpot: false,
        },
        PayRule {
            pattern: PayPattern::FiveOfAKind(Symbol::Magnathonk),
            payout: 250,
            is_jackpot: false,
        },
        PayRule {
            pattern: PayPattern::ThreeOfAKind(Symbol::Smugbrow),
            payout: 25,
            is_jackpot: false,
        },
        PayRule {
            pattern: PayPattern::ThreeOfAKind(Symbol::Smileyes),
            payout: 25,
            is_jackpot: false,
        },
        PayRule {
            pattern: PayPattern::MinCount(Symbol::Smileyes, 2),
            payout: 6,
            is_jackpot: false,
        },
        PayRule {
            pattern: PayPattern::MinCountAnyDistribution(
                vec![Symbol::Smugbrow, Symbol::Magnathonk, Symbol::Mean],
                3,
            ),
            payout: 6,
            is_jackpot: false,
        },
        PayRule {
            pattern: PayPattern::MinCount(Symbol::Smileyes, 1),
            payout: 2,
            is_jackpot: false,
        },
    ]
}

pub fn generate_gore_slots(previous_rolling_jackpot: f64) -> SlotMachine {
    let weighted_symbol_pool: Vec<Symbol> = generate_gore_slots_weights();
    let pay_table: Vec<PayRule> = gore_slots_paytable();
    let cost_per_play: u32 = 10; // Cost per play in cents
    let jackpot_growth_rate: f64 = 0.01; // 5% growth rate for the jackpot
    let slot_machine = SlotMachine::new(
        cost_per_play,
        pay_table,
        jackpot_growth_rate,
        weighted_symbol_pool,
        previous_rolling_jackpot,
    );
    return slot_machine;
}

fn generate_gore_slots_weights() -> Vec<Symbol> {
    let mut weights = HashMap::new();
    weights.insert(Symbol::Gore, 9 as f64);
    weights.insert(Symbol::Mean, 10 as f64);
    weights.insert(Symbol::Magnathonk, 12 as f64);
    weights.insert(Symbol::Smugbrow, 19 as f64);
    weights.insert(Symbol::Smileyes, 20 as f64);
    weights.insert(Symbol::Blank, 6 as f64);

    return generate_weighted_symbol_pool(weights.clone());
}
