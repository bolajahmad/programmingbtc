use rug::Integer;
use scripts::{helpers::Stack, Script};

mod helpers;


fn use_lifetime<'a>(text: &'a str, text_2: &'a str) -> &'a str {
    text
}

struct Test<'a> {
    text: &'a str,
}

fn main() {
    // let command = "6a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937";
    // let script = Script::parse(command).unwrap();
}
