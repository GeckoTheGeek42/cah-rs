use card::{Hand, Deck, Card, WhiteCard, BlackCard};
use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
use rustc_serialize::json::{encode, decode};

pub mod state;
pub mod input;

use self::state::*;

#[derive(Debug)]
pub struct GameLoop {
	state: GameState,

}