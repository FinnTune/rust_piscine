fn main () {
    let x = 42;
    let mut y = 13;  // Declare y as mutable

    // Creating raw pointers
    let x_ptr: *const i32 = &x;
    let y_ptr: *mut i32 = &mut y;  // No need for two 'as' statements now

    unsafe {
        println!("x: {}", *x_ptr); // Dereferencing immutable raw pointer
        *y_ptr = 20; // Dereferencing and modifying mutable raw pointer
        println!("y: {}", *y_ptr);
    }
}