/**
You've begun your new job to organize newspapers.
Each morning, you are to separate the newspapers into smaller piles and
assign each pile to a co-worker.

This way, your co-workers can read through the newspapers and
examine their contents simultaneously.

Each newspaper is marked with a read time to finish all its contents.
 A worker can read one newspaper at a time, and, when they finish one,
  they can start reading the next.

  Your goal is to minimize the amount of time needed for your co-workers to finish all newspapers.
  Additionally, the newspapers came in a particular order, and
  you must not disarrange the original ordering when distributing the newspapers.
  In other words, you cannot pick and choose newspapers randomly from the whole pile
  to assign to a co-worker, but rather you must take a subsection of consecutive newspapers
  from the whole pile.

What is the minimum amount of time it would take to have your coworkers go
through all the newspapers?

That is, if you optimize the distribution of newspapers,
what is the longest reading time among all piles?
Constraints

1 <= newspapers_read_times.length <= 10^5

1 <= newspapers_read_times[i] <= 10^5

1 <= num_coworkers <= 10^5
Examples
Example 1:
Input: newspapers_read_times = [7,2,5,10,8], num_coworkers = 2
Output: 18
Explanation:

Assign first 3 newspapers to one coworker then assign the rest to another. The time it takes for the first 3 newspapers is 7 + 2 + 5 = 14 and for the last 2 is 10 + 8 = 18.
Example 2:
Input: newspapers_read_times = [2,3,5,7], num_coworkers = 3
Output: 7
Explanation:

Assign [2, 3], [5], and [7] separately to workers. The minimum time is 7.
*/

//helper fn for testing a time for feasibility
fn feasible(newspapers_read_times: &Vec<i32>, num_readers: i32, limit: i32) -> bool {
    let mut time = 0;
    let mut workers = 0;

    for i in newspapers_read_times {
        if time + i > limit {
            time = 0;
            workers += 1;
        }
        time += i;
    }

    if time != 0 {
        workers += 1;
    }

    workers <= num_readers
}

fn newspapers_split(newspapers_read_times: Vec<i32>, num_coworkers: i32) -> i32 {
    let mut left: i32 = *newspapers_read_times.iter().max().unwrap();
    let mut right: i32 = newspapers_read_times.iter().sum::<i32>();

    let mut ans = 0;

    while left < right {
        let mid = left + (right - left) / 2;

        if feasible(&newspapers_read_times, num_coworkers, mid) {
            ans = mid;
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    ans
}
