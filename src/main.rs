let mut num = 5;

let r1 = &num as *count i32;
let r2 =&mut num as *mut i32;

unsafe {
    println!("r1 is {}", *r1);
    println!("r2 is {}", *r2)
}