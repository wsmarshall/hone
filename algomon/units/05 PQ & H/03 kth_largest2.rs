use std::error;
use std::io;
use std::str::FromStr;

//takes quickselect approach - O(n) in average case, faster than O(n + k log n) in heap
//approach, but slightly trickier to implement

fn quickselect(nums: &mut Vec<i32>, l: usize, r: usize, p: usize) -> usize {
    if nums.len() == 1 {
        return r;
    }

    let mut vacant = l;
    let pivot = r;

    for i in l..r {
        if nums[i] <= nums[pivot] {
            nums.swap(i, vacant);
            vacant += 1;
        }
    }

    nums.swap(vacant, pivot);

    if vacant > p {
        //recurse on left half
        return quickselect(nums, 0, vacant - 1, p);
    } else if vacant < p {
        //recurse on right half
        return quickselect(nums, vacant + 1, r, p);
    } else {
        return nums[vacant].try_into().unwrap();
    }
}

fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let place: usize = nums.len() - k as usize; //final index for return value
    let last_place = nums.len() - 1;
    let mut nums = nums.clone();

    quickselect(&mut nums, 0, last_place, place)
        .try_into()
        .unwrap()
}
