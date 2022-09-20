mod field;
mod field_element_2625;

use field::FieldElement;

pub fn ed25519_pk_to_curve25519(pk: [u8; 32]) -> [u8; 32] {
    let AY = FieldElement::from_bytes(&pk);

    let mut one_minus_y = FieldElement::one();

    one_minus_y = &one_minus_y - &AY;

    one_minus_y = one_minus_y.invert();

    let mut x = FieldElement::one();

    x = &x + &AY;

    x = &x * &one_minus_y;

    x.to_bytes()
}
