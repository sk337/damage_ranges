pub mod fract;
pub use fract::Fract;
mod math;

#[macro_export]
macro_rules! fract_literal {
    ($num:literal _ $den:literal) => {
        Fract::new($num, $den)
    };
}

#[derive(Debug)]
pub struct posCord {
    pub range: Fract,
    pub damage: Fract,
}

#[derive(Debug)]
pub struct DamageTri {
    pub control_point: posCord,
    pub end_point: posCord,
}

#[derive(Debug)]
pub struct DamageQuad {
    pub control_point: posCord,
    pub control_point2: posCord,
    pub end_point: posCord,
}

#[derive(Debug)]
pub enum RangePoint {
    Single(posCord),
    Tri(DamageTri),
    Quad(DamageQuad),
}

#[derive(Debug)]
pub struct DamageRange {
    start_point: posCord,
    ranges: Vec<RangePoint>,
}

impl DamageRange {
    pub fn new(start_damage: u32) -> DamageRange {
        DamageRange {
            start_point: posCord {
                range: 0.try_into().unwrap(),
                damage: start_damage.try_into().unwrap(),
            },
            ranges: Vec::new(),
        }
    }

    pub fn add_single(&mut self, range: posCord) {
        self.ranges.push(RangePoint::Single(range));
    }

    pub fn add_tri(&mut self, range: DamageTri) {
        self.ranges.push(RangePoint::Tri(range));
    }

    pub fn add_quad(&mut self, range: DamageQuad) {
        self.ranges.push(RangePoint::Quad(range));
    }

    pub fn get_damage_at_dist(&mut self, range: u32) -> Fract {
        let mut last_point = &self.start_point;
        for point in &self.ranges {
            match point {
                RangePoint::Single(pos) => {
                    let dist = pos.range - last_point.range;
                    if range < dist {
                        return math::get_sp(*last_point, *pos, range);
                    }
                }
                RangePoint::Tri(tri) => {}
                RangePoint::Quad(quad) => {}
            }
        }
    }
}
