// Testing Git out
fn main() {
    let mut num: u128;
    let mut prime: bool;
    num = 1;
    while num <= 100000000{
        prime = is_prime_number(num);
        if prime {println!("{} is prime!", num)}
        num += 1;
    }
}

fn is_prime_number(n: u128) -> bool {

        if n <= 1 {
            return false;
        }
        if n % 2 == 0 {
            return false;
        }
        let nf = n as f64;
        if n.sqrt().fract() == 0 {
            return false;
        }

    let mut i: u128;
    let mut divisor = 0.0000001;

    i = 3;

    while i <= 11 {
        if n % i == 0 {
            divisor = (n % i) as f64;
            return false;
        }  

    println!("i:{} remainder: {}", i, divisor);
    i += 2
    }
    return true;

}
