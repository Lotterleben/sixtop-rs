pub mod seqnums;
pub mod types;
pub mod msg_builder;
pub mod msg_reader;

use once_cell::sync::OnceCell;

use crate::types::{Msg, Response, ReturnCode, SixtopMsg};
use crate::seqnums::SeqNums;

static SEQNUMS: OnceCell<SeqNums> = OnceCell::new();

pub fn init() {
   SEQNUMS.set(SeqNums::new()).unwrap();
}

// dummy handling for now
// returns an answer to be sent if necessary
pub fn handle_msg(msg: SixtopMsg) -> Result<Option<SixtopMsg>, ()>{
    match msg {
        SixtopMsg::RequestMsg(request) => {
            // dummy: pick first two cells and accept
            let mut response = Response::new();
            response.header.code = ReturnCode::RC_SUCCESS as u8;
            response.header.seqnum = request.header.seqnum; // TODO store and handle seqnum etc

            // just choose the first two cells. obvs missing coherence check etc
            for index in 0..request.num_cells {
                response.cell_list.push(*request.cell_list.get(index as usize).unwrap());
            }

            Ok(Some(SixtopMsg::ResponseMsg(response)))
        },
        SixtopMsg::ResponseMsg(response) => { unimplemented!() },
        _ => { unimplemented!() }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_lib() {
        assert_eq!(0,0);
    }
}

