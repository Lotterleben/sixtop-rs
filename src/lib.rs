pub mod seqnums;
pub mod types;
pub mod msg_builder;
pub mod msg_reader;

use once_cell::sync::OnceCell;

use crate::types::{Msg, NeighborID, Response, ReturnCode, SixtopMsg};
use crate::seqnums::SeqNums;

static SEQNUMS: OnceCell<SeqNums> = OnceCell::new();

pub fn init() {
   SEQNUMS.set(SeqNums::new()).unwrap();
}

// dummy handling for now
// returns an answer to be sent if necessary
pub fn handle_msg(_sender: NeighborID, msg: SixtopMsg) -> Result<Option<SixtopMsg>, ()>{
    match msg {
        SixtopMsg::RequestMsg(_request) => {
            unimplemented!()
            // dummy: pick first two cells and accept
            //let mut response = Response::new();

            //let foo = SEQNUMS.get_mut().unwrap();
            //let bar = (*foo).update_seqnum(sender, 33);
            /*
            match SEQNUMS.get().unwrap().update_seqnum(sender, request.header.seqnum) {
                Ok(new_seqnum) => {
                    response.header.code = ReturnCode::RC_SUCCESS as u8;
                    response.header.seqnum = new_seqnum;

                    // DUMMY: just choose the first two cells. obvs missing coherence check etc
                    for index in 0..request.num_cells {
                        response.cell_list.push(*request.cell_list.get(index as usize).unwrap());
                    }

                    // TODO this is not the right way to do this: "if node A receives the link-layer
                    // acknowledgment for its 6P Request, it will increment the SeqNum by exactly 1
                    // after the 6P Transaction ends."
                    //SEQNUMS.get().unwrap().increment_seqnum(sender);
                }
                Err(_) => { unimplemented!() }
            }
            */

            //Ok(Some(SixtopMsg::ResponseMsg(response)))
        }
        SixtopMsg::ResponseMsg(_response) => { unimplemented!() }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_lib() {
        assert_eq!(0,0);
    }
}

