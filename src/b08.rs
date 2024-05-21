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
fn test_time(list: &Vec<usize>, time: usize, workers: usize) -> bool {
    let mut workers_used = 0;
    let mut temp = 0;

    for i in list {
        if i + temp > time && workers_used < workers {
            workers_used = workers_used + 1;
            temp = *i;
        } else if i + temp <= time && workers_used < workers {
            temp = temp + i;
        } else {
            return false;
        }
    }

    return true;
}

pub fn newspapers_split(list: &[usize], num_coworkers: usize) -> usize {
    let mut smallest_time = 0;
    let mut largest_time = 0;
    let mut times = Vec::new();

    for i in 0..list.len() {
        if list[i] > smallest_time {
            smallest_time = list[i];
        }
        largest_time = largest_time + list[i];
    }

    for i in smallest_time..largest_time + 1 {
        times.push(i);
    }
    println!("times list: {:?}", times);

    let length = times.len();

    let mut left = 0;
    let mut right = length - 1; //guaranteed non-empty array
    let mut mid = 0;

    while left <= right {
        println!("just inside while");
        mid = left + ((right - left) / 2); //to avoid overflow

        if test_time(&times, times[mid], num_coworkers) {
            //true
            if right < 1 {
                println!("just inside less if case");
                //avoids underflow from usize indexing
                //particularly when right = 0
                break;
            } else {
                println!("just inside less else case");
                println!("left, right, mid = {}, {}, {}", left, right, mid);
                right = mid - 1;
            }
        } else {
            left = mid + 1;
        }
    }

    mid
}
