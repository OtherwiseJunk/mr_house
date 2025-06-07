use mr_house::slotmachine::*;
use std::collections::HashMap;

fn main() {
    let mut weights = HashMap::new();
    weights.insert(Symbol::Gore, 8 as f64);
    weights.insert(Symbol::Mean,10 as f64);
    weights.insert(Symbol::Magnathonk, 12 as f64);
    weights.insert(Symbol::Smugbrow, 28 as f64);
    weights.insert(Symbol::Smileyes, 27 as f64);

    let weighted_symbol_pool: Vec<Symbol> = generate_weighted_symbol_pool(weights.clone());
    let pay_table: Vec<PayRule> = example_paytable();
    let cost_per_play: u32 = 10; // Cost per play in cents
    let jackpot_growth_rate: f64 = 0.01; // 5% growth rate for the jackpot
    let mut slot_machine = SlotMachine::new(cost_per_play, pay_table, jackpot_growth_rate, weighted_symbol_pool);
    // Simulate 100000 spins to calculate RTP
    let mut plays: Vec<PlayResult> = Vec::new();
    let mut jackpot_count = 0;
    let mut jackpot_winnings = 0;
    for _ in 0..10 {
        let play_result = slot_machine.play();
        if play_result.payout >= 500 {
            jackpot_winnings += play_result.payout;
            jackpot_count += 1;
        }
        println!("Play Result: {:?}", play_result.symbols);
        println!("Payout: {}", play_result.payout);
        println!("Current Jackpot Value: {}", play_result.current_jackpot_value);
        plays.push(play_result);
    }
    // Calculate RTP
    let total_payout: u32 = plays.iter().map(|play| play.payout).sum();
    let total_cost: u32 = plays.len() as u32 * cost_per_play;
    let rtp = (total_payout as f64 / total_cost as f64) * 100.0; // RTP in percentage
    println!("Total Payout: {}", total_payout);
    println!("Total Cost: {}", total_cost);
    println!("RTP: {:.2}%", rtp);
    println!("Jackpot Count: {}", jackpot_count);
    println!("Jackpot Winnings: {}", jackpot_winnings);
    println!("Average Jackpot Winnings: {:.2}", jackpot_winnings as f64 / jackpot_count as f64);
}

fn example_paytable() -> Vec<PayRule> {
        vec![
            PayRule { pattern: PayPattern::FiveOfAKind(Symbol::Gore), payout: 500, is_jackpot: true },
            PayRule { pattern: PayPattern::FiveOfAKind(Symbol::Mean), payout: 250, is_jackpot: false },
            PayRule { pattern: PayPattern::ThreeOfAKind(Symbol::Smugbrow), payout: 25, is_jackpot: false },
            PayRule { pattern: PayPattern::MinCount(Symbol::Smileyes, 2), payout: 5, is_jackpot: false },
            PayRule { pattern: PayPattern::MinCountAnyDistribution(vec![Symbol::Smugbrow, Symbol::Magnathonk, Symbol::Mean], 3), payout: 5, is_jackpot: false },
            PayRule { pattern: PayPattern::MinCount(Symbol::Smileyes, 1), payout: 2, is_jackpot: false },
        ]
}

fn generate_weighted_symbol_pool(weights: HashMap<Symbol, f64>) -> Vec<Symbol> {
        let mut weighted_symbol_pool = Vec::new();
        for (symbol, weight) in weights {
            for _ in 0..weight as usize {
                weighted_symbol_pool.push(symbol);
            }
        }

        weighted_symbol_pool
}