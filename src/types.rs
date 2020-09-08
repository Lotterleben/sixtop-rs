#![allow(dead_code)]

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum MsgType {
    REQUEST = 0,
    RESPONSE = 1,
    CONFIRMATION = 2,
    Unassigned = 3,
}

#[repr(u8)]
pub enum RequestType {
    Reserved,
    ADD,
    DELETE,
    RELOCATE,
    COUNT,
    LIST,
    SIGNAl,
    CLEAR,
    Unassigned,
}

#[allow(non_camel_case_types)]
pub enum ReturnCode {
    RC_SUCCESS = 0,
    RC_EOL,
    RC_ERR,
    RC_RESET,
    RC_ERR_VERSION,
    RC_ERR_SFID,
    RC_ERR_SEQNUM,
    RC_ERR_CELLLIST,
    RC_ERR_BUSY,
    RC_ERR_LOCKED,
    Unassigned,
}

//                            +-+-+-+-+-+-+-+-+
// mask to get/set the T in   |Version| T | R |
//                            +-+-+-+-+-+-+-+-+
pub const PREAMBLE_TYPE_MASK: u8 = 0b00001100;

pub const DEFAULT_SFID: u8 = 0; // todo check with std
pub const SIXTOP_VERSION: u8 = 0;

pub type NeighborID = u8; // todo use actually useful type
pub type CellList = Vec<Cell>;

pub struct Cell {
    pub slot_offset: u16,
    pub channel_offset: u16,
}

#[derive(Debug, PartialEq)]
pub struct MsgHdr {
    pub msg_type: MsgType,
    pub code: u8, // RequestType for requests, ReturnCode for responses
    pub sfid: u8,
    pub seqnum: u8,
}

pub struct Request {
    pub header: MsgHdr,
    pub metadata: u16,
    pub cell_options: u8,
    pub num_cells: u8,
    pub cell_list: CellList,
    // TODO how do we best add a second celllist for RELOCATE requests?
    // use an option type?
}

pub struct Response {
    pub header: MsgHdr,
    pub cell_list: CellList,
}

impl MsgType {
    pub fn from_u8(value: u8) -> Result<MsgType, ()> {
        match value {
            0 => { Ok(MsgType::REQUEST) }
            1 => { Ok(MsgType::RESPONSE) }
            2 => { Ok(MsgType::CONFIRMATION) }
            _ => { Err(()) }
        }
    }
}

impl MsgHdr {
    pub fn new() -> MsgHdr {
        MsgHdr {
            msg_type: MsgType::Unassigned,
            code: 0,
            sfid: DEFAULT_SFID,
            seqnum: 0,
        }
    }
}

impl Request {
    pub fn new() -> Request {
        Request {
            header: MsgHdr::new(),
            metadata: 0,
            cell_options: 0,
            num_cells: 0,
            cell_list: CellList::new(),
        }
    }
}

impl Response {
    pub fn new() -> Response {
        Response {
            header: MsgHdr::new(),
            cell_list: CellList::new(),
        }
    }
}