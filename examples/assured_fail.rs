use meticulous::OptionExt;

fn main() {
    None::<i32>.assured("bad reason");
}
