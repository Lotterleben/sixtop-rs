#![allow(dead_code)]

#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MsgType {
    REQUEST = 0,
    RESPONSE = 1,
    CONFIRMATION = 2, // currently unimplemented
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

pub const SIXTOP_VERSION: u8 = 0;

pub type NeighborID = u8; // todo use actually useful type
pub type CellList = Vec<Cell>;

#[allow(non_camel_case_types)]
pub type SFID = u8;

pub const DEFAULT_SFID: SFID = 0; // todo check with std

#[derive(Debug, PartialEq, Copy, Clone)]
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

#[derive(Debug, PartialEq)]
// TODO impl debug for this and the data structures it uses for nicer visualization?
pub struct Request {
    pub header: MsgHdr,
    pub metadata: u16,
    pub cell_options: u8,
    pub num_cells: u8,
    pub cell_list: CellList,
    // TODO how do we best add a second celllist for RELOCATE requests?
    // use an option type?
}

#[derive(Debug, PartialEq)]
pub struct Response {
    pub header: MsgHdr,
    pub cell_list: CellList,
}

// Meta container for parsing returns
#[derive(Debug)]
pub enum SixtopMsg {
    RequestMsg(Request),
    ResponseMsg(Response),
}

pub trait SchedulingFunction {
    fn get_sfid() -> SFID;
    fn pick_cells(cell_list: Vec<Cell>, num_cells: u8) -> Result<Vec<Cell>, ()>;
    // TODO: how do I let the SF trigger requests?
}

pub trait Msg {
    fn new() -> Self;
}

impl MsgHdr {
    //        +-+-+-+-+-+-+-+-+   where version = SIXTOP_VERSION = 0
    // create |Version| T | R |         T = REQUEST
    //        +-+-+-+-+-+-+-+-+         R = 0b00
    pub fn serialize(&self) -> Result<Vec<u8>, ()> {
        let mut bytes = Vec::new();
        let preamble: u8 = PREAMBLE_TYPE_MASK & ((self.msg_type as u8) << 2);

        bytes.push(preamble);
        bytes.push(self.code);
        bytes.push(self.sfid);
        bytes.push(self.seqnum);

        Ok(bytes)
    }
}

// TODO add getters and setterr for req/resp header fields, esp. header.code to avoid type confusion
impl Msg for Request {
    fn new() -> Request {
        Request {
            header: MsgHdr::new(MsgType::REQUEST),
            metadata: 0,
            cell_options: 0,
            num_cells: 0,
            cell_list: CellList::new(),
        }
    }
}

impl Msg for Response {
    fn new() -> Response {
        Response {
            header: MsgHdr::new(MsgType::RESPONSE),
            cell_list: CellList::new(),
        }
    }
}

impl MsgType {
    pub fn from_u8(value: u8) -> Result<MsgType, ()> {
        match value {
            0 => Ok(MsgType::REQUEST),
            1 => Ok(MsgType::RESPONSE),
            2 => Ok(MsgType::CONFIRMATION),
            _ => Err(()),
        }
    }
}

impl MsgHdr {
    pub fn new(msg_type: MsgType) -> MsgHdr {
        MsgHdr {
            msg_type: msg_type,
            code: 0,
            sfid: DEFAULT_SFID,
            seqnum: 0,
        }
    }
}
