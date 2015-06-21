use std::fmt::{Display, Debug, self};
use std::default::Default;
use rand::{Rng, self};
use std::ops::{Index, IndexMut};
use std::mem;

use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};

const HAND_SIZE: usize = 7;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Deck<C: Card>(Vec<C>);

impl<T: Card> Default for Deck<T> {
	fn default() -> Deck<T> { Deck(Vec::new()) }
}

impl<C: Card> Deck<C> {
	fn pick(&mut self) -> C {
		let i = rand::thread_rng().gen_range(0, self.0.len());
		self.0.swap_remove(i)
	}

	fn pop(&mut self) -> Option<C> {
		self.0.pop()
	}
}

#[derive(Debug, RustcEncodable)]
pub struct Hand([WhiteCard; HAND_SIZE], Vec<BlackCard>);

impl Hand {
	fn choose(&mut self, i: usize, mut new: WhiteCard) -> WhiteCard {
		let mut chosen = unsafe { mem::uninitialized() };
		mem::swap(&mut chosen, &mut self[i]);
		mem::swap(&mut self[i], &mut new);
		chosen
	}

	fn points(&self) -> u8 { self.1.len() as u8 }
}

impl Index<usize> for Hand {
	type Output = WhiteCard;

	fn index(&self, i: usize) -> &WhiteCard {
		&self.0[i]
	}
}
impl IndexMut<usize> for Hand {
	fn index_mut(&mut self, i: usize) -> &mut WhiteCard {
		&mut self.0[i]
	}
}

impl Decodable for Hand {
	fn decode<D: Decoder>(d: &mut D) -> Result<Hand, D::Error> {
		d.read_seq(|d, len| {
            assert!(len > HAND_SIZE);
            let mut arr: [WhiteCard; HAND_SIZE] = unsafe { mem::uninitialized() };
            for (i, slot) in arr.iter_mut().enumerate() {
            	*slot = try!(d.read_seq_elt(i, |d| Decodable::decode(d)));
            }
            Ok(Hand(arr, Vec::new()))
        })
	}
} 

pub trait Card: Encodable + Decodable + Debug + Display {
	fn text(&self) -> &String;
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct WhiteCard(String);

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct BlackCard(String);
	
impl<S: Into<String>> From<S> for WhiteCard {
	fn from(s: S) -> WhiteCard {
		WhiteCard(s.into())
	}
}
impl<S: Into<String>> From<S> for BlackCard {
	fn from(s: S) -> BlackCard {
		BlackCard(s.into())
	}
}

impl Card for WhiteCard {
	fn text(&self) -> &String {
		&self.0
	}
}
impl Card for BlackCard {
	fn text(&self) -> &String {
		&self.0
	}
}

impl Display for WhiteCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "White: {}", self.0)
	}
}
impl Display for BlackCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "Black: {}", self.0)
	}
}
impl<C: Card> Display for Deck<C> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		try!(write!(f, "Deck: {} cards", self.0.len()));
		for (i, card) in self.0.iter().enumerate() {
			try!(write!(f, "{i}) {card}", card=card, i=i))
		}
		Ok(())
	}
}
impl Display for Hand {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		try!(write!(f, "Hand: {} cards, {} points", HAND_SIZE, self.1.len()));
		let Hand(ref cards, ref points) = *self;
		for (i, card) in cards.iter().enumerate() {
			try!(write!(f, "{i}) {card}", card=card, i=i));
		}
		for (i, card) in points.iter().enumerate() {
			try!(write!(f, "{i}) {card}", card=card, i=i));
		}
		Ok(())
	}
}