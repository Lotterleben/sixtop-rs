
use std::vec::Vec;
use std::convert::TryInto;

use crate::types::{Cell, CellList,
                   Msg, MsgHdr, MsgType,
                   SixtopMsg,
                   Request, Response,
                   PREAMBLE_TYPE_MASK};

const SIXTOP_HDR_SZ_BYTES: usize = 4;

fn deserialize_cell_list(data: Vec<u8>) -> Result<CellList, ()> {
    // TODO: make this somehow less.. awful
    let mut cell_list = CellList::new();
    for (position, _) in data.iter().enumerate() {
        if (position % 4) == 0 {
            let this_elem = data.get(position..position+2).unwrap();
            let next_elem = data.get(position+2..position+4).unwrap();
            let cell = Cell{ slot_offset: u16::from_le_bytes(this_elem.try_into().unwrap()),
                                    channel_offset: u16::from_le_bytes(next_elem.try_into().unwrap())};
            cell_list.push(cell);
        } // otherwise this is data we've already read in the prev step; skipping
    }

    Ok(cell_list)
}

fn deserialize_request_body(mut data: Vec<u8>) -> Result<Request, ()> {
    let mut request = Request::new();

    let metadata = data.get(0..2).unwrap();
    request.metadata = u16::from_le_bytes(metadata.try_into().unwrap());
    request.cell_options = *data.get(2).unwrap();
    request.num_cells = *data.get(3).unwrap();

    let previous_data_sz = 4;
    request.cell_list = deserialize_cell_list( data.split_off(previous_data_sz) ).unwrap();

    Ok(request)
}

fn deserialize_header(data: Vec<u8>) -> Result<MsgHdr, ()> {
    let mut header = MsgHdr::new(MsgType::Unassigned);
    // todo coherence check for: data length, preamble (version, reserved)...

    let preamble = *data.get(0).unwrap();
    header.msg_type = MsgType::from_u8((PREAMBLE_TYPE_MASK & preamble) >> 2).unwrap();
    header.code = *data.get(1).unwrap(); // todo coherence check?
    header.sfid = *data.get(2).unwrap();
    header.seqnum = *data.get(3).unwrap();

    Ok(header)
}

pub fn deserialize_message(mut data: Vec<u8>) -> Result<SixtopMsg, ()>
    {
    let payload = data.split_off(SIXTOP_HDR_SZ_BYTES);
    let msg_hdr = deserialize_header(data).unwrap();
    match msg_hdr.msg_type {
        MsgType::REQUEST => {
            let mut request = deserialize_request_body(payload).unwrap();
            request.header = msg_hdr;
            Ok(SixtopMsg::RequestMsg(request))
        }
        MsgType::RESPONSE => {
            // note: response body only contains a CellList, no need for a dedicated
            // deserialize_response_body()
            let mut response = Response::new();
            response.header = msg_hdr;
            response.cell_list = deserialize_cell_list(payload).unwrap();

            Ok(SixtopMsg::ResponseMsg(response))
        }
        _ => { unimplemented!() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{DEFAULT_SFID, RequestType,
                       ReturnCode,
                       SixtopMsg, Request, Response};

    const TEST_SEQNUM: u8 = 4;
    const TEST_METADATA: u16 = 0b1111_1111_0000_0000;

    #[test]
    fn test_deserialize_response_header() {
        let test_hdr = vec![0b0000_0100, ReturnCode::RC_ERR as u8, DEFAULT_SFID, TEST_SEQNUM];

        let mut ref_msg_hdr = MsgHdr::new(MsgType::RESPONSE);
        ref_msg_hdr.code = ReturnCode::RC_ERR as u8;
        ref_msg_hdr.seqnum = TEST_SEQNUM;

        let result = deserialize_header(test_hdr).unwrap();
        assert_eq!(result, ref_msg_hdr);
    }

    #[test]
    fn test_deserialize_request() {
        let test_msg = vec![0b0000_0000, RequestType::ADD as u8, DEFAULT_SFID, TEST_SEQNUM,
                                     0b0000_0000, 0b1111_1111, 0b0000_0100, 3, 1, 0, 2, 0, 3, 0, 9, 0];

        let mut reference_msg = Request::new();
        reference_msg.header.code = RequestType::ADD as u8;
        reference_msg.header.seqnum = TEST_SEQNUM;

        reference_msg.metadata = TEST_METADATA;
        reference_msg.cell_options = 0b100;
        reference_msg.num_cells = 3;
        reference_msg.cell_list.push(Cell{slot_offset: 1, channel_offset: 2});
        reference_msg.cell_list.push(Cell{slot_offset: 3, channel_offset: 9});

        let result = deserialize_message(test_msg).unwrap();
        if let SixtopMsg::RequestMsg(request) = result {
            assert_eq!(request, reference_msg);
        } else {
            // should have been recognized as a response
            assert_eq!(0, 1);
        }
    }

    #[test]
    fn test_deserialize_response() {
        let test_msg = vec![0b0000_0100, ReturnCode::RC_ERR_SEQNUM as u8, DEFAULT_SFID,
                                     TEST_SEQNUM, 2, 0, 3, 0, 4, 0, 5, 0];

        let mut reference_msg = Response::new();
        reference_msg.header.code = ReturnCode::RC_ERR_SEQNUM as u8;
        reference_msg.header.seqnum = TEST_SEQNUM;

        reference_msg.cell_list.push(Cell{slot_offset: 2, channel_offset: 3});
        reference_msg.cell_list.push(Cell{slot_offset: 4, channel_offset: 5});

        let result = deserialize_message(test_msg).unwrap();
        if let SixtopMsg::ResponseMsg(response) = result {
            assert_eq!(response, reference_msg);
        } else {
            // should have been recognized as a response
            assert_eq!(0, 1);
        }
    }

    // TODO add test for incomplete celllist
}