
use std::vec::Vec;
use crate::types::{MsgHdr, MsgType, ReturnCode, PREAMBLE_TYPE_MASK};

fn deserialize_header(data: Vec<u8>) -> Result<MsgHdr, ()> {
    let mut header = MsgHdr::new();
    // todo coherence check for: data length, preamble (version, reserved)...

    let preamble = *data.get(0).unwrap();
    header.msg_type = MsgType::from_u8((PREAMBLE_TYPE_MASK & preamble) >> 2).unwrap();
    header.code = *data.get(1).unwrap(); // todo coherence check?
    header.sfid = *data.get(2).unwrap();
    header.seqnum = *data.get(3).unwrap();

    Ok(header)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{DEFAULT_SFID};

    const TEST_SEQNUM: u8 = 4;

    #[test]
    fn test_deserialize_response_header() {
        let test_hdr = vec![0b0000_0100, ReturnCode::RC_ERR as u8, DEFAULT_SFID, TEST_SEQNUM];

        let mut ref_msg_hdr = MsgHdr::new();
        ref_msg_hdr.msg_type = MsgType::RESPONSE;
        ref_msg_hdr.code = ReturnCode::RC_ERR as u8;
        ref_msg_hdr.seqnum = TEST_SEQNUM;

        let result = deserialize_header(test_hdr).unwrap();
        assert_eq!(result, ref_msg_hdr);
    }
}