fn main() {
    let mut number = 1;

    loop {
        if number > 10 {
            break;
        }
        println!("{}", number);
        number += 1;
    }
}
