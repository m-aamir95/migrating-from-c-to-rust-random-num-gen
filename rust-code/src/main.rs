

static mut X :  u64 = 123456789;


pub fn vseed(seed : u64){

    //Have to write this kind of code explicitly in the unsafe block 
    //Because by default RUST won't allow mutating a static variable
    //As this can be potentially modified by multiple threads at the same time
    //Therefore, this kind of explicit unsafe block
    unsafe{
        X = seed;
    }
}

pub fn vrandom() -> u64{

    //Have to write this kind of code explicitly in the unsafe block 
    //Because by default RUST won't allow mutating a static variable
    //As this can be potentially modified by multiple threads at the same time
    //Therefore, this kind of explicit unsafe block
    unsafe{

        //By default RUST will panick because a buffer overflow is expected
        //In C by default things overlfow and wrap around, but not in RUST
        //This boy will try to stop as much as possible self sabotage 
        //X = X * 69069 + 362437;

        //Hence, we gotta do something like below, this allows overflow and wrap around behavior
        //Just like in C. And especially because its a pseudo random number generator we would def
        //Like this kind of behavior
        X = X.wrapping_mul(69069).wrapping_add(362437);
        X
    }
}


fn main() {

    for _ in 1..10{
        println!("{}", vrandom());
    }
}
