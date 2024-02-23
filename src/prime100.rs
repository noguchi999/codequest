fn is_prime(n: usize) -> bool {
    for i in 2..n {
        if n%i == 0 {
            return false;
        }
    }
    return true;
}

fn get_primes(prime: &mut[usize; 100]) {
    let mut i = 2;
    let mut count = 0;
    while count < 100 {
        if is_prime(i) {
            prime[count] = i;
            count += 1;
        }
        i += 1;
    }
}

pub fn main() {
    let mut prime = [0; 100];
    get_primes(&mut prime);
    println!("{:?}", prime);
}