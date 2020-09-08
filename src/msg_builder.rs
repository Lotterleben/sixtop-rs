use std::io::Cursor;

use byte_strings::concat_bytes;
use scroll::IOwrite;

use crate::types::{Msg,
                   MsgHdr};

const HDR_SZ: usize = 4; // in bytes
const MAX_PAYLOAD_SZ: usize = 128 - HDR_SZ; // in bytes; chosen at random

// mask to set the T in
// +-+-+-+-+-+-+-+-+
// |Version| T | R |
// +-+-+-+-+-+-+-+-+
const PREAMBLE_TYPE_MASK: u8 = 0b00001100;


// todo useful lifetime
fn serialize_header(msg_hdr: MsgHdr) -> Result<[u8; HDR_SZ], ()> {
    let mut bytes = [0; HDR_SZ];
    let mut cursor = Cursor::new(&mut bytes[..]);

    //        +-+-+-+-+-+-+-+-+   where version = SIXTOP_VERSION
    // create |Version| T | R |         T = REQUEST
    //        +-+-+-+-+-+-+-+-+         R = 0b00
    let preamble: u8 = PREAMBLE_TYPE_MASK & ((msg_hdr.msg_type as u8) << 2);

    cursor.iowrite(preamble).unwrap();
    cursor.iowrite(msg_hdr.code).unwrap();
    cursor.iowrite(msg_hdr.sfid).unwrap();
    cursor.iowrite(msg_hdr.seqnum).unwrap();

    Ok(bytes)
}

fn serialize_msg(msg: Msg) {
    // TODO do we want to do some sort of coherence check for the msg type and code fields?
    let header = serialize_header(msg.header).unwrap();

    let mut payload = [0; MAX_PAYLOAD_SZ];
    let mut cursor = Cursor::new(&mut payload[..]);
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
        let result = serialize_header(test_request);

        // ASSERT POSTCONDITION
        assert_eq!(result,
                   Ok([0b00000000, RequestType::ADD as u8, DEFAULT_SFID, TEST_SEQNUM]));
    }

    #[test]
    fn test_serialize_response_header() {
        let mut test_request = MsgHdr::new();
        test_request.msg_type = MsgType::RESPONSE;
        test_request.code = ReturnCode::RC_ERR as u8;
        test_request.seqnum = TEST_SEQNUM;

        // RUN TEST
        let result = serialize_header(test_request);

        // ASSERT POSTCONDITION
        assert_eq!(result,
                   Ok([0b00000100, ReturnCode::RC_ERR as u8, DEFAULT_SFID, TEST_SEQNUM]));
    }
}

