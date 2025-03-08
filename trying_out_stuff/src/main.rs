fn main() {
	#[derive(Debug)]
	struct Cuboid {
		width: u32,
		height: u32,
		depth: u32
	}
	impl Cuboid {
		fn area(&self) -> u32 {
			self.width * self.height * self.depth
		}
		fn new(width: u32, height: u32, depth: u32) -> Cuboid {
			Cuboid { width, height, depth }
		}
	}
	let cub = Cuboid::new(10, 10, 10);
	println!("{}", cub.area());
	println!("cub has the following state: {:#?}", cub)
}