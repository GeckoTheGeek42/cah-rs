use std::io::{Lines, Stdin, StdinLock, stdin, BufRead};
use std::io::Error as IoErr;
use std::str::FromStr;
use std::error::{Error, self};
use std::fmt::{Display, Formatter, Debug};
use std::fmt::Error as FmtErr;
use std::marker::Reflect;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum StrReadError<T> {
	Other(T), 
	//Custom(E)
	IO(IoErr)
}

pub struct ReadLoop {
	stdin: Stdin, 
	buf: String 
}

impl ReadLoop {
	fn new() -> ReadLoop {
		ReadLoop {
			stdin: stdin(),
			buf: String::new(),
		}
	}

	fn stdin(&mut self) -> &mut Stdin {
		&mut self.stdin
	}

	fn read_line(&mut self) -> Result<&str, IoErr> {
		try!(self.stdin.read_line(&mut self.buf));
		Ok(&self.buf)
	}

	fn next_line<T: FromStr>(&mut self) -> Result<T, StrReadError<T::Err>> {
		self.read_line()
			.map_err(|e| StrReadError::IO(e))
			.and_then(|line| FromStr::from_str(line)
				.map_err(|e| StrReadError::Other(e))
			)
	}
}

#[derive(Debug)]
pub enum Input {
	Choose(usize),
	Pick,
	Decide(usize)
}

#[derive(Debug)]
pub enum InputParseError {
	MissingCommand, InvalidCommand, MissingArgument, InvalidArgument
}

impl From<ParseIntError> for InputParseError {
	fn from(e: ParseIntError) -> InputParseError {
		InputParseError::InvalidArgument
	}
}

impl FromStr for Input {
	type Err = InputParseError;

	fn from_str(s: &str) -> Result<Input, InputParseError> {
		let mut tokens = s.split(' ');
		
		match tokens.next() {
			Some(command) => match command {
				"Pick" | "pick" => return Ok(Input::Pick),
				"Choose" | "choose" => return Ok(Input::Choose({
					let argstr = match tokens.next() {
						Some(arg) => arg,
						None => return Err(InputParseError::MissingArgument),
					};
					try!(argstr.parse())
				})),
				"Decide" | "decide" => return Ok(Input::Decide({
					let argstr = match tokens.next() {
						Some(arg) => arg,
						None => return Err(InputParseError::MissingArgument),
					};
					try!(argstr.parse())
				})),
				_ => return Err(InputParseError::InvalidCommand)
			},
			None => return Err(InputParseError::MissingCommand),
		};
	}
}