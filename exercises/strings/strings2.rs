// strings2.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    //Allocate memory on the heap for the string "green".
    //This will give you a pointer to the string "green" on the heap.
    let word = String::from("green"); // Try not changing this line :)

    //Pass a reference to the string pointer.
    //I need to pass a reference because I want main to keep ownership of the string.
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &String) -> bool {
    //Here I have a reference to a string pointer on the heap.

    //Deref coercion will happen here, so that you have a string value
    //instead of a reference to a String. Meaning you can compare the
    //value to a string literal directly, using the PartialEq trait.

    //String implements PartialEq, so you can compare a string value to a string literal.
    attempt == "green" || attempt == "blue" || attempt == "red"
}
