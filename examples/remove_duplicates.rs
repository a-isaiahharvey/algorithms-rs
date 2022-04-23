use algorithms::strings::remove_duplicates;

fn main() {
    let value = remove_duplicates(
        "Peter Piper picked a peck of pickled peppers
            A peck of pickled peppers Peter Piper picked
            If Peter Piper picked a peck of pickled peppers
            Where’s the peck of pickled peppers Peter Piper picked?",
    );

    println!("{}", value);
}
