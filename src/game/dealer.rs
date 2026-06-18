use crate::{Card, Rank};

pub fn card_value(card: &Card) -> u8 {
    match card.rank {
        Rank::Number(n) => n,
        Rank::Jack | Rank::Queen | Rank::King => 10,
        Rank::Ace => 11,
    }
}
