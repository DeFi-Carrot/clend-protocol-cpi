use std::str::FromStr;

use anchor_lang::prelude::Pubkey;

pub fn clend_event_authority() -> Pubkey {
    Pubkey::from_str("CVq8mEtBtcc74sx4zgjtS9AHvcsL8DsKFugU55QeK7jd").unwrap()
}
