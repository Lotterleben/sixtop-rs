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

pub struct Msg {
    pub msg_type: MsgType,
    pub code: u8, // RequestType for requests, ReturnCode for responses
    pub sfid: u8,
    pub seqnum: u8,
}

impl Msg {
    pub fn new() -> Msg {
        Msg {
            msg_type: MsgType::Unassigned,
            code: 0,
            sfid: DEFAULT_SFID,
            seqnum: 0,
        }
    }

    // style Q: would I impl getters and setters here or just access the struct like normal?
}