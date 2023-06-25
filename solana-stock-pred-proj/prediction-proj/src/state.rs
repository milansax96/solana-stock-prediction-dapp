use anchor_lang::prelude::*;
use anchor_lang::prelude::borsh;

#[account]
pub struct Master {
    pub last_bet_id: u64,
}

#[account]
pub struct Bet {
    // Unique identifier per user
    pub id: u64,
    // How much it costs to take the bet in lamports
    pub amount: u64,
    // Maker's prediction (i.e. the person who made the initial bet)
    pub prediction_a: BetPrediction,
    // Taker's prediction, NONE at bet creation (i.e. the person who is challenge person A's initial bet)
    pub prediction_b: Option<BetPrediction>,
    // Current state of the bet
    pub state: BetState,
    // Pyth price oracle account
    pub pyth_price_key: Pubkey,
    // Bet becomes invalid after this (UNIX Timestamp)
    pub expiry_ts: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BetPrediction {
    // The address that bets
    pub player: Pubkey,
    // Price prediction in USD
    pub price: f64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum BetState {
    Created,
    Started,
    PlayerAWon,
    PlayerBWon,
    Draw
}