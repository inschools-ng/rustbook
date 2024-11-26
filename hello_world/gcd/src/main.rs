fn gcd(mut n: u64, mut m: u64) -> u64 {
    // mut allows th e function to assign to the variables
    assert!(n != 0 && m != 0); // assert verifies that neither variables are zero by checking if true or not
    while m != 0 {
        if m < n {
            let t: u64 = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test] // this marks test_gcd as a test function - attribute
fn test_gcd() {
    // define a function named test_gcd
    assert_eq!(gcd(14, 15), 1); // checks that it returns the correct values

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

fn main() {
    println!("Hello, world!!");
}
