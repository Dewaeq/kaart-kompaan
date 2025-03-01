use crate::{
    action::Action,
    game_state::GameState,
    mcts::{action_list::ActionList, state::State},
};

use super::Player;

#[derive(Default)]
pub struct RandomPlayer {
    index: usize,
}

impl Player for RandomPlayer {
    fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    fn decide(&mut self, state: GameState) -> Action {
        state.possible_actions().pop_random().unwrap()

        //let mut cards = self.cards;
        //
        //// have to follow if possible,
        //if let Some(suite) = game.trick.suite_to_follow().filter(|&s| cards.has_suite(s)) {
        //    let filtered_cards = cards & suite.mask();
        //    if filtered_cards != 0 {
        //        cards = filtered_cards;
        //    }
        //}
        //
        //// this also means we're not the first player, i.e. the suite
        //// to follow has been determined
        //if let Some((winning_card, winning_player)) = game.trick.winner() {
        //    // our team is winning
        //    if winning_player % 2 == self.index % 2 {
        //        //todo!();
        //    } else {
        //        // have to buy if possible, but can't 'under-buy', except if that's our only possible move
        //        if let Some(trump) = game.trick.trump() {
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
        //        // so we simply need to played a higher card of the same suite
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
        //loop {
        //    let card = cards.pick_random_card();
        //
        //    if game.is_legal(card, self.index) {
        //        return card;
        //    }
        //}
    }

    //fn pick_trump(&self, _state: GameState) -> Option<Suite> {
    //    Some(self.cards.pick_random_suite())
    //}
}
