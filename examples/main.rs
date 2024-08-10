use damage_ranges::fract::Fract;
use damage_ranges::{fract_literal, posCord, DamageRange};

fn main() {
    let mut range = DamageRange::new(32);

    range.add_single(posCord {
        range: fract_literal!(2 _ 1),
        damage: fract_literal!(2 _ 1),
    });

    println!("{:#?}", range);
}
