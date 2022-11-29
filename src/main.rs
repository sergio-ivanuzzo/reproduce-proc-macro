use my_proc_macros_lib::Debugger;

#[derive(Debugger)]
struct Test {
    #[dynamic_field]
    size: u8,
    #[dynamic_field]
    size1: u8,
    #[dynamic_field]
    size2: u8,
    field2: u8,
    field3: u8,
}

impl Test {
    fn size() {}
    fn size1() {}
    fn size2() {}
}

fn main() {
    Test::debug();
}
