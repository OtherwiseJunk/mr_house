use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Symbol {
    Gore,
    Mean,
    Magnathonk,
    Smugbrow,
    Smileyes,
    Blank,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PayPattern {
    FiveOfAKind(Symbol),
    ThreeOfAKind(Symbol),
    MinCountAnyDistribution(Vec<Symbol>, u8),
    MinCount(Symbol, u8),
}

#[derive(Debug, Clone)]
pub struct PayRule {
    pub pattern: PayPattern,
    pub payout: u32,
    pub is_jackpot: bool,
}

pub struct PlayResult {
    pub symbols: Vec<String>,
    pub payout: u32,
    pub current_jackpot_value: f64,
}

pub struct SlotMachine {
    cost_per_play: u32,
    pay_table: Vec<PayRule>,
    rolling_jackpot: f64,
    min_jackpot: u32,
    jackpot_growth_rate: f64,
    symbol_map: HashMap<Symbol, String>,
    weighted_symbol_pool: Vec<Symbol>,
}

pub struct RTPBalancer {}

fn check_pay_pattern_match(symbols: &[Symbol], pattern: &PayPattern) -> bool {
    match pattern {
        PayPattern::FiveOfAKind(symbol) => symbols.iter().all(|&s| s == *symbol),
        PayPattern::ThreeOfAKind(symbol) => symbols.iter().filter(|&&s| s == *symbol).count() >= 3,
        PayPattern::MinCountAnyDistribution(symbols_list, min_count) => {
            let total = symbols_list
                .iter()
                .map(|&s| symbols.iter().filter(|&&sym| sym == s).count())
                .sum::<usize>();
            total >= *min_count as usize
        }
        PayPattern::MinCount(symbol, min_count) => {
            symbols.iter().filter(|&&s| s == *symbol).count() >= *min_count as usize
        }
    }
}

fn single_spin(
    weighted_symbol_pool: &[Symbol],
    pay_table: &[PayRule],
    current_jackpot: f64,
    cost_per_play: u32,
    jackpot_growth_rate: f64,
    min_jackpot: u32,
) -> (f64, f64, Vec<Symbol>) {
    let mut rng = rand::rng();

    let mut generated_symbols: Vec<Symbol> = (0..5)
        .map(|_| {
            *weighted_symbol_pool
                .get(rng.random_range(0..weighted_symbol_pool.len()))
                .unwrap()
        })
        .collect();

    generated_symbols.sort_unstable();

    let mut spin_payout: f64 = 0.0;
    let mut jackpot_hit_this_spin = false;

    for rule in pay_table {
        if check_pay_pattern_match(&generated_symbols, &rule.pattern) {
            spin_payout = rule.payout as f64;
            if rule.is_jackpot {
                spin_payout = current_jackpot;
                jackpot_hit_this_spin = true;
            }
            break;
        }
    }

    let next_jackpot_value: f64 = if jackpot_hit_this_spin {
        min_jackpot as f64
    } else {
        current_jackpot as f64 + (cost_per_play as f64 * jackpot_growth_rate) as f64
    };

    (spin_payout, next_jackpot_value, generated_symbols)
}

pub fn generate_weighted_symbol_pool(weights: HashMap<Symbol, f64>) -> Vec<Symbol> {
    let mut weighted_symbol_pool = Vec::new();
    for (symbol, weight) in weights {
        for _ in 0..weight as usize {
            weighted_symbol_pool.push(symbol);
        }
    }

    weighted_symbol_pool
}

impl SlotMachine {
    pub fn new(
        cost_per_play: u32,
        pay_table: Vec<PayRule>,
        jackpot_growth_rate: f64,
        weighted_symbol_pool: Vec<Symbol>,
    ) -> Self {
        let min_jackpot = pay_table
            .iter()
            .find(|rule| rule.is_jackpot)
            .map(|rule| rule.payout)
            .expect("Pay table must contain at least one jackpot rule (is_jackpot = true)");

        let mut symbol_map = HashMap::new();
        symbol_map.insert(Symbol::Gore, "<:gore:854587419391164457>".to_string());
        symbol_map.insert(Symbol::Mean, "<:mean:1260290196541280288>".to_string());
        symbol_map.insert(
            Symbol::Magnathonk,
            "<:magnathonk:928158272836472872>".to_string(),
        );
        symbol_map.insert(
            Symbol::Smugbrow,
            "<:smugbrow:1013536644621664268>".to_string(),
        );
        symbol_map.insert(
            Symbol::Smileyes,
            "<:smileyes:927806099661422613>".to_string(),
        );
        symbol_map.insert(Symbol::Blank, "<:white:785272845890486293>".to_string());

        SlotMachine {
            cost_per_play,
            pay_table,
            rolling_jackpot: min_jackpot as f64,
            min_jackpot,
            jackpot_growth_rate,
            symbol_map,
            weighted_symbol_pool,
        }
    }

    pub fn play(&mut self) -> PlayResult {
        let (payout, next_jackpot_value, generated_symbols) = single_spin(
            &self.weighted_symbol_pool,
            &self.pay_table,
            self.rolling_jackpot,
            self.cost_per_play,
            self.jackpot_growth_rate,
            self.min_jackpot,
        );

        self.rolling_jackpot = next_jackpot_value;

        let display_symbols: Vec<String> = generated_symbols
            .iter()
            .map(|&s| self.get_symbol_string(s))
            .collect();
        let payout_u32 = payout as u32;

        PlayResult {
            symbols: display_symbols,
            payout: payout_u32,
            current_jackpot_value: self.rolling_jackpot,
        }
    }

    fn get_symbol_string(&self, symbol: Symbol) -> String {
        self.symbol_map
            .get(&symbol)
            .cloned()
            .unwrap_or_else(|| "Unknown".to_string())
    }
}
