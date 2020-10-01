pub mod msg_builder;
pub mod msg_reader;
pub mod seqnums;
pub mod types;

use crate::seqnums::SeqNums;
use crate::types::{Msg, NeighborID, Response, ReturnCode, SixtopMsg};

pub struct Sixtop {
    seqnums: SeqNums,
}

impl Sixtop {
    pub fn new() -> Sixtop {
        Sixtop {
            seqnums: SeqNums::new(),
        }
    }

    pub fn handle_msg(
        &mut self,
        sender: NeighborID,
        msg: SixtopMsg,
    ) -> Result<Option<SixtopMsg>, ()> {
        match msg {
            SixtopMsg::RequestMsg(request) => {
                let mut response = Response::new();

                match self.seqnums.verify(sender, request.header.seqnum) {
                    Ok(seqnum) => {
                        response.header.code = ReturnCode::RC_SUCCESS as u8;
                        response.header.seqnum = seqnum;

                        // DUMMY: just choose the first two cells. obvs missing coherence check etc.
                        // Proper pick should be done by the SF.
                        for index in 0..request.num_cells {
                            response
                                .cell_list
                                .push(*request.cell_list.get(index as usize).unwrap());
                        }
                        // TODO lock in cells in schedule

                        // TODO this is not the right way to do this: "if node A receives the link-layer
                        // acknowledgment for its 6P Request, it will increment the SeqNum by exactly 1
                        // after the 6P Transaction ends."
                        self.seqnums.increment_seqnum(sender);
                    }
                    Err(_) => unimplemented!(),
                }

                Ok(Some(SixtopMsg::ResponseMsg(response)))
            }
            SixtopMsg::ResponseMsg(response) => {
                match self.seqnums.verify(sender, response.header.seqnum) {
                    Ok(_) => {
                        // TODO lock in cells in schedule

                        // TODO this is not the right way to do this: "if node A receives the link-layer
                        // acknowledgment for its 6P Request, it will increment the SeqNum by exactly 1
                        // after the 6P Transaction ends."
                        self.seqnums.increment_seqnum(sender);

                        println!("6top TRANSACTION COMPLETE");

                        Ok(None)
                    }
                    Err(_) => { unimplemented!() }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_lib() {
        assert_eq!(0, 0);
    }
}
