use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, VecDeque};

#[derive(Copy, Clone)]
pub struct Task {
    time: i32,
    index: usize,
}
#[derive(Default)]
pub struct TaskQueue {
    queue: VecDeque<Task>,
}

impl TaskQueue {
    pub fn add_new_task(&mut self, task: Task) {
        self.queue.push_back(task);
    }

    pub fn get_task(&mut self) -> Option<Task> {
        self.queue.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Server {
    weight: i32,
    index: usize,
}

impl PartialOrd for Server {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            match (self.weight.cmp(&other.weight), self.index.cmp(&other.index)) {
                (Ordering::Less, _) => Ordering::Greater,
                (Ordering::Greater, _) => Ordering::Less,
                (Ordering::Equal, Ordering::Less) => Ordering::Greater,
                (Ordering::Equal, Ordering::Equal) => Ordering::Equal,
                (Ordering::Equal, Ordering::Greater) => Ordering::Less,
            },
        )
    }
}

impl Ord for Server {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
    let servers: Vec<Server> = {
        servers
            .iter()
            .enumerate()
            .map(|(index, &weight)| Server { index, weight })
            .collect()
    };
    let tasks: Vec<Task> = {
        tasks
            .iter()
            .enumerate()
            .map(|(index, &time)| Task { index, time })
            .collect()
    };

    let mut available_servers: BinaryHeap<Server> = {
        let mut heap = BinaryHeap::default();
        for server in &servers {
            heap.push(*server);
        }
        heap
    };

    let mut availability_tracker: BTreeMap<i32, Vec<usize>> = BTreeMap::new();

    let mut task_schedule = vec![0; tasks.len()];

    let mut current_time = 0;
    for task in tasks {
        // Get available servers at the current time.
        while let Some((time, newly_available_servers)) = availability_tracker.first_key_value() {
            if *time > current_time {
                break; // Not yet.
            }
            available_servers.extend(
                newly_available_servers
                    .iter()
                    .map(|&index| servers[index])
                    .collect::<Vec<_>>(),
            );

            availability_tracker.pop_first();
        }

        // Handle this task.
        if available_servers.is_empty() {
            let (time, newly_available_servers) = availability_tracker.first_key_value().unwrap();
            current_time = *time;
            available_servers.extend(
                newly_available_servers
                    .iter()
                    .map(|&index| servers[index])
                    .collect::<Vec<_>>(),
            );

            availability_tracker.pop_first();
        }

        let target_server = available_servers.pop().unwrap();
        task_schedule[task.index] = target_server.index;
        let next_available = current_time + task.time;
        availability_tracker
            .entry(next_available)
            .or_default()
            .push(target_server.index);

        current_time += 1;
    }

    task_schedule.iter().map(|&x| x as i32).collect()
}

#[test]
fn example_1() {
    let servers = vec![3, 3, 2];
    let tasks = vec![1, 2, 3, 2, 1, 2];
    assert_eq!(assign_tasks(servers, tasks), vec![2, 2, 0, 2, 1, 2]);
}

#[test]
fn example_2() {
    let servers = vec![5, 1, 4, 3, 2];
    let tasks = vec![2, 1, 2, 4, 5, 2, 1];
    assert_eq!(assign_tasks(servers, tasks), vec![1, 4, 1, 4, 1, 3, 2]);
}
