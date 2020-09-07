#![allow(dead_code)]

/**
 * Each node remembers the last used SeqNum for each neighbor.
 * That is, a node stores as many SeqNum values as it has neighbors.
 */
use std::collections::HashMap;
use crate::types::{NeighborID};

pub type SeqNum = u8;
pub const DEFAULT_SEQNUM : SeqNum = 0;

pub struct SeqNums {
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

    pub fn get_seqnum(&mut self, neighbor: NeighborID) -> Option<&SeqNum> {
        (*self).values.get(&neighbor)
    }

    pub fn reset_seqnum(&mut self, neighbor: NeighborID) {
        let curr_seqnum = (*self).values.get_mut(&neighbor);
        if let Some(s) = curr_seqnum {
            *s = 0;
        }
    }

    /**
     * @return the new sequence number if a sequence number for @p neighbor exists
     */
    pub fn increment_seqnum(&mut self, neighbor: NeighborID) {
        let curr_seqnum = (*self).values.get_mut(&neighbor);
        if let Some(s) = curr_seqnum {
            match *s {
                0xFF => {
                    /* The SeqNum MUST be implemented as a lollipop counter: it rolls over
                     * from 0xFF to 0x01 (not to 0x00). This is used to detect a neighbor reset */
                    (*s) = 1;
                }
                _ => {
                    (*s) += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO more constants less copypasta

    #[test]
    fn test_add_neighbor() {
        let test_neighbor: NeighborID = 11;
        let mut test_seqnums = SeqNums::new();
        let test_seqnum = 1;

        // RUN TEST
        test_seqnums.add_neighbor(test_neighbor, test_seqnum);

        // ASSERT POSTCONDITION
        assert_eq!(*(test_seqnums.values.get(&test_neighbor).unwrap()), test_seqnum);
    }

    #[test]
    fn test_get_seqnum() {
        let test_neighbor: NeighborID = 22;
        let mut test_seqnums = SeqNums::new();
        let test_seqnum = 2;
        test_seqnums.add_neighbor(test_neighbor, test_seqnum);

        // RUN TEST
        let result = test_seqnums.get_seqnum(test_neighbor);

        // ASSERT POSTCONDITION
        assert_eq!(*result.unwrap(), test_seqnum);
    }

    #[test]
    fn test_reset_seqnum() {
        let test_neighbor: NeighborID = 11;
        let mut test_seqnums = SeqNums::new();
        let test_seqnum = 1;
        test_seqnums.add_neighbor(test_neighbor, test_seqnum);

        // RUN TEST
        test_seqnums.reset_seqnum(test_neighbor);

        // ASSERT POSTCONDITION
        let result = test_seqnums.get_seqnum(test_neighbor).unwrap();
        assert_eq!(*result, 0);
    }
    #[test]
    fn test_increment_seqnum() {
        let test_neighbor: NeighborID = 33;
        let mut test_seqnums = SeqNums::new();
        let test_seqnum = 3;
        test_seqnums.add_neighbor(test_neighbor, test_seqnum);

        // RUN TEST
        test_seqnums.increment_seqnum(test_neighbor);

        // ASSERT POSTCONDITION
        let result = test_seqnums.get_seqnum(test_neighbor).unwrap();
        assert_eq!(*result, 4);
    }

    /*
     * The SeqNum MUST be implemented as a lollipop counter: it rolls over from 0xFF to 0x01
     * (not to 0x00). This is used to detect a neighbor reset
     */
    #[test]
    fn test_increment_seqnum_wraparound() {
        let test_neighbor: NeighborID = 22;
        let mut test_seqnums = SeqNums::new();
        let test_seqnum = 0xFF;
        test_seqnums.add_neighbor(test_neighbor, test_seqnum);

        // RUN TEST
        test_seqnums.increment_seqnum(test_neighbor);

        // ASSERT POSTCONDITION
        let result = test_seqnums.get_seqnum(test_neighbor).unwrap();
        assert_eq!(*result, 1);
    }
}