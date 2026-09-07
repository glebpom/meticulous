use meticulous::ResultExt;

fn main() {
    let _: u8 = Ok::<u8, &str>(1).todo();
}
