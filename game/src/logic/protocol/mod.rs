use prost::Message;
use protocol::{
    frame::parse_function_frame,
    generated::{
        chain::{BlockProposal, BlockProposalFinalResponse, BlockProposalResponse, Blockchain},
        msg::{ClientHandlers, Presentation},
    },
};

pub struct ProtocolHandler {}

impl ProtocolHandler {
    pub fn handle_message(msg: &Vec<u8>) -> Result<(), ()> {
        if let Ok((id, data)) = parse_function_frame(msg) {
            let id = ClientHandlers::from(id);
            match id {
                ClientHandlers::Presentation => {
                    let p = Presentation::decode(data).unwrap();
                }
                ClientHandlers::BlockProposal => {
                    let p = BlockProposal::decode(data).unwrap();
                }
                ClientHandlers::BlockProposalFinalResponse => {
                    let p = BlockProposalFinalResponse::decode(data).unwrap();
                }
                ClientHandlers::BlockProposalResponse => {
                    let p = BlockProposalResponse::decode(data).unwrap();
                }
                ClientHandlers::Chain => {
                    let p = Blockchain::decode(data).unwrap();
                }
            }
        }

        Ok(())
    }
}
