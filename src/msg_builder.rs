use std::io::Cursor;
use std::vec::Vec;

use crate::types::{Msg,
                   MsgHdr};

// mask to set the T in
// +-+-+-+-+-+-+-+-+
// |Version| T | R |
// +-+-+-+-+-+-+-+-+
const PREAMBLE_TYPE_MASK: u8 = 0b00001100;

// todo useful lifetime
fn serialize_header(msg_hdr: MsgHdr) -> Result<Vec<u8>, ()> {
    let mut bytes = Vec::new();

    //        +-+-+-+-+-+-+-+-+   where version = SIXTOP_VERSION
    // create |Version| T | R |         T = REQUEST
    //        +-+-+-+-+-+-+-+-+         R = 0b00
    let preamble: u8 = PREAMBLE_TYPE_MASK & ((msg_hdr.msg_type as u8) << 2);

    bytes.push(preamble);
    bytes.push(msg_hdr.code);
    bytes.push(msg_hdr.sfid);
    bytes.push(msg_hdr.seqnum);

    Ok(bytes)
}

fn serialize_msg(msg: Msg) -> Result<Vec<u8>, ()> {
    // TODO do we want to do some sort of coherence check for the msg type and code fields?
    let mut header = serialize_header(msg.header).unwrap();

    let payload = Vec::new();

    header.extend_from_slice(&payload);
    Ok(header)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{DEFAULT_SFID,
                       MsgType,
                       RequestType,
                       ReturnCode};

    const TEST_SEQNUM: u8 = 4;

    #[test]
    fn test_serialize_request_header() {
        let mut test_request = MsgHdr::new();
        test_request.msg_type = MsgType::REQUEST;
        test_request.code = RequestType::ADD as u8;
        test_request.seqnum = TEST_SEQNUM;

        // RUN TEST
        let result = serialize_header(test_request).unwrap();

        // ASSERT POSTCONDITION
        assert_eq!(result.as_slice(),
                   [0b00000000, RequestType::ADD as u8, DEFAULT_SFID, TEST_SEQNUM]);
    }

    #[test]
    fn test_serialize_response_header() {
        let mut test_request = MsgHdr::new();
        test_request.msg_type = MsgType::RESPONSE;
        test_request.code = ReturnCode::RC_ERR as u8;
        test_request.seqnum = TEST_SEQNUM;

        // RUN TEST
        let result = serialize_header(test_request).unwrap();

        // ASSERT POSTCONDITION
        assert_eq!(result.as_slice(),
                   [0b00000100, ReturnCode::RC_ERR as u8, DEFAULT_SFID, TEST_SEQNUM]);
    }
}

