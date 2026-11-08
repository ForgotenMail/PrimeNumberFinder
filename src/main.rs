
fn main() {
    let mut num: u64;
    let mut prime: bool;
    num = 0;
    let mut primes: Vec<u64> = vec![];
    while num <= 1000000{
        prime = is_prime_number(num, &primes);
        if prime{println!("{} is prime!", num); primes.push(num);}
        num += 1;
    }
    println!("I found all the primes to a million :) {:#?}", primes);
}

fn is_prime_number(n: u64, p: &Vec<u64>) -> bool {

        if n <= 2 {
            return true;
        }
        else if n % 2 == 0 {
            return false;
        }

    let mut i: usize;

    i = 2;

    while p[i] * p[i] <= n {
        if n % p[i] == 0 {return false;}  
        i += 1
    }
    return true;

}
