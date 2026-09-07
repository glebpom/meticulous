use meticulous::{OptionExt, ResultExt};

fn main() {
    let _: u8 = Ok::<u8, &str>(1).assured("the value is present");
    let _: u8 = Ok::<u8, &str>(1).verified("the value was checked");
    let _: u8 = Some(1).assured("the value is present");
    let _: u8 = Some(1).verified("the value was checked");
}
