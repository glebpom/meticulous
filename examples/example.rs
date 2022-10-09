use meticulous::ResultExt;
use rand::random;

fn main() {
    i32::try_from(0i64).assured("never fails because 0 is a valid i32");
    let i: i64 = random();
    if (i32::MIN as i64..=i32::MAX as i64).contains(&i) {
        i32::try_from(0i64).verified("boundaries were checked");
    }
    i32::try_from(random::<i64>()).todo();
}
