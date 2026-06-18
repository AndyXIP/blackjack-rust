#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn value(&self) -> u8 {
        match self.rank {
            Rank::Number(n) => n,
            Rank::Jack | Rank::Queen | Rank::King => 10,
            Rank::Ace => 11,
        }
    }

    pub fn display(&self) -> String {
        let rank_str = match self.rank {
            Rank::Number(n) => n.to_string(),
            Rank::Jack => "Jack".to_string(),
            Rank::Queen => "Queen".to_string(),
            Rank::King => "King".to_string(),
            Rank::Ace => "Ace".to_string(),
        };
        let suit_str = match self.suit {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        };
        format!("{} of {}", rank_str, suit_str)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy)]
pub enum Rank {
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn heart(rank: Rank) -> Card {
        Card { suit: Suit::Hearts, rank }
    }

    #[test]
    fn number_card_value() {
        assert_eq!(heart(Rank::Number(7)).value(), 7);
    }

    #[test]
    fn face_cards_value_ten() {
        for rank in [Rank::Jack, Rank::Queen, Rank::King] {
            assert_eq!(heart(rank).value(), 10);
        }
    }

    #[test]
    fn ace_value_is_eleven() {
        assert_eq!(heart(Rank::Ace).value(), 11);
    }
}
