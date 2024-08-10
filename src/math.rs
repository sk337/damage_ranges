use super::posCord;
use super::Fract;

pub fn get_sp(start: posCord, end: posCord, range: u32) -> Fract {
    let rise = end.damage - start.damage.clone();
    let run = end.range - start.range.clone();
    let m = rise / run;
    let b = start.damage - m * start.range;
    m * range + b
}
