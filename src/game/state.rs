use card::{Hand, Deck, Card, WhiteCard, BlackCard};
use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
use rustc_serialize::json::{encode, decode};

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct BWDeck {
	black: Deck<BlackCard>,
	white: Deck<WhiteCard>
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Player {
	name: String,
	hand: Hand
}

#[derive(Debug)]
pub struct ActiveState {
	black: BlackCard,
	answers: Vec<WhiteCard>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct StaticState {
	players: Vec<Player>,
	deck: BWDeck,
}

#[derive(Debug)]
pub struct GameState {
	static_state: StaticState,
	active_state: ActiveState
}