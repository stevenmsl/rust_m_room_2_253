use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, Eq, PartialEq)]
pub struct Meeting {
  start: usize,
  end: usize,
}

impl Meeting {
  pub fn length(&self) -> usize {
    &self.end - &self.start
  }
}

pub struct Solution {}

/* takeaways
   - the key concept is that if the start time
     of a meeting that is about to be kicked-off
     is later than the end time of the on-going
     meeting that will be wrapped-up the earliest
     you don't need an additional room
   - sort all the meetings by the start time
   - use a min heap to keep track of the end time
     of all on-going meetings
     - the len of the heap is how many rooms you
       need once you looped through all the meetings
   - walk through all meetings and see in the end
     how many rooms are needed
*/

impl Solution {
  pub fn min_meeting_rooms(meetings: &mut Vec<Meeting>) -> usize {
    /* sort by start time ascending */
    meetings.sort_by(|meet_1, meet_2| meet_1.start.cmp(&meet_2.start));
    println!("sorted intervals: {:?}", meetings);

    let mut meeting_rooms = BinaryHeap::new();

    /*
      - book our very first meeting room and
        record the end time
      - use min heap so we can ensure the
        the room that will be freed up the
        earliest will be at the root of
        the binary heap
      - use Reverse ordering to create
        a min heap
    */
    meeting_rooms.push(Reverse(meetings[0].end));

    for i in 1..meetings.len() - 1 {
      /*
        - the on-going meeting that will be finished
          the earliest
        - when you peek it will Option<Reverse<usize>>
          that's why you need to unwrap first to get
          to Reverse, and then use .0 to get to the
          end time stored in Reverse struct
      */
      let end_time = meeting_rooms.peek().unwrap().0;
      /*
        - remember that we have sorted the meetings
          by start time already
        - the min heap has all the meetings that are
          still on-going.
        - we compare the start time of the meeting
          to be kicked off to the on-going meeting
          that will be finished the earliest
      */
      if meetings[i].start >= end_time {
        /*
           - we don't need a extra room as one will be
             freed up once this meeting started
        */
        meeting_rooms.pop();
      }

      /*
        - two scenarios:
          - reuse the meeting room that will be freed up
            with an updated end time
          - allocate an additional meeting room with
            an end time
        - coding-wise these two scenarios are the same
          - don't forget we already pop the meeting room
            out if it can be freed up
      */
      meeting_rooms.push(Reverse(meetings[i].end));
    }

    meeting_rooms.len()
  }
}

pub struct TestFixtures {}
impl TestFixtures {
  pub fn test_fixture_1() -> Vec<Meeting> {
    vec![
      Meeting { start: 0, end: 30 },
      Meeting { start: 5, end: 10 },
      Meeting { start: 15, end: 20 },
    ]
  }
  pub fn test_fixture_2() -> Vec<Meeting> {
    vec![Meeting { start: 7, end: 10 }, Meeting { start: 2, end: 4 }]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let result = Solution::min_meeting_rooms(&mut TestFixtures::test_fixture_1());
    assert_eq!(result, 2);
  }
  #[test]
  fn test_2() {
    let result = Solution::min_meeting_rooms(&mut TestFixtures::test_fixture_2());
    assert_eq!(result, 1);
  }
}
