#![allow(dead_code)]

use crate::types::NeighborID;
/**
 * Each node remembers the last used SeqNum for each neighbor.
 * That is, a node stores as many SeqNum values as it has neighbors.
 */
use std::collections::HashMap;

pub type SeqNum = u8;
pub const DEFAULT_SEQNUM: SeqNum = 0;

#[derive(Debug)]
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
        SeqNums {
            ..Default::default()
        }
    }

    /// TODO do I ever need this?
    /// If a SeqNum entry for `neighbor` already exists, return it.
    /// If it doesn't, create a new entry and return its initial seqnum.
    pub fn guaranteed_get_seqnum(&mut self, neighbor: NeighborID) -> SeqNum {
        match (*self).values.get(&neighbor) {
            None => {
                self.add_neighbor(neighbor, DEFAULT_SEQNUM);
                DEFAULT_SEQNUM
            }
            Some(seqnum) => *seqnum,
        }
    }

    /// When node B receives a 6P Request from node A with SeqNum equal to 0, it checks the stored
    /// SeqNum for A. If A is a new neighbor, the stored SeqNum in B will be 0. The transaction can
    /// continue. If the stored SeqNum for A in B is different than 0, a potential inconsistency is
    /// detected. In this case, B MUST return RC_ERR_SEQNUM with SeqNum=0. The SF of node A MAY
    /// decide what to do next, as described in Section 3.4.6.2.
    ///
    /// returns Ok(<neighbor seqnum>) if `seqnum` is legitimate,
    ///         Err on seqnum inconsistency
    pub fn update_seqnum(&mut self, neighbor: NeighborID, seqnum: SeqNum) -> Result<SeqNum, ()> {
        match self.get_seqnum(neighbor) {
            Some(known_seqnum) => {
                match (seqnum, *known_seqnum) {
                    (0, 0) => Ok(seqnum),
                    (0, _) => {
                        /* inconsistency detected */
                        Err(())
                    }
                    (new, old) if new > old => Ok(seqnum),
                    _ => unimplemented!(),
                }
            }
            None => {
                self.add_neighbor(neighbor, seqnum);
                Ok(seqnum)
            }
        }
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
        assert_eq!(
            *(test_seqnums.values.get(&TEST_NEIGHBOR).unwrap()),
            TEST_SEQNUM
        );
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
