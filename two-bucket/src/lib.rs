use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}


#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    
    pub moves: u8,
    
    pub goal_bucket: Bucket,
    
    pub other_bucket: u8,
}


pub fn solve(capacity_1: u8, capacity_2: u8, goal: u8, start_bucket: &Bucket) -> BucketStats {
    let mut queue = VecDeque::from(vec![(0, 0, 0)]);
    let mut seen = HashSet::new();
    match start_bucket {
        Bucket::One => seen.insert((0, capacity_2)),
        Bucket::Two => seen.insert((capacity_1, 0)),
    };
    while let Some((moves, bucket_1, bucket_2)) = queue.pop_front() {
        if seen.contains(&(bucket_1, bucket_2)) {
            continue;
        }
        seen.insert((bucket_1, bucket_2));
        if bucket_1 == goal {
            return BucketStats {
                moves,
                goal_bucket: Bucket::One,
                other_bucket: bucket_2,
            };
        } else if bucket_2 == goal {
            return BucketStats {
                moves,
                goal_bucket: Bucket::Two,
                other_bucket: bucket_1,
            };
        }
        queue.push_back((moves + 1, 0, bucket_2));
        queue.push_back((moves + 1, bucket_1, 0));
        queue.push_back((moves + 1, capacity_1, bucket_2));
        queue.push_back((moves + 1, bucket_1, capacity_2));
        let volume = bucket_1 + bucket_2;
        if volume > capacity_2 {
            queue.push_back((moves + 1, volume - capacity_2, capacity_2));
        } else {
            queue.push_back((moves + 1, 0, volume));
        }
        if volume > capacity_1 {
            queue.push_back((moves + 1, capacity_1, volume - capacity_1));
        } else {
            queue.push_back((moves + 1, volume, 0));
        }
    }
    BucketStats {
        moves: 0,
        goal_bucket: Bucket::One,
        other_bucket: 0,
    }
}