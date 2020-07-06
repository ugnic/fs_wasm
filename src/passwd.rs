use crate::mt;

const BASE_CHAR:[char; 62] = [
    'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',
    'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
    '0','1','2','3','4','5','6','7','8','9'
];

pub(crate) fn generate(len: usize) -> String {
    let mut mt = mt::MersenneTwister::new();
    let mut passwd: Vec<char> = Vec::default();
    for _i in 0..len {
        let index: usize = (mt.next() % 61) as usize;
        passwd.push(BASE_CHAR[index]);
    };
    passwd.iter().cloned().collect::<String>()
}