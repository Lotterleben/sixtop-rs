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

    const TEST_NEIGHBOR: NeighborID = 22;
    const TEST_SEQNUM: SeqNum = 3;

    // TODO more constants less copypasta

    #[test]
    fn test_add_neighbor() {
        let mut test_seqnums = SeqNums::new();

        // RUN TEST
        test_seqnums.add_neighbor(TEST_NEIGHBOR, TEST_SEQNUM);

        // ASSERT POSTCONDITION
        assert_eq!(*(test_seqnums.values.get(&TEST_NEIGHBOR).unwrap()), TEST_SEQNUM);
    }

    #[test]
    fn test_get_seqnum() {
        let mut test_seqnums = SeqNums::new();
        test_seqnums.add_neighbor(TEST_NEIGHBOR, TEST_SEQNUM);

        // RUN TEST
        let result = test_seqnums.get_seqnum(TEST_NEIGHBOR);

        // ASSERT POSTCONDITION
        assert_eq!(*result.unwrap(), TEST_SEQNUM);
    }

    #[test]
    fn test_reset_seqnum() {
        let mut test_seqnums = SeqNums::new();
        test_seqnums.add_neighbor(TEST_NEIGHBOR, TEST_SEQNUM);

        // RUN TEST
        test_seqnums.reset_seqnum(TEST_NEIGHBOR);

        // ASSERT POSTCONDITION
        let result = test_seqnums.get_seqnum(TEST_NEIGHBOR).unwrap();
        assert_eq!(*result, 0);
    }
    #[test]
    fn test_increment_seqnum() {
        let mut test_seqnums = SeqNums::new();
        test_seqnums.add_neighbor(TEST_NEIGHBOR, TEST_SEQNUM);
        let next_seqnum = TEST_SEQNUM + 1;

        // RUN TEST
        test_seqnums.increment_seqnum(TEST_NEIGHBOR);

        // ASSERT POSTCONDITION
        let result = test_seqnums.get_seqnum(TEST_NEIGHBOR).unwrap();
        assert_eq!(*result, next_seqnum);
    }

    /*
     * The SeqNum MUST be implemented as a lollipop counter: it rolls over from 0xFF to 0x01
     * (not to 0x00). This is used to detect a neighbor reset
     */
    #[test]
    fn test_increment_seqnum_wraparound() {
        let mut test_seqnums = SeqNums::new();
        let max_seqnum = 0xFF;
        test_seqnums.add_neighbor(TEST_NEIGHBOR, max_seqnum);

        // RUN TEST
        test_seqnums.increment_seqnum(TEST_NEIGHBOR);

        // ASSERT POSTCONDITION
        let result = test_seqnums.get_seqnum(TEST_NEIGHBOR).unwrap();
        assert_eq!(*result, 1);
    }
}