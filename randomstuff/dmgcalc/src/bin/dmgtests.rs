use num_bigint::{BigInt, BigUint};
use dmgcalc::{ApplicationMode, Attribute, AttributeCollection, DamageType};
use dmgcalc::DamageType::Piercing;


fn main() {
    let mut attrs = AttributeCollection::from(&[
        Attribute::weakness(DamageType::Fire, ApplicationMode::FLAT, 20)
    ]);
    println!("{}", Attribute::reduction(DamageType::Fire, ApplicationMode::PERCENT, 20).apply(150, DamageType::Fire));
}
