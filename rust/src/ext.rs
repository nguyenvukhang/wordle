use crate::types::Arr5;

pub trait String5 {
    fn to_arr(&self) -> Arr5;
}

impl String5 for &str {
    fn to_arr(&self) -> Arr5 {
        let (mut arr, word) = ([0, 0, 0, 0, 0], self.as_bytes());
        (0..5).for_each(|i| arr[i] = word[i]);
        return arr;
    }
}
