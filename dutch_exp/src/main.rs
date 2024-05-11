fn main() {
    let start_date = 5;
    let end_date = 15;

    let a = 20;
    let b = 30;

    let start_price = a * start_date + b;
    let reserve_price = a * end_date + b;

    println!("start price: {}", start_price);
    println!("reserve_price: {}", reserve_price);
}
