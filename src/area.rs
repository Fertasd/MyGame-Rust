use std::cell::RefCell;
use std::rc::Rc;

pub struct Position {
	x: i64,
	y: i64,
	z: i64
}

impl Position {
	pub fn new(x: i64, y: i64, z: i64) -> Position {
		Position { x, y, z }
	}
}

trait Place {}

trait Location {
	fn location(&self) -> Place;
}

pub struct Dungeon {
	levels: Vec<Rc<Level>>
}

impl Dungeon {
	pub fn new() -> Dungeon {
		Dungeon { levels: Vec::new() }
	}
	pub fn add_level(&mut self, level: Rc<Level>) {
		self.levels.push(level);
	}
}

pub struct Level {
	rooms: Vec<Rc<Room>>
}

impl Level {
	pub fn new() -> Level {
		Level { rooms: Vec::new() }
	}
	pub fn add_room(&mut self, room: Rc<Room>) {
		self.rooms.push(room);
	}
}

impl Place for Level {
}

pub struct Room {
}

impl Place for Room {
}
