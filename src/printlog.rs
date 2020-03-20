#[cfg(debug_assertions)]
macro_rules! printlog {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

#[cfg(not(debug_assertions))]
macro_rules! printlog {
    ($( $args:expr ),*) => {};
}
