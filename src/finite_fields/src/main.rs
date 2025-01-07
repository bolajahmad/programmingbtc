use finite_fields::FieldElement;
use finite_fields::helpers::is_prime;
use rug::Integer;
use rug::ops::Pow;

fn main() {
    let field_a = FieldElement::new(
        Integer::from(5), 
        Integer::from(31)
    );
    // println!("Multiplication of terms, {}", 3 * field_a);
    let prime = Integer::from(2).pow(256) - Integer::from(2).pow(32) - Integer::from(977);
    println!("The calculated prime for secp256 is {}", prime);

    println!("It is a prime, {}", is_prime(prime));
}
