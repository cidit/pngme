pub fn to_xbyte_arr<const C: usize>(s: &[u8]) -> [u8; C] {
    let mut arr = [0u8; C];
    for i in 0..C {
        arr[i] = s[i]
    }
    arr
}