pub mod answers;
pub mod guesses;

pub fn build(list: &[&[u8; 5]]) -> Vec<[u8; 5]> {
    list.into_iter()
        .map(|v| {
            let mut r = [0, 0, 0, 0, 0];
            (0..5).for_each(|i| r[i] = v[i]);
            r
        })
        .collect()
}
