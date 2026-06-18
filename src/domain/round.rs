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
        Self { player_hand, dealer_hand }
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
