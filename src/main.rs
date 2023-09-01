mod routing;
mod nat_v4;

mod bit_utils;
fn main() {
    println!("Hello, world!");
    routing::check_routing();
    nat_v4::test_translation_incoming();
    nat_v4::test_translation_outgoing();
}
