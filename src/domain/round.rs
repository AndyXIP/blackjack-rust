use super::deck::Deck;
use super::hand::Hand;

pub enum RoundOutcome {
    PlayerBlackjack,
    PlayerBust,
    DealerBust,
    PlayerWins,
    DealerWins,
    Tie,
}

impl RoundOutcome {
    pub fn winnings(&self, bet: i32) -> i32 {
        match self {
            RoundOutcome::PlayerBlackjack => (bet as f64 * 2.5) as i32,
            RoundOutcome::PlayerBust | RoundOutcome::DealerWins => -bet,
            RoundOutcome::DealerBust | RoundOutcome::PlayerWins => bet,
            RoundOutcome::Tie => 0,
        }
    }
}

pub struct Round {
    pub player_hand: Hand,
    pub dealer_hand: Hand,
}

impl Round {
    pub fn new(deck: &mut Deck) -> Self {
        let mut player_hand = Hand::new();
        let mut dealer_hand = Hand::new();
        player_hand.hit(deck.deal().unwrap());
        dealer_hand.hit(deck.deal().unwrap());
        player_hand.hit(deck.deal().unwrap());
        dealer_hand.hit(deck.deal().unwrap());
        Self {
            player_hand,
            dealer_hand,
        }
    }

    pub fn player_hit(&mut self, deck: &mut Deck) {
        self.player_hand.hit(deck.deal().unwrap());
    }

    pub fn resolve_dealer(&mut self, deck: &mut Deck) {
        while self.dealer_hand.value() < 17 {
            self.dealer_hand.hit(deck.deal().unwrap());
        }
    }

    pub fn outcome(&self) -> RoundOutcome {
        if self.player_hand.is_blackjack() {
            return RoundOutcome::PlayerBlackjack;
        }
        if self.player_hand.is_bust() {
            return RoundOutcome::PlayerBust;
        }
        if self.dealer_hand.is_bust() {
            return RoundOutcome::DealerBust;
        }
        match self.player_hand.value().cmp(&self.dealer_hand.value()) {
            std::cmp::Ordering::Greater => RoundOutcome::PlayerWins,
            std::cmp::Ordering::Less => RoundOutcome::DealerWins,
            std::cmp::Ordering::Equal => RoundOutcome::Tie,
        }
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

    impl Round {
        fn from_hands(player_hand: Hand, dealer_hand: Hand) -> Self {
            Self {
                player_hand,
                dealer_hand,
            }
        }
    }

    fn hand(ranks: &[Rank]) -> Hand {
        let mut h = Hand::new();
        for &rank in ranks {
            h.hit(heart(rank));
        }
        h
    }

    #[test]
    fn blackjack_winnings() {
        assert_eq!(RoundOutcome::PlayerBlackjack.winnings(10), 25);
    }

    #[test]
    fn win_winnings() {
        assert_eq!(RoundOutcome::PlayerWins.winnings(10), 10);
    }

    #[test]
    fn loss_winnings() {
        assert_eq!(RoundOutcome::DealerWins.winnings(10), -10);
    }

    #[test]
    fn tie_winnings() {
        assert_eq!(RoundOutcome::Tie.winnings(10), 0);
    }

    #[test]
    fn player_wins_higher_hand() {
        let round = Round::from_hands(
            hand(&[Rank::Number(10), Rank::Number(9)]),
            hand(&[Rank::Number(10), Rank::Number(7)]),
        );
        assert!(matches!(round.outcome(), RoundOutcome::PlayerWins));
    }

    #[test]
    fn dealer_wins_higher_hand() {
        let round = Round::from_hands(
            hand(&[Rank::Number(10), Rank::Number(6)]),
            hand(&[Rank::Number(10), Rank::Number(9)]),
        );
        assert!(matches!(round.outcome(), RoundOutcome::DealerWins));
    }

    #[test]
    fn tie_equal_hands() {
        let round = Round::from_hands(
            hand(&[Rank::Number(10), Rank::Number(8)]),
            hand(&[Rank::Number(10), Rank::Number(8)]),
        );
        assert!(matches!(round.outcome(), RoundOutcome::Tie));
    }

    #[test]
    fn player_bust_outcome() {
        let round = Round::from_hands(
            hand(&[Rank::King, Rank::Queen, Rank::Number(2)]),
            hand(&[Rank::Number(10), Rank::Number(8)]),
        );
        assert!(matches!(round.outcome(), RoundOutcome::PlayerBust));
    }

    #[test]
    fn dealer_bust_outcome() {
        let round = Round::from_hands(
            hand(&[Rank::Number(10), Rank::Number(8)]),
            hand(&[Rank::King, Rank::Queen, Rank::Number(5)]),
        );
        assert!(matches!(round.outcome(), RoundOutcome::DealerBust));
    }
}
