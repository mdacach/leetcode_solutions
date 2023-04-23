#[derive(Debug, Copy, Clone)]
struct Interval {
    start: i32,
    end: i32,
}

pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut intervals = intervals
        .iter()
        .map(|v| Interval {
            start: v[0],
            end: v[1],
        })
        .collect::<Vec<_>>();
    let new_interval = Interval {
        start: new_interval[0],
        end: new_interval[1],
    };

    let closest = closest_smaller(&intervals, &new_interval);
    match closest {
        None => {
            intervals.insert(0, new_interval);
        }
        Some(index) => {
            intervals.insert(index + 1, new_interval);
        }
    }

    let answer = merge(&intervals);

    answer.iter().map(|i| vec![i.start, i.end]).collect()
}

fn closest_smaller(intervals: &Vec<Interval>, new_interval: &Interval) -> Option<usize> {
    intervals.get(0).and_then(|f| {
        if f.start > new_interval.start {
            None
        } else {
            for (index, interval) in intervals.iter().enumerate() {
                if interval.start >= new_interval.start {
                    return Some(index - 1);
                }
            }
            Some(intervals.len() - 1)
        }
    })
}

fn merge(intervals: &[Interval]) -> Vec<Interval> {
    if intervals.is_empty() {
        return vec![];
    }

    let mut answer = vec![];
    let mut current = intervals[0];
    for interval in intervals {
        if interval.start <= current.end {
            current.end = std::cmp::max(current.end, interval.end);
        } else {
            answer.push(current);
            current = *interval;
        }
    }
    answer.push(current);

    answer
}

#[test]
fn example_1() {
    let intervals = vec![vec![1, 3], vec![6, 9]];
    let new_interval = vec![2, 5];
    assert_eq!(
        insert(intervals, new_interval),
        vec![vec![1, 5], vec![6, 9]]
    );
}

#[test]
fn example_2() {
    let intervals = vec![
        vec![1, 2],
        vec![3, 5],
        vec![6, 7],
        vec![8, 10],
        vec![12, 16],
    ];
    let new_interval = vec![4, 8];
    assert_eq!(
        insert(intervals, new_interval),
        vec![vec![1, 2], vec![3, 10], vec![12, 16]]
    );
}
