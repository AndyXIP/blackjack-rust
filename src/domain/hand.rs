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

    pub fn split_off(&mut self) -> Card {
        self.cards.pop().expect("cannot split empty hand")
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

    pub fn is_soft_seventeen(&self) -> bool {
        if self.value() != 17 {
            return false;
        }
        let hard_total: u8 = self
            .cards
            .iter()
            .map(|c| match c.rank {
                Rank::Number(n) => n,
                Rank::Jack | Rank::Queen | Rank::King => 10,
                Rank::Ace => 1,
            })
            .sum();
        let has_ace = self.cards.iter().any(|c| matches!(c.rank, Rank::Ace));
        has_ace && hard_total + 10 == 17
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::card::{Card, Rank, Suit};

    fn heart(rank: Rank) -> Card {
        Card {
            suit: Suit::Hearts,
            rank,
        }
    }

    #[test]
    fn basic_value() {
        let mut hand = Hand::new();
        hand.hit(heart(Rank::Number(7)));
        hand.hit(heart(Rank::Number(8)));
        assert_eq!(hand.value(), 15);
    }

    #[test]
    fn ace_counts_as_eleven() {
        let mut hand = Hand::new();
        hand.hit(heart(Rank::Ace));
        hand.hit(heart(Rank::Number(9)));
        assert_eq!(hand.value(), 20);
    }

    #[test]
    fn ace_adjusts_to_avoid_bust() {
        let mut hand = Hand::new();
        hand.hit(heart(Rank::Ace));
        hand.hit(heart(Rank::King));
        hand.hit(heart(Rank::Number(5)));
        assert_eq!(hand.value(), 16);
    }

    #[test]
    fn multiple_aces_adjust() {
        let mut hand = Hand::new();
        hand.hit(heart(Rank::Ace));
        hand.hit(heart(Rank::Ace));
        hand.hit(heart(Rank::Number(9)));
        assert_eq!(hand.value(), 21);
    }

    #[test]
    fn is_bust() {
        let mut hand = Hand::new();
        hand.hit(heart(Rank::King));
        hand.hit(heart(Rank::Queen));
        hand.hit(heart(Rank::Number(2)));
        assert!(hand.is_bust());
    }

    #[test]
    fn blackjack_detected() {
        let mut hand = Hand::new();
        hand.hit(heart(Rank::Ace));
        hand.hit(heart(Rank::King));
        assert!(hand.is_blackjack());
    }

    #[test]
    fn three_card_twenty_one_is_not_blackjack() {
        let mut hand = Hand::new();
        hand.hit(heart(Rank::Number(7)));
        hand.hit(heart(Rank::Number(7)));
        hand.hit(heart(Rank::Number(7)));
        assert_eq!(hand.value(), 21);
        assert!(!hand.is_blackjack());
    }

    #[test]
    fn soft_seventeen_detected() {
        let mut hand = Hand::new();
        hand.hit(heart(Rank::Ace));
        hand.hit(heart(Rank::Number(6)));
        assert!(hand.is_soft_seventeen());
    }

    #[test]
    fn hard_seventeen_not_soft() {
        let mut hand = Hand::new();
        hand.hit(heart(Rank::Number(10)));
        hand.hit(heart(Rank::Number(7)));
        assert!(!hand.is_soft_seventeen());
    }
}
