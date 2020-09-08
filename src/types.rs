#![allow(dead_code)]

#[repr(u8)]
pub enum MsgType {
    REQUEST,
    RESPONSE,
    CONFIRMATION,
    Unassigned,
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

pub const DEFAULT_SFID: u8 = 0; // todo check with std
pub const SIXTOP_VERSION: u8 = 0;

pub type NeighborID = u8; // todo use actually useful type

pub struct MsgHdr {
    pub msg_type: MsgType,
    pub code: u8, // RequestType for requests, ReturnCode for responses
    pub sfid: u8,
    pub seqnum: u8,
}

pub struct Msg {
    pub header: MsgHdr,
    pub metadata: u16,
    pub cell_options: u8,
    pub num_cells: u8,
    pub cell_list: u8 // todo
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
    // style Q: would I impl getters and setters here or just access the struct like normal?
}

impl Msg {
    pub fn new() -> Msg {
        Msg {
            header: MsgHdr::new(),
            metadata: 0,
            cell_options: 0,
            num_cells: 0,
            cell_list: 0, // todo
        }
    }
}