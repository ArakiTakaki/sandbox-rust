use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Table {
    forks: Vec<Mutex<()>>,
}

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        return Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        };
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        thread::sleep(Duration::from_millis(150));
        let _right = table.forks[self.right].lock().unwrap();

        println!("{}は食事中です。", self.name);

        thread::sleep(Duration::from_millis(1000));

        println!("{}は食事を終えました", self.name);
    }
}

fn sample() -> String {
    println!("sample");
    let sample = String::new();
    return sample;
}

fn main() {
    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ],
    });

    println!("Hello, world!");
    let philosopher_list = vec![
        Philosopher::new("荒木", 0, 1),
        Philosopher::new("井上", 1, 1),
        Philosopher::new("酒井", 2, 3),
        Philosopher::new("吉田", 3, 4),
        Philosopher::new("梶", 0, 4),
    ];

    println!("{}", sample());
    let handles: Vec<_> = philosopher_list
        .into_iter()
        .map(|p| {
            let table = table.clone();

            thread::spawn(move || {
                p.eat(&table);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}
