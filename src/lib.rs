mod seqnums;
mod types;

use types::{MsgType};

const SFID: u8 = 0; // todo get/set? give sensible default?

fn _build_add_request() -> Result<types::Msg, ()> {
    let _add_msg = types::Msg {
        version: 0,
        msg_type: MsgType::REQUEST,
        code: types::RequestType::ADD as u8,
        sfid: SFID,
        seqnum: 0, // TODO get/set seqnums
    };

    Err(())
}

fn _serialize(_msg: types::Msg) {
    // todo figure out return type
}

