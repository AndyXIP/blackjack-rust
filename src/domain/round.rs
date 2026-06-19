use super::deck::Deck;
use super::hand::Hand;

#[derive(Debug)]
pub enum RoundOutcome {
    PlayerBlackjack,
    DealerBlackjack,
    BothBlackjack,
    PlayerBust,
    DealerBust,
    PlayerWins,
    DealerWins,
    Tie,
}

impl RoundOutcome {
    pub fn winnings(&self, bet: i32) -> i32 {
        match self {
            RoundOutcome::PlayerBlackjack => (bet as f64 * 1.5) as i32,
            RoundOutcome::DealerBlackjack | RoundOutcome::PlayerBust | RoundOutcome::DealerWins => {
                -bet
            }
            RoundOutcome::BothBlackjack | RoundOutcome::Tie => 0,
            RoundOutcome::DealerBust | RoundOutcome::PlayerWins => bet,
        }
    }
}

pub struct Round {
    pub player_hands: Vec<Hand>,
    pub dealer_hand: Hand,
    current_hand: usize,
    doubled: Vec<bool>,
}

impl Round {
    pub fn new(deck: &mut Deck) -> Option<Self> {
        let mut player_hand = Hand::new();
        let mut dealer_hand = Hand::new();
        player_hand.hit(deck.deal()?);
        dealer_hand.hit(deck.deal()?);
        player_hand.hit(deck.deal()?);
        dealer_hand.hit(deck.deal()?);
        Some(Self {
            player_hands: vec![player_hand],
            dealer_hand,
            current_hand: 0,
            doubled: vec![false],
        })
    }

    pub fn current_hand(&self) -> &Hand {
        &self.player_hands[self.current_hand]
    }

    pub fn current_hand_index(&self) -> usize {
        self.current_hand
    }

    pub fn hand_count(&self) -> usize {
        self.player_hands.len()
    }

    pub fn is_doubled(&self, index: usize) -> bool {
        self.doubled[index]
    }

    pub fn player_has_blackjack(&self) -> bool {
        self.player_hands.len() == 1 && self.player_hands[0].is_blackjack()
    }

    pub fn dealer_has_blackjack(&self) -> bool {
        self.dealer_hand.is_blackjack()
    }

    pub fn can_split(&self) -> bool {
        let cards = self.player_hands[self.current_hand].cards();
        cards.len() == 2 && cards[0].value() == cards[1].value()
    }

    pub fn can_double(&self) -> bool {
        self.player_hands[self.current_hand].cards().len() == 2
    }

    pub fn split(&mut self, deck: &mut Deck) -> Option<()> {
        let second_card = self.player_hands[self.current_hand].split_off();
        self.player_hands[self.current_hand].hit(deck.deal()?);
        let mut new_hand = Hand::new();
        new_hand.hit(second_card);
        new_hand.hit(deck.deal()?);
        self.player_hands.insert(self.current_hand + 1, new_hand);
        self.doubled.insert(self.current_hand + 1, false);
        Some(())
    }

    pub fn player_hit(&mut self, deck: &mut Deck) -> Option<()> {
        let card = deck.deal()?;
        self.player_hands[self.current_hand].hit(card);
        Some(())
    }

    pub fn player_double(&mut self, deck: &mut Deck) -> Option<()> {
        let card = deck.deal()?;
        self.player_hands[self.current_hand].hit(card);
        self.doubled[self.current_hand] = true;
        Some(())
    }

    pub fn advance_hand(&mut self) {
        self.current_hand += 1;
    }

    pub fn resolve_dealer(&mut self, deck: &mut Deck) -> Option<()> {
        while self.dealer_hand.value() < 17 || self.dealer_hand.is_soft_seventeen() {
            self.dealer_hand.hit(deck.deal()?);
        }
        Some(())
    }

    pub fn outcome_for_hand(&self, index: usize) -> RoundOutcome {
        let hand = &self.player_hands[index];
        if hand.is_bust() {
            return RoundOutcome::PlayerBust;
        }
        if self.dealer_hand.is_bust() {
            return RoundOutcome::DealerBust;
        }
        match hand.value().cmp(&self.dealer_hand.value()) {
            std::cmp::Ordering::Greater => RoundOutcome::PlayerWins,
            std::cmp::Ordering::Less => RoundOutcome::DealerWins,
            std::cmp::Ordering::Equal => RoundOutcome::Tie,
        }
    }

    pub fn winnings_for_hand(&self, index: usize, bet: i32) -> i32 {
        let effective_bet = if self.doubled[index] { bet * 2 } else { bet };
        self.outcome_for_hand(index).winnings(effective_bet)
    }

    pub fn total_winnings(&self, bet: i32) -> i32 {
        (0..self.player_hands.len())
            .map(|i| self.winnings_for_hand(i, bet))
            .sum()
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

    fn hand(ranks: &[Rank]) -> Hand {
        let mut h = Hand::new();
        for &rank in ranks {
            h.hit(heart(rank));
        }
        h
    }

    impl Round {
        fn from_hands(player_hand: Hand, dealer_hand: Hand) -> Self {
            Self {
                player_hands: vec![player_hand],
                dealer_hand,
                current_hand: 0,
                doubled: vec![false],
            }
        }
    }

    #[test]
    fn blackjack_pays_three_to_two() {
        assert_eq!(RoundOutcome::PlayerBlackjack.winnings(10), 15);
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
    fn dealer_blackjack_player_loses() {
        assert_eq!(RoundOutcome::DealerBlackjack.winnings(10), -10);
    }

    #[test]
    fn both_blackjack_is_push() {
        assert_eq!(RoundOutcome::BothBlackjack.winnings(10), 0);
    }

    #[test]
    fn player_wins_higher_hand() {
        let round = Round::from_hands(
            hand(&[Rank::Number(10), Rank::Number(9)]),
            hand(&[Rank::Number(10), Rank::Number(7)]),
        );
        assert!(matches!(
            round.outcome_for_hand(0),
            RoundOutcome::PlayerWins
        ));
    }

    #[test]
    fn dealer_wins_higher_hand() {
        let round = Round::from_hands(
            hand(&[Rank::Number(10), Rank::Number(6)]),
            hand(&[Rank::Number(10), Rank::Number(9)]),
        );
        assert!(matches!(
            round.outcome_for_hand(0),
            RoundOutcome::DealerWins
        ));
    }

    #[test]
    fn tie_equal_hands() {
        let round = Round::from_hands(
            hand(&[Rank::Number(10), Rank::Number(8)]),
            hand(&[Rank::Number(10), Rank::Number(8)]),
        );
        assert!(matches!(round.outcome_for_hand(0), RoundOutcome::Tie));
    }

    #[test]
    fn player_bust_outcome() {
        let round = Round::from_hands(
            hand(&[Rank::King, Rank::Queen, Rank::Number(2)]),
            hand(&[Rank::Number(10), Rank::Number(8)]),
        );
        assert!(matches!(
            round.outcome_for_hand(0),
            RoundOutcome::PlayerBust
        ));
    }

    #[test]
    fn dealer_bust_outcome() {
        let round = Round::from_hands(
            hand(&[Rank::Number(10), Rank::Number(8)]),
            hand(&[Rank::King, Rank::Queen, Rank::Number(5)]),
        );
        assert!(matches!(
            round.outcome_for_hand(0),
            RoundOutcome::DealerBust
        ));
    }

    #[test]
    fn player_blackjack_detected() {
        let round = Round::from_hands(
            hand(&[Rank::Ace, Rank::King]),
            hand(&[Rank::Number(7), Rank::Number(8)]),
        );
        assert!(round.player_has_blackjack());
        assert!(!round.dealer_has_blackjack());
    }

    #[test]
    fn dealer_blackjack_detected() {
        let round = Round::from_hands(
            hand(&[Rank::Number(7), Rank::Number(8)]),
            hand(&[Rank::Ace, Rank::King]),
        );
        assert!(round.dealer_has_blackjack());
        assert!(!round.player_has_blackjack());
    }

    #[test]
    fn both_blackjack_detected() {
        let round = Round::from_hands(
            hand(&[Rank::Ace, Rank::King]),
            hand(&[Rank::Ace, Rank::Queen]),
        );
        assert!(round.player_has_blackjack());
        assert!(round.dealer_has_blackjack());
    }

    #[test]
    fn can_split_equal_value_cards() {
        let round = Round::from_hands(
            hand(&[Rank::Number(8), Rank::Number(8)]),
            hand(&[Rank::Number(7), Rank::Number(8)]),
        );
        assert!(round.can_split());
    }

    #[test]
    fn can_split_ten_value_cards() {
        let round = Round::from_hands(
            hand(&[Rank::Jack, Rank::Queen]),
            hand(&[Rank::Number(7), Rank::Number(8)]),
        );
        assert!(round.can_split());
    }

    #[test]
    fn cannot_split_different_value_cards() {
        let round = Round::from_hands(
            hand(&[Rank::Number(7), Rank::Number(8)]),
            hand(&[Rank::Number(7), Rank::Number(8)]),
        );
        assert!(!round.can_split());
    }

    #[test]
    fn double_doubles_effective_bet() {
        let mut round = Round::from_hands(
            hand(&[Rank::Number(9), Rank::Number(9)]),
            hand(&[Rank::Number(7), Rank::Number(8)]),
        );
        round.doubled[0] = true;
        assert!(matches!(
            round.outcome_for_hand(0),
            RoundOutcome::PlayerWins
        ));
        assert_eq!(round.winnings_for_hand(0, 10), 20);
    }
}
