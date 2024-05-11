use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

// the median is the value in a list of sorted numbers where half the values are below and half the values are above
// it is the midpoint number or the average of the two equal midpoints

// SOLUTIONS
// SLOW keep a sorted array as you add numbers, but it will take O(n^2) time
// FAST/EFFICIENT just use heaps

// GENERAL NOTES
// Modularize first
// Shows excellent coding style, and helps conceptualize the process

#[derive(Debug, PartialEq)]
struct MinFloat(f64);

impl Eq for MinFloat {}

impl PartialOrd for MinFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for MinFloat {
    fn cmp(&self, other: &MinFloat) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, PartialEq)]
struct MaxFloat(f64);

impl Eq for MaxFloat {}

impl PartialOrd for MaxFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for MaxFloat {
    fn cmp(&self, other: &MaxFloat) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    // for higher nums
    let mut highers = BinaryHeap::new();
    // for lower nums
    let mut lowers = BinaryHeap::new();
    // all values
    let vals = vec![1.0, 5.0, 6.1, 1.2, 7.0, 11.2];

    for val in vals {
        add_number(val, &mut lowers, &mut highers);
        rebalance(&mut lowers, &mut highers);
    }

    let median: f64 = get_median(&mut lowers, &mut highers);
    println!("{:?}", median);
}

fn add_number(
    val: f64,
    lowers: &mut BinaryHeap<Reverse<MinFloat>>,
    highers: &mut BinaryHeap<MinFloat>,
) {
    if lowers.len() == 0 || Reverse(MinFloat(val)) < *lowers.peek().unwrap() {
        lowers.push(Reverse(MinFloat(val)))
    } else {
        highers.push(MinFloat(val))
    }
}

fn rebalance(lowers: &mut BinaryHeap<Reverse<MinFloat>>, highers: &mut BinaryHeap<MinFloat>) {
    if lowers.len().checked_sub(highers.len()) >= Some(2) {
        let Reverse(val) = lowers.pop().unwrap();
        highers.push(val);
    }
    if highers.len().checked_sub(lowers.len()) >= Some(2) {
        let val = highers.pop().unwrap();
        lowers.push(Reverse(val));
    }
}

fn get_median(
    lowers: &mut BinaryHeap<Reverse<MinFloat>>,
    highers: &mut BinaryHeap<MinFloat>,
) -> f64 {
    if (lowers.len() + highers.len()) % 2 == 0 {
        println!("low: {:?}, high: {:?}", lowers, highers);
        println!("len {}, {}", lowers.len(), highers.len());
        let Reverse(MinFloat(low_val)) = lowers.pop().unwrap();
        let MinFloat(high_val) = highers.pop().unwrap();
        println!("{}, {}", low_val, high_val);
        return (low_val + high_val) / 2.0;
    } else if lowers.len() > highers.len() {
        let Reverse(MinFloat(low_val)) = lowers.pop().unwrap();
        return low_val;
    } else {
        let MinFloat(high_val) = highers.pop().unwrap();
        return high_val;
    }
}
