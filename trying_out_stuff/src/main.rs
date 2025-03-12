fn main() {
	enum Shape {
		Cuboid{length: u8, width: u8, height: u8},
		Sphere(f64)
	}
	impl Shape {
		fn hello(){println!("hello");}
		fn area(&self) {println!("I don't know what I'm doing!")}
	}
	let s1 = Shape::Sphere(3.14f64);
	let b1 = Shape::Cuboid{length: 3, width: 3, height: 3};
	Shape::hello();
	s1.area();
}