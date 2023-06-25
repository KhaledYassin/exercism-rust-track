fn main() {
    let mut number = 121;
    let mut reverse = 0;

    while number > 0 {
        let dig = number % 10;
        reverse = reverse * 10 + dig;
        number  /= 10;
    }
    println!("{:?}", reverse)
} 