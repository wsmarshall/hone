use std::error;
use std::io;
use std::str::FromStr;

fn meeting_rooms(intervals: Vec<Vec<i32>>) -> bool {
    let mut meetings = intervals.clone();
    meetings.sort();
    for i in 1..meetings.len() {
        if meetings[i][0] < meetings[i - 1][1] {
            return false;
        }
    }
    true
}
