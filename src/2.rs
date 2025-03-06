fn main() {
    let mut num = 0;
    loop {
        println!("{}", num);
        num += 1;
        if num == 10 {
            break;
        }
    }
}
