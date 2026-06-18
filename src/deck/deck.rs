use super::card::{Card, Rank, Suit};

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        for suit in [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for rank in 2..=10 {
                cards.push(Card {
                    suit,
                    rank: Rank::Number(rank),
                });
            }
            cards.push(Card {
                suit,
                rank: Rank::Jack,
            });
            cards.push(Card {
                suit,
                rank: Rank::Queen,
            });
            cards.push(Card {
                suit,
                rank: Rank::King,
            });
            cards.push(Card {
                suit,
                rank: Rank::Ace,
            });
        }
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        let mut rng = rand::rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}
