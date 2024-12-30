
pub fn create_function_frame<T: prost::Message>(function_id: u32, compression: u8, message: &T) -> Result<Vec<u8>, ()> {

    let mut payload = Vec::new();
    message.encode(&mut payload).unwrap();

    let mut grpc_frame = Vec::with_capacity(5 + payload.len());
    grpc_frame.push(compression);
    grpc_frame.extend(&(payload.len() as u32).to_be_bytes());
    grpc_frame.extend_from_slice(&payload);

    let mut function_frame = Vec::with_capacity(4 + grpc_frame.len());
    function_frame.extend(&function_id.to_be_bytes());
    function_frame.extend_from_slice(&grpc_frame);

    Ok(function_frame)
}


pub fn parse_function_frame(frame: &[u8]) -> Result<(u32, &[u8]), ()> {
    if frame.len() < 9 {
        return Err(());
    }

    let function_id = u32::from_be_bytes([frame[0], frame[1], frame[2], frame[3]]);

    let grpc_frame = &frame[4..];

    Ok((function_id, &grpc_frame[5..]))
}



#[cfg(test)]
mod tests {

    use prost::Message;

    use crate::generated::msg::Presentation;

    use super::*;

    #[test]
    fn encode_and_decode() {
        let mut presentation = Presentation::default();
        presentation.name = "test".into();


        let frame = create_function_frame(15, 0, &presentation).unwrap();


        let (id, data) = parse_function_frame(&frame).unwrap();
        let new_presentation = Presentation::decode(data).unwrap();


        assert_eq!(15, id);
        assert_eq!("test".to_string(), new_presentation.name);

    }
}