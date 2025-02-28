use std::io::stdin;

use bench::bench;
use game::Game;
use players::{random_player::RandomPlayer, Player, PlayerVec};

mod action;
mod action_list;
mod array;
mod bench;
mod bits;
mod card;
mod game;
mod game_phase;
mod game_state;
mod mcts;
mod players;
mod stack;
mod suite;
mod trick;

fn main() {
    romu::seed();

    let args: Vec<String> = std::env::args().collect();

    if args.contains(&"bench".to_owned()) {
        let size = args.last().and_then(|x| x.parse::<usize>().ok());
        bench(size);
    }

    if args.contains(&"d".to_owned()) {
        let players: PlayerVec = vec![
            RandomPlayer::boxed(),
            RandomPlayer::boxed(),
            RandomPlayer::boxed(),
            RandomPlayer::boxed(),
        ];
        let mut game = Game::new(players);
        while !game.is_terminal() {
            let mut buf = String::new();
            stdin().read_line(&mut buf).unwrap();
            println!("{buf}");

            match buf.trim() {
                "r" => game.play_round(),
                "p" => println!("{game:?}"),
                _ => (),
            }
        }
    }
}
