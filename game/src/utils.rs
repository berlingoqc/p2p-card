

pub fn vec_u32_to_u8(data: &[u32]) -> Vec<u8> {
    let capacity = data.len();
    let mut output = Vec::with_capacity(capacity);
    for value in data {
        let v: u8 = *value as u8;
        output.push(v);
    }
    output
}

pub fn splice<T>(v: &mut Vec<T>, start: usize, count: usize) -> Vec<T> {
    if start >= v.len() {
        return Vec::new(); // Handle out-of-bounds start index
    }

    let end = std::cmp::min(start + count, v.len()); // Clamp end index to vector length
    let removed = v.drain(start..end).collect(); 

    removed
}