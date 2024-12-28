enum Option <T> {
    Some(T),
    None,
}

fn even_number(number: &u8) -> Option<&u8> {
    if number % 2 == 0 {
        Option::Some(number)
    }
    else {
        Option::None
    }
}
fn main() {
    let x = 8;

    match even_number(&x){
        Option::Some(&n) => println!("The number {} is an even number", &n),
        Option::None => println!("No, the number is not an even number"),
    }

    let y = 9;

    match even_number(&y) {
        Option::Some(&n) => println!("The number {} is an even number", &n),
        Option::None => println!("The number {} is an odd number", &y),
    }


}