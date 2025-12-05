use std::str::FromStr;

use anchor_lang::prelude::Pubkey;

pub fn event_authority_address() -> Pubkey {
    Pubkey::from_str("CVq8mEtBtcc74sx4zgjtS9AHvcsL8DsKFugU55QeK7jd").unwrap()
}
