use add::add;

fn main() {
    let x = 1;
    let y = 5;
    let z = add(&x, &y);
    println!("The addition of {} and {} is {}", x, y, z);
}