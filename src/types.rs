#![allow(dead_code)]
pub enum MsgType {
    REQUEST,
    RESPONSE,
    CONFIRMATION
}
pub enum RequestType {
    Reserved,
    ADD,
    DELETE,
    RELOCATE,
    COUNT,
    LIST,
    SIGNAl,
    CLEAR,
}

pub struct Msg {
    pub version: u8,
    pub msg_type: MsgType,
    pub code: u8, // RequestType if msg_type is REQUEST; ReturnCode if msg_type is RESPONSE/CONFIRMATION
    pub sfid: u8,
    pub seqnum: u8,
}

pub type NeighborID = u8; // todo sue actually useful type
