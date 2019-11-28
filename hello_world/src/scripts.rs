use crate::console;

pub fn donut() {
//    read("press any key to exit...\n\n>>> ", false);
    let input = console::ask("do you like C#? [Y/N]: ", false);
    let normalized = input.trim();
    if normalized.eq_ignore_ascii_case("y") {
        println!("donut, go away, shoo, begone")
    } else {
        println!("glad to see you like python! welcome to the club!");
    }
}
