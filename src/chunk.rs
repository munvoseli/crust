use std::fs::File;
use std::io::{Read, Write};

use crate::player::Player;

pub struct Chunk {
	x: i32,
	y: i32,
	tiles: [u8; 128 * 128],
}
impl Chunk {
	fn new_empty(x: i32, y: i32) -> Self {
		Self {
			x: x, y: y,
			tiles: [1; 128 * 128]
		}
	}
	// load chunk from file, generate if necessary
	fn new_maybe_file(x: i32, y: i32) -> Self {
		let f = File::open(format!("save/{}_{}.dat", x, y));
		let mut chunk = Self::new_empty(x, y);
		if let Ok(mut file) = f {
			let mut buf: Vec<u8> = Vec::new();
//			let mut buf: [u8; 9 + 128 * 128] = [1; 128*128+9];
			file.read_to_end(&mut buf).unwrap();
			match buf[0] {
			0 => {
				for i in 0..128*128 {
					chunk.tiles[i] = buf[i + 9];
				}
			},
			1 => {
				let mut ib: usize = 9;
				let mut ic: usize = 0;
				loop {
					if ic == 128 * 128 { break; }
					if ic > 128 * 128 { println!("chunk parse went wrong"); break; }
					let ice = ic + buf[ib] as usize;
					ib += 1;
					for i in ic..ice {
						chunk.tiles[i] = buf[ib];
					}
					ib += 1;
					ic = ice;
				}
			},
			_ => {
				println!("unknown chunk format");
			}
			}
		}
		chunk
	}
	fn new_from_filename(filename: String) -> Self {
		let x: i32 = 0;
		let y: i32 = 0;
		let mut chunk = Self::new_empty(x, y);
		if let Ok(mut file) = File::open(filename.to_string()) {
			let mut buf: Vec<u8> = Vec::new();
//			let mut buf: [u8; 9 + 128 * 128] = [1; 128*128+9];
			file.read_to_end(&mut buf).unwrap();
			let x = ((((((buf[1] as i32) << 8) | buf[2] as i32) << 8) | buf[3] as i32) << 8) | buf[4] as i32;
			let y = ((((((buf[5] as i32) << 8) | buf[6] as i32) << 8) | buf[7] as i32) << 8) | buf[8] as i32;
			chunk = Self::new_empty(x, y);
			println!("reading chunk format {}", buf[0]);
			match buf[0] {
			0 => {
				for i in 0..128*128 {
					chunk.tiles[i] = buf[i + 9];
//					if chunk.tiles[i] >= 0x95 {
//						let u = x + (i as i32 & 0x7f);
//						let v = y + (i as i32 / 0x7f);
//						if u.abs() + v.abs() > 8 {
//							println!("Found rest area at {} {}", u, v);
//						}
//					}
				}
			},
			1 => {
				let mut ib: usize = 9;
				let mut ic: usize = 0;
				loop {
					if ic == 128 * 128 { break; }
					if ic > 128 * 128 { println!("chunk parse went wrong"); break; }
					let ice = ic + buf[ib] as usize;
					ib += 1;
					for i in ic..ice {
						chunk.tiles[i] = buf[ib];
					}
					ib += 1;
					ic = ice;
				}
			},
			_ => {
				println!("unknown chunk format");
			}
			}
		}
		chunk
	}
	fn save(&self) {
//		let mut buf: [u8; 9 + 128 * 128] = [0; 128*128+9];
//		buf[1] = ((self.x >> 24) & 255) as u8; // store x and y
//		buf[2] = ((self.x >> 16) & 255) as u8; // big endian
//		buf[3] = ((self.x >>  8) & 255) as u8;
//		buf[4] = ((self.x >>  0) & 255) as u8;
//		buf[5] = ((self.y >> 24) & 255) as u8;
//		buf[6] = ((self.y >> 16) & 255) as u8;
//		buf[7] = ((self.y >>  8) & 255) as u8;
//		buf[8] = ((self.y >>  0) & 255) as u8;
//		for i in 0..128*128 {
//			buf[i + 9] = self.tiles[i];
//		}
//		let mut f = File::create(format!("{}_{}.dat", self.x, self.y)).unwrap();
//		f.write(&buf).unwrap();
		let mut buf: Vec<u8> = vec!(
			1,
			((self.x >> 24) & 255) as u8, // store x and y
			((self.x >> 16) & 255) as u8, // big endian
			((self.x >>  8) & 255) as u8,
			((self.x >>  0) & 255) as u8,
			((self.y >> 24) & 255) as u8,
			((self.y >> 16) & 255) as u8,
			((self.y >>  8) & 255) as u8,
			((self.y >>  0) & 255) as u8
		);
		let mut ic: usize = 0;
		loop {
			if ic == 128*128 { break; }
			let ics = ic;
			let t = self.tiles[ics];
			loop {
				ic += 1;
				if ic == 128*128 || ic == 255 + ics || self.tiles[ic] != t { break; }
			}
			let len = (ic - ics) as u8;
			buf.push(len);
			buf.push(t);
		}
		let mut f = File::create(format!("save/{}_{}.dat", self.x, self.y)).unwrap();
		f.write(&buf).unwrap();
	}
	pub fn get_tile_at_off(&mut self, x: i32, y: i32) -> u8 {
		let i = (x | (y << 7)) as usize;
		self.tiles[i]
	}
}

pub struct WorldTiles {
	pub chunks: Vec<Chunk>
}

impl WorldTiles {
	fn loaded_chunk_at(&self, x: i32, y: i32) -> bool {
		for chunk in &self.chunks {
			if chunk.x == x && chunk.y == y {
				return true;
			}
		}
		return false;
	}
	pub fn get_chunk_id_at(&mut self, x: i32, y: i32) -> usize {
		for i in 0..self.chunks.len() {
			if self.chunks[i].x == x && self.chunks[i].y == y {
				return i;
			}
		}
		self.chunks.push(Chunk::new_maybe_file(x, y));
		self.chunks.len() - 1
	}
	pub fn get_tile_at(&mut self, x: i32, y: i32) -> u8 {
		let cx = x & -128i32;
		let cy = y & -128i32;
		let i = self.get_chunk_id_at(cx, cy);
		let chunk = &self.chunks[i];
		let i = (x & 127) | ((y & 127) << 7);
		chunk.tiles[i as usize]
//		for chunk in &self.chunks {
//			if chunk.x == cx && chunk.y == cy {
//				let i = (x & 0x7F) | ((y & 0x7F) << 7);
//				return chunk.tiles[i as usize];
//			}
//		}
//		1
	}
	pub fn set_tile_at(&mut self, x: i32, y: i32, tile: u8) {
		let i = self.get_chunk_id_at(x & -128i32, y & -128i32);
		let mut chunk = &mut self.chunks[i];
		let i = (x & 127) | ((y & 127) << 7);
		chunk.tiles[i as usize] = tile;
	}
	pub fn new() -> Self {
		Self { chunks: Vec::new() }
	}
	pub fn save_all(&self) {
		for chunk in &self.chunks {
			chunk.save();
		}
	}
	pub fn unload_unused(&mut self, players: &Vec<Player>) {
		let mut i: usize = 0;
		loop {
			if i >= self.chunks.len() {
				break;
			}
			let mut out_of_range = true;
			for player in players {
				let xd = self.chunks[i].x + 64 - player.x;
				let yd = self.chunks[i].y + 64 - player.y;
				if xd.abs() < 400 && yd.abs() < 400 {
					out_of_range = false;
					break;
				}
			}
			if out_of_range {
				self.chunks[i].save();
				self.chunks.swap_remove(i);
			} else {
				i = i + 1;
			}
		}
	}
	pub fn load_all_file(&mut self) {
		for entry in std::fs::read_dir("./").unwrap() {
			let entry = entry.unwrap();
			let path = entry.path();
			let pathh = path.file_name();
			if let Some(pathh) = pathh {
				if let Some(pathhh) = pathh.to_str() {
					let pstr = pathhh.to_string();
					if pstr.len() > 4 {
						let valid = (&pstr[pstr.len() - 4..pstr.len()]).eq(".dat");
						if valid {
							self.chunks.push(Chunk::new_from_filename(pstr));
						}
					}
				}
			}
		}
	}
}
