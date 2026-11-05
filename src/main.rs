fn main() {
    let mut num: u128;
    let mut prime: bool;
    num = 1;
    loop {
        prime = is_prime_number(num);
        println!("{} is a prime number? {}", num, prime);
        num += 1;
    }
}

fn is_prime_number(n: u128) -> bool {
    match {
        n <= 1 => return true,

        n % 2 == 0 => return false,

        n % n == 0 => return false,
    }

    let mut i: u128;

    i = 3;

    while i * i <= n {
        n % i == 0 => return false;

    }
    
    return true;

}
