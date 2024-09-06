pub mod front_house {
    pub mod housing {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path: the reason crate::paths::front_house is known to the module resolvser is due to the "use" statement in paths.rs
    crate::paths::front_house::housing::add_to_waitlist();

    // Relative path: here front_house is known to the module resolver due to its location in the same file
    front_house::housing::add_to_waitlist();
}