
#[repr(C)]
struct Vector {
    a: f32,
    b: f32,
}

extern "C" {
    fn extract_a(v: Vector) -> f32;
}

fn main() {
    let v = unsafe {
        extract_a(Vector { a: 5.0, b: 4.0 });
    };

    println!("{:?}", v);
}
