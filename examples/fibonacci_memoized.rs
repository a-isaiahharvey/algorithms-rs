use algorithms::math::fibonacci::memoized;

fn main() {
    let value = memoized(2);
    println!("{}", value);
}
