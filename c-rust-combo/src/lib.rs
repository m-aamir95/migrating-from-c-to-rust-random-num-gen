
static mut X : u64 = 123456789;

#[no_mangle] //no_mangle prevents the compiler from randomizing function names
             //This is required otherwise C code wont be able to resolve functions
pub extern "C" fn seed_random_int(seed : u64) {
  
    unsafe{
        X = seed;
    }
}

#[no_mangle] 
pub extern "C" fn get_random_int() -> u64{

    unsafe {
        X = X.wrapping_mul(69069).wrapping_add(362437);
        X
    }
}