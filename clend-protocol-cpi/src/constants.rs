use fixed::types::I80F48 as FixedI80F48;
use fixed_macro::types::I80F48;

// interpret a small dust amount as an empty balance
pub const EMPTY_BALANCE_THRESHOLD: FixedI80F48 = I80F48!(1);
