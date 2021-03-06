use std::vec::Vec;

use crate::types::{CellList, Request, Response};

fn serialize_cell_list(cell_list: CellList) -> Result<Vec<u8>, ()> {
    let mut bytes = Vec::new();

    for cell in cell_list {
        bytes.extend_from_slice(&cell.slot_offset.to_le_bytes());
        bytes.extend_from_slice(&cell.channel_offset.to_le_bytes());
    }

    Ok(bytes)
}

// TODO could these just be struct impls?
pub fn serialize_request(request: Request) -> Result<Vec<u8>, ()> {
    // TODO do we want to do some sort of coherence check for the msg type and code fields?
    let mut header = request.header.serialize().unwrap();

    let mut payload = Vec::new();
    payload.extend_from_slice(&request.metadata.to_le_bytes());
    payload.push(request.cell_options);
    payload.push(request.num_cells);
    payload.extend_from_slice(&serialize_cell_list(request.cell_list).unwrap());

    header.extend_from_slice(&payload);
    Ok(header)
}

pub fn serialize_response(response: Response) -> Result<Vec<u8>, ()> {
    // TODO do we want to do some sort of coherence check for the msg type and code fields?
    let mut header = response.header.serialize().unwrap();
    let payload = serialize_cell_list(response.cell_list).unwrap();
    header.extend_from_slice(&payload);
    Ok(header)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Cell, Msg, MsgType, RequestType, ReturnCode, DEFAULT_SFID};

    const TEST_SEQNUM: u8 = 4;
    const TEST_METADATA: u16 = 0b1111_1111_0000_0000;

    #[test]
    fn test_serialize_request_header() {
        let mut test_hdr = MsgHdr::new(MsgType::REQUEST);
        test_hdr.code = RequestType::ADD as u8;
        test_hdr.seqnum = TEST_SEQNUM;

        // RUN TEST
        let result = test_hdr.serialize().unwrap();

        // ASSERT POSTCONDITION
        assert_eq!(
            result.as_slice(),
            [
                0b0000_0000,
                RequestType::ADD as u8,
                DEFAULT_SFID,
                TEST_SEQNUM
            ]
        );
    }

    #[test]
    fn test_serialize_response_header() {
        let mut test_hdr = MsgHdr::new(MsgType::RESPONSE);
        test_hdr.code = ReturnCode::RC_ERR as u8;
        test_hdr.seqnum = TEST_SEQNUM;

        // RUN TEST
        let result = test_hdr.serialize().unwrap();

        // ASSERT POSTCONDITION
        assert_eq!(
            result.as_slice(),
            [
                0b0000_0100,
                ReturnCode::RC_ERR as u8,
                DEFAULT_SFID,
                TEST_SEQNUM
            ]
        );
    }

    #[test]
    fn test_serialize_request() {
        let mut test_request = Request::new();
        test_request.header.code = RequestType::ADD as u8;
        test_request.header.seqnum = TEST_SEQNUM;

        test_request.metadata = TEST_METADATA;
        test_request.cell_options = 0b100;
        test_request.num_cells = 3;
        test_request.cell_list.push(Cell {
            slot_offset: 1,
            channel_offset: 2,
        });
        test_request.cell_list.push(Cell {
            slot_offset: 3,
            channel_offset: 9,
        });

        // RUN TEST
        let result = serialize_request(test_request).unwrap();

        // ASSERT POSTCONDITION
        assert_eq!(
            result.as_slice(),
            [
                0b0000_0000,
                RequestType::ADD as u8,
                DEFAULT_SFID,
                TEST_SEQNUM,
                0b0000_0000,
                0b1111_1111,
                0b0000_0100,
                3,
                1,
                0,
                2,
                0,
                3,
                0,
                9,
                0
            ]
        );
    }

    #[test]
    fn test_serialize_response() {
        let mut test_response = Response::new();
        test_response.header.code = ReturnCode::RC_ERR_SEQNUM as u8;
        test_response.header.seqnum = TEST_SEQNUM;

        test_response.cell_list.push(Cell {
            slot_offset: 2,
            channel_offset: 3,
        });
        test_response.cell_list.push(Cell {
            slot_offset: 4,
            channel_offset: 5,
        });

        // RUN TEST
        let result = serialize_response(test_response).unwrap();

        // ASSERT POSTCONDITION
        assert_eq!(
            result.as_slice(),
            [
                0b0000_0100,
                ReturnCode::RC_ERR_SEQNUM as u8,
                DEFAULT_SFID,
                TEST_SEQNUM,
                2,
                0,
                3,
                0,
                4,
                0,
                5,
                0
            ]
        );
    }
}
