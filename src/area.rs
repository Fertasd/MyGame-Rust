use rand;
use std::collections::HashMap;
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

#[derive(Clone, Copy)]
enum Direction {
	UP,
	DOWN,
	LEFT,
	RIGHT
}

impl Direction {
	fn random() -> Self {
		match rand::random::<u8>() % 4 {
			0 => Direction::UP,
			1 => Direction::DOWN,
			2 => Direction::LEFT,
			3 => Direction::RIGHT,
			_ => unreachable!()
		}
	}
}


impl Level {
	pub fn new() -> Level {
		Level { rooms: Vec::new() }
	}
	pub fn add_room(&mut self, room: Rc<Room>) {
		self.rooms.push(room);
	}
	fn step(dir: Direction, pos: usize, len: usize) -> usize {
		match dir {
			Direction::LEFT => pos - 1,
			Direction::RIGHT => pos + 1,
			Direction::DOWN => pos - len,
			Direction::UP => pos + len
		}
	}
	fn walker(pos: usize, steps: u8, len: usize, map: &mut Vec<bool>, speed: u8, gen: &mut HashMap<usize, bool>, counter: &mut usize, num: &mut usize) {
		let mut pos = pos;
		for i in 0..steps {
			let dir = Direction::random();
			for h in 0..speed {
				let a = Level::step(dir, pos, len);
				match map[a] {
					true => pos = a,
					false => {
						pos = a;
						map[a] = true;
						*counter += 1;
					}
				}
				if counter >= num { break; }
			}
			if counter >= num { break; }
		}
		gen.insert(pos, true);
	}
	fn generate_rooms(num: usize) {
		let mut fullmap = Vec::new();
		for i in 0..(2 * num) ^ 2 {
			fullmap.push(false);
		}
		let mut counter = 0;
		let mut num = num;
		let mut currentgen = HashMap::new();
		let mut nextgen = HashMap::new();
		fullmap[2 * num ^ 2] = true;
		counter += 1;
		currentgen.insert(2 * num ^ 2, fullmap[2 * num ^ 2]);
		while counter < num {
			for (key, value) in currentgen {
				Level::walker(key, 3, 2 * num, &mut fullmap, rand::random::<u8>() % 4, &mut nextgen, &mut counter, &mut num);
				Level::walker(key, 3, 2 * num, &mut fullmap, rand::random::<u8>() % 4, &mut nextgen, &mut counter, &mut num);
				if counter >= num { break; };
			}
			currentgen = nextgen;
			nextgen = HashMap::new();
		}
		for (index, value) in fullmap.iter().enumerate() {
			if *value {
				let newroom = Room::new();
				Room::set_position(newroom, index % (2 * num), (index - index % (2 * num)) / (2 * num));
			}
		}
	}
}

impl Place for Level {}

pub struct Room {}

impl Room {
	pub fn new() -> Room {
		Room {}
	}
	fn set_position(room: Room, x: usize, y: usize) {}
}

impl Place for Room {}
