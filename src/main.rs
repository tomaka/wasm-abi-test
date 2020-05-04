
#[repr(C)]
struct Vector {
    a: i32,
    b: i32,
}

extern "C" {
    fn extract_a(v: Vector) -> i32;
}

fn main() {
    unsafe {
        extract_a(Vector { a: 5, b: 4 });
    }
}
