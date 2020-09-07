#![allow(dead_code)]

/**
 * Each node remembers the last used SeqNum for each neighbor.
 * That is, a node stores as many SeqNum values as it has neighbors.
 */
use std::collections::HashMap;
use crate::types::{NeighborID};

pub type SeqNum = u8;
pub const DEFAULT_SEQNUM : SeqNum = 0;

struct SeqNums {
    values: HashMap<NeighborID, SeqNum>,
}

impl Default for SeqNums {
    fn default() -> SeqNums {
        SeqNums {
            values: HashMap::new(),
        }
    }
}

impl SeqNums {
    pub fn new() -> SeqNums {
        SeqNums{..Default::default()}
    }

    pub fn add_neighbor(&mut self, neighbor: NeighborID, seqnum: SeqNum) {
        (*self).values.insert(neighbor, seqnum);
    }

    pub fn get_seqnum_for_neighbor(_neighbor: NeighborID) -> Result<SeqNum, ()> {
        return Ok(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_neighbor() {
        let test_neighbor: NeighborID = 11;
        let mut test_seqnums = SeqNums::new();
        let test_seqnum = 1;
        test_seqnums.add_neighbor(test_neighbor, test_seqnum);
        assert_eq!(*(test_seqnums.values.get(&test_neighbor).unwrap()), test_seqnum);
    }

    #[test]
    fn test_get_seqnum() {
        let test_neighbor: NeighborID = 0;
        //let result = get_seqnum(test_neighbor);
        //assert_eq!(result.unwrap(), 0);
    }
}