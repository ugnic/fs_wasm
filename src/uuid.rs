use crate::mt;

pub(crate) fn generate() -> Vec<u16> {
    let mut uuid: Vec<u16> = Vec::with_capacity(16);
    let mut mt = mt::MersenneTwister::new();
    for _i in 0..16 {
        let digits = (mt.next() % 0xff) as u16;
        uuid.push(digits);
    }
    uuid[6] = (uuid[6] & 0x0f) | 0x40;
    uuid[8] = (uuid[8] & 0x3f) | 0x80;
    uuid
}

pub(crate) fn parse(uuid: Vec<u16>) -> String {
    format!(
        "{:x}{:x}{:x}{:x}-{:x}{:x}-{:x}{:x}-{:x}{:x}-{:x}{:x}{:x}{:x}{:x}{:x}"
        ,uuid[0],uuid[1],uuid[2],uuid[3]
        ,uuid[4],uuid[5]
        ,uuid[6],uuid[7]
        ,uuid[8],uuid[9]
        ,uuid[10],uuid[11],uuid[12],uuid[13],uuid[14],uuid[15]
    )
}