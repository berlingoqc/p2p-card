

pub fn vec_u32_to_u8(data: &[u32]) -> Vec<u8> {
    let capacity = data.len();
    let mut output = Vec::with_capacity(capacity);
    for value in data {
        let v: u8 = *value as u8;
        output.push(v);
    }
    output
}