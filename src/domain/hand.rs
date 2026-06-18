use super::card::{Card, Rank};

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Default for Hand {
    fn default() -> Self {
        Self::new()
    }
}

impl Hand {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    pub fn hit(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn value(&self) -> u8 {
        let mut total = 0;
        let mut aces = 0;

        for card in &self.cards {
            match card.rank {
                Rank::Number(n) => total += n,
                Rank::Jack | Rank::Queen | Rank::King => total += 10,
                Rank::Ace => {
                    total += 11;
                    aces += 1;
                }
            }
        }

        while total > 21 && aces > 0 {
            total -= 10;
            aces -= 1;
        }

        total
    }

    pub fn first_card(&self) -> Option<&Card> {
        self.cards.first()
    }

    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    pub fn is_bust(&self) -> bool {
        self.value() > 21
    }

    pub fn is_blackjack(&self) -> bool {
        self.cards.len() == 2 && self.value() == 21
    }
}
