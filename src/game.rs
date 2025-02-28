use std::fmt::Debug;

use crate::{
    action::Action,
    game_state::GameState,
    mcts::{action_list::ActionList, state::State},
    players::PlayerVec,
};

#[derive(Default)]
pub struct Game {
    pub players: PlayerVec,
    state: GameState,
}

impl Game {
    pub fn new(mut players: PlayerVec) -> Self {
        let mut game = Game::default();

        for (i, player) in players.iter_mut().enumerate() {
            player.set_index(i);
        }

        game.players = players;
        game.deal_cards();

        game
    }

    pub fn deal_cards(&mut self) {
        self.state.deal_cards();
        self.state.set_random_dealer();

        for (i, player) in self.players.iter_mut().enumerate() {
            player.set_cards(self.state.cards(i));
        }
    }

    /// returns the winning team and the score of all cards in this trick
    pub fn play_trick(&mut self) {
        for i in self.state.turn()..(self.state.turn() + 4) {
            let player_idx = i % 4;
            let action = self.players[player_idx].decide(self.state.clone());

            println!("player {player_idx} plays {action:?}");

            match action {
                Action::PlayCard(card) => {
                    debug_assert!(self.is_legal(action));

                    self.state.apply_action(action);
                    self.players[player_idx].toggle_card(card.get_index());
                }
                _ => unreachable!(),
            }
        }
    }

    /// play an entire round, i.e. 8 tricks
    /// this method also assigns the next dealer
    pub fn play_round(&mut self) {
        let action = self.players[self.state.dealer()].decide(self.state.clone());
        println!("{} plays {action:?}", self.state.dealer());
        self.state.apply_action(action);
        //let trump = self.players[self.state.dealer()].pick_trump(self.state.clone());
        //self.state.apply_action(Action::PickTrump(trump));

        for _ in 0..8 {
            println!("{:?}", self.state);
            self.play_trick();
        }
    }

    /// controleer of deze speler al dan niet kan volgen
    pub fn is_legal(&self, action: Action) -> bool {
        self.legal_actions().has(&action)
    }

    pub fn legal_actions(&self) -> <GameState as State>::ActionList {
        self.state.possible_actions()
        //let mut cards = self.players[player].cards();
        //
        //// have to follow if possible,
        //if let Some(suite) = self.trick.suite_to_follow() {
        //    let filtered_cards = cards & suite.mask();
        //    if filtered_cards != 0 {
        //        cards = filtered_cards;
        //    }
        //}
        //
        //// this also means we're not the first player, i.e. the suite
        //// to follow has been determined
        //if let Some((winning_card, winning_player)) = self.trick.winner() {
        //    // our team is winning
        //    if winning_player % 2 == player % 2 {
        //        //todo!();
        //    } else {
        //        // have to buy if possible, but can't 'under-buy', except if that's our only possible move
        //        if let Some(trump) = self.trick.trump() {
        //            let mut mask = Stack::all_above(winning_card) & winning_card.suite().mask();
        //
        //            // we can play any trump if the current winning card isn't a trump
        //            if winning_card.suite() != trump {
        //                mask |= trump.mask();
        //            }
        //
        //            let filtered_cards = cards & mask;
        //            if filtered_cards != 0 {
        //                cards = filtered_cards;
        //            }
        //        }
        //        // this means that we're playing without trump,
        //        // so we simply need to play a higher card of the same suite
        //        else {
        //            let mask = Stack::all_above(winning_card) & winning_card.suite().mask();
        //            let filtered_cards = cards & mask;
        //
        //            if filtered_cards != 0 {
        //                cards = filtered_cards;
        //            }
        //        }
        //    }
        //}
        //
        //cards
    }

    pub fn is_terminal(&self) -> bool {
        self.state.is_terminal()
    }

    pub fn winner(&self) -> usize {
        self.state.winner()
    }

    pub fn state_ref(&self) -> &GameState {
        &self.state
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.state)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Game;
    use crate::{
        players::{random_player::RandomPlayer, Player, PlayerVec},
        stack::Stack,
    };

    #[test]
    fn test_dealing() {
        let players: PlayerVec = vec![
            RandomPlayer::boxed(),
            RandomPlayer::boxed(),
            RandomPlayer::boxed(),
            RandomPlayer::boxed(),
        ];

        let game = Game::new(players);
        let mut seen_cards = Stack::default();

        for player in &game.players {
            let cards = player.cards();
            seen_cards |= cards;

            assert!(cards.len() == 32 / (game.players.len() as u32));
        }

        assert!(seen_cards == Stack::ALL);
    }

    #[test]
    fn test_random_round() {
        let players: PlayerVec = vec![
            RandomPlayer::boxed(),
            RandomPlayer::boxed(),
            RandomPlayer::boxed(),
            RandomPlayer::boxed(),
        ];

        let mut game = Game::new(players);
        game.play_round();
    }
}
