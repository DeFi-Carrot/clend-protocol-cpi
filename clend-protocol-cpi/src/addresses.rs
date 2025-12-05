use std::str::FromStr;

use anchor_lang::prelude::Pubkey;

// clend event authority address
// this is the same for the entire clend protocol
// used by clend protocol to emit events via CPI
pub fn clend_ea() -> Pubkey {
    Pubkey::from_str("CVq8mEtBtcc74sx4zgjtS9AHvcsL8DsKFugU55QeK7jd").unwrap()
}
