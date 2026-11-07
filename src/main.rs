// Testing Git out
fn main() {
    let mut num: u128;
    let mut prime: bool;
<<<<<<< HEAD
    num = 1;
    while num <= 100000000{
        prime = is_prime_number(num);
        if prime {println!("{} is prime!", num)}
=======
    num = 100000000;
    let mut primes: Vec<u128> = Vec::new();
    while num <= 1000000000{
        prime = is_prime_number(num, primes);
        if prime {println!("{} Is prime!", num); primes.push(num);}
>>>>>>> 9c6566e (Another thing)
        num += 1;
    }
}

<<<<<<< HEAD
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
=======
fn is_prime_number(n: u128, p: Vec<u128>) -> bool {
    if n <= 2 {return true;}

    else if n % 2 == 0 {return false;}

    else if n.isqrt() == 0 {return false;}
>>>>>>> 9c6566e (Another thing)

    let mut i: u128;
    let mut divisor = 0.0000001;

    i = 0;

<<<<<<< HEAD
    while i <= 11 {
        if n % i == 0 {
            divisor = (n % i) as f64;
            return false;
        }  
=======
    while p[i] * p[i] <= n {
        if n % p[i] == 0 { return false; }
        i += 1
>>>>>>> 9c6566e (Another thing)

    println!("i:{} remainder: {}", i, divisor);
    i += 2
    }
    return true;

}
