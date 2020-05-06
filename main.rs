use std::collections::VecDeque;

fn main() {
    fn _testing(nums: &Vec<usize>, sort: fn(nums: &mut Vec<usize>)) {
        // let mut nums_copy: Vec<usize> = nums.clone();
        let mut nums_copy: Vec<usize> = nums.to_vec();
        sort(&mut nums_copy);
        println!("{:?}", nums_copy)
    }
    let nums: Vec<usize> = vec![5, 1, 3, 6, 8, 7, 2, 4];
    _testing(&nums, bubble);
    _testing(&nums, insert);
    _testing(&nums, select);
    _testing(&nums, shell);
    _testing(&nums, count);
    _testing(&nums, merge);
    _testing(&nums, quick);
    _testing(&nums, heap);
}

fn bubble(nums: &mut Vec<usize>) {
    for _i in 0..nums.len() - 1 {
        for i in 1..nums.len() - _i {
            if nums[i - 1] > nums[i] {
                nums.swap(i - 1, i);
            }
        }
    }
}

fn insert(nums: &mut Vec<usize>) {
    for i in 1..nums.len() {
        let (mut p, v) = (i, nums[i]);
        while p > 0 && nums[p - 1] > v {
            nums[p] = nums[p - 1];
            p -= 1;
        }
        nums[p] = v;
    }
}

fn select(nums: &mut Vec<usize>) {
    for i in 0..nums.len() - 1 {
        let mut p = i;
        for j in i + 1..nums.len() {
            if nums[j] < nums[p] {
                p = j;
            }
        }
        nums.swap(i, p);
    }
}

fn shell(nums: &mut Vec<usize>) {
    fn _insert(nums: &mut Vec<usize>, g: usize) {
        for i in g..nums.len() {
            let (mut p, v) = (i, nums[i]);
            while p >= g && nums[p - g] > v {
                nums[p] = nums[p - g];
                p -= g;
            }
            nums[p] = v;
        }
    }
    let mut a: VecDeque<usize> = VecDeque::new();
    a.push_front(1);
    while *a.front().unwrap() <= nums.len() {
        a.push_front(3 * a.front().unwrap() + 1);
    }
    for &g in a.iter() {
        _insert(nums, g);
    }
}

fn count(nums: &mut Vec<usize>) {
    let max = nums.iter().max().unwrap();
    let origin = nums.clone();
    let mut count: Vec<usize> = Vec::new();
    for _ in 0..max + 1 {
        count.push(0);
    }
    for &v in nums.iter() {
        count[v] += 1;
    }
    for i in 1..count.len() {
        count[i] += count[i - 1];
    }
    for &v in origin.iter() {
        nums[count[v] - 1] = v;
        count[v] -= 1;
    }
}

fn merge(nums: &mut Vec<usize>) {
    fn _merge(nums: &mut Vec<usize>, left: usize, mid: usize, right: usize) {
        let left_part: Vec<usize> = nums[left..mid].iter().cloned().collect();
        let right_part: Vec<usize> = nums[mid..right].iter().cloned().collect();
        let (mut left_offset, mut right_offset) = (0usize, 0usize);
        while left_offset < left_part.len() || right_offset < right_part.len() {
            if right_offset == right_part.len()
                || (left_offset < left_part.len()
                    && left_part[left_offset] <= right_part[right_offset])
            {
                nums[left + left_offset + right_offset] = left_part[left_offset];
                left_offset += 1;
            } else {
                nums[left + left_offset + right_offset] = right_part[right_offset];
                right_offset += 1;
            }
        }
    }
    fn _sort(nums: &mut Vec<usize>, left: usize, right: usize) {
        if left + 1 < right {
            let mid = (left + right) / 2;
            _sort(nums, left, mid);
            _sort(nums, mid, right);
            _merge(nums, left, mid, right);
        }
    }
    _sort(nums, 0, nums.len());
}

fn quick(nums: &mut Vec<usize>) {
    fn _partition(nums: &mut Vec<usize>, begin: usize, end: usize) -> usize {
        let (mut i, v) = (begin, nums[end - 1]);
        for j in begin..end - 1 {
            if nums[j] <= v {
                nums.swap(i, j);
                i += 1;
            }
        }
        nums.swap(i, end - 1);
        i
    }
    fn _quick(nums: &mut Vec<usize>, begin: usize, end: usize) {
        if begin + 1 < end {
            let mid = _partition(nums, begin, end);
            _quick(nums, begin, mid);
            _quick(nums, mid, end);
        }
    }
    _quick(nums, 0, nums.len());
}

#[allow(dead_code)]
fn heap(_nums: &mut Vec<usize>) {}
