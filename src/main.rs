fn main() {
    let mut numbers = Vec::new();
    for num in 1..1000 {
        if num % 3 == 0 {
            numbers.push(num)
        } else if num % 5 == 0 {
            numbers.push(num)
        }
    }
    let total: u32 = numbers.iter().sum();
    println!("Sum of numbers is {}", total);
}
