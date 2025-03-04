use std::{cmp::Ordering, fmt::Debug};

use crate::{
    action::Action, action_collection::ActionCollection, card::Card, mcts::state::State,
    stack::Stack, suite::Suite, trick::Trick,
};

#[derive(Default, Clone, Copy, Debug)]
enum RoundPhase {
    #[default]
    PickTrump,
    PlayCards,
}

#[derive(Default, Clone, Copy)]
pub struct Round {
    turn: usize,
    dealer: usize,
    player_cards: [Stack; 4],
    played_cards: Stack,
    scores: [i16; 2],
    trick: Trick,
    phase: RoundPhase,
}

impl Round {
    pub fn new(dealer: usize) -> Self {
        let mut round = Round::default();

        round.set_dealer(dealer);
        round.deal_cards();

        round
    }

    const fn set_dealer(&mut self, dealer: usize) {
        self.dealer = dealer;
        self.turn = (dealer + 1) % 4;
    }

    pub fn setup_for_next_round(&mut self) {
        let next_dealer = (self.dealer + 1) % 4;
        self.set_dealer(next_dealer);
        self.deal_cards();

        self.played_cards = Stack::default();
        self.scores = [0; 2];
        self.trick.clear();
        self.phase = RoundPhase::PickTrump;
    }

    fn deal_cards(&mut self) {
        let mut indices: [u32; 32] = std::array::from_fn(|i| i as u32);
        let mut cards = [Stack::default(); 3];

        // number of cards per player
        let n = indices.len() / 4;

        for i in (n..4 * n).rev() {
            let j = romu::mod_usize(i + 1);
            indices.swap(i, j);

            cards[(i / n) - 1] |= 1 << indices[i];
        }

        self.player_cards[0] = cards[0];
        self.player_cards[1] = cards[1];
        self.player_cards[2] = cards[2];
        self.player_cards[3] = Stack::ALL ^ cards[0] ^ cards[1] ^ cards[2];
    }

    fn play_card(&mut self, card: Card) {
        self.trick.play(card, self.turn);
        self.played_cards |= 1 << card.get_index();
        self.player_cards[self.turn] ^= 1 << card.get_index();

        if self.trick.is_finished() {
            self.on_trick_finish();
        } else {
            self.turn = (self.turn + 1) % 4;
        }
    }

    const fn set_trump(&mut self, trump: Option<Suite>) {
        self.trick.set_trump(trump);
        self.phase = RoundPhase::PlayCards;
    }

    const fn on_trick_finish(&mut self) {
        let (_, winner) = self.trick.winner().unwrap();
        let winning_team = winner % 2;

        self.scores[winning_team] += self.trick.score() as i16;
        self.turn = winner;
        self.trick.clear();
    }

    fn possible_card_actions(&self) -> <Self as State>::ActionList {
        let mut cards = self.player_cards[self.turn];

        // have to follow if possible,
        if let Some(suite) = self.trick.suite_to_follow() {
            let filtered_cards = cards & suite.mask();
            if filtered_cards != 0 {
                cards = filtered_cards;
            }
        }

        // this also means we're not the first player, i.e. the suite
        // to follow has been determined
        if let Some((winning_card, winning_player)) = self.trick.winner() {
            // our team isn't winning
            if winning_player % 2 != self.turn % 2 {
                // have to buy if possible, but can't 'under-buy', except if that's our only possible move
                if let Some(trump) = self.trick.trump() {
                    let mut mask = Stack::all_above(winning_card) & winning_card.suite().mask();

                    // we can play any trump if the current winning card isn't a trump
                    if winning_card.suite() != trump {
                        mask |= trump.mask();
                    }

                    let filtered_cards = cards & mask;
                    if filtered_cards != 0 {
                        cards = filtered_cards;
                    }
                }
                // this means that we're playing without trump,
                // so we simply need to play a higher card of the same suite
                else {
                    let mask = Stack::all_above(winning_card) & winning_card.suite().mask();
                    let filtered_cards = cards & mask;

                    if filtered_cards != 0 {
                        cards = filtered_cards;
                    }
                }
            }
        }

        ActionCollection::Cards(cards)
    }

    /// TODO: add possibility to play without trump
    fn possible_trump_actions(&self) -> <Self as State>::ActionList {
        let cards = self.player_cards[self.dealer];
        let mut bits = 0;

        for suite in [Suite::Pijkens, Suite::Klavers, Suite::Harten, Suite::Koeken] {
            if cards.has_suite(suite) {
                bits |= 1 << suite as u8;
            }
        }

        ActionCollection::Trumps(bits)
    }

    pub const fn player_cards(&self, player: usize) -> Stack {
        self.player_cards[player]
    }

    pub const fn scores(&self) -> [i16; 2] {
        self.scores
    }
}

impl State for Round {
    type Action = Action;
    type ActionList = ActionCollection;

    fn turn(&self) -> usize {
        match self.phase {
            RoundPhase::PickTrump => self.dealer,
            RoundPhase::PlayCards => self.turn,
        }
    }

    fn randomize(&self, observer: usize) -> Self {
        let mut round = *self;
        let cards_to_deal = Stack::ALL ^ self.player_cards[observer] ^ self.played_cards;
        let mut indices = (0..32)
            .filter(|&x| cards_to_deal.has_index(x))
            .collect::<Vec<_>>();

        romu::shuffle(&mut indices);
        let mut start = 0;

        for i in 1..=3 {
            let n = self.player_cards[(observer + i) % 4].len() as usize;
            round.player_cards[(observer + i) % 4] =
                Stack::from_slice(&indices[start..(start + n)]);
            start += n;
        }

        round
    }

    fn possible_actions(&self) -> Self::ActionList {
        match self.phase {
            RoundPhase::PickTrump => self.possible_trump_actions(),
            RoundPhase::PlayCards => self.possible_card_actions(),
        }
    }

    fn apply_action(&mut self, action: Self::Action) {
        match action {
            Action::PlayCard(card) => self.play_card(card),
            Action::PickTrump(trump) => self.set_trump(trump),
        }
    }

    fn is_terminal(&self) -> bool {
        self.played_cards == Stack::ALL
    }

    fn reward(&self, perspective: usize) -> f32 {
        assert!(self.is_terminal());

        let team = perspective % 2;

        match self.scores[team].cmp(&self.scores[1 - team]) {
            Ordering::Greater => 1.,
            Ordering::Less => 0.,
            Ordering::Equal => 0.5,
        }
    }
}

impl Debug for Round {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..4 {
            writeln!(f, "player {i}: {:?}", self.player_cards[i])?;
        }

        f.debug_struct("Round")
            .field("turn", &self.turn)
            .field("dealer", &self.dealer)
            //.field("player_cards", &self.player_cards)
            .field("played_cards", &self.played_cards)
            .field("trick", &self.trick)
            .field("scores", &self.scores)
            .field("phase", &self.phase)
            .finish()
    }
}
