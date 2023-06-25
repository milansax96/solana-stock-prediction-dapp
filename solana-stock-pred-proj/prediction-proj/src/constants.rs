use anchor_lang::solana_program::clock::UnixTimestamp;

pub const MASTER_SEED: &[u8] = b"master";

pub const BET_SEED: &[u8] = b"bet";

// The closer we are to the expiry, the more chances a player has to win
// Adjust this to make it more fair
// For the last two minutes, people CANNOT bet
pub const MINIMUM_REMAINING_TIME_UNTIL_EXPIRY: UnixTimestamp = 120;

// Claim period for winner to take their money (in seconds)
pub const MAXIMUM_CLAIMABLE_PERIOD: UnixTimestamp = 300;