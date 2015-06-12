use std::sync::{Arc, Mutex};
use std::thread;

struct Philosopher {
    name:  String,
    left:  usize, // index into forks vector
    right: usize, //
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name:  name.to_string(),
            left:  left,
            right: right,
        }
    }

    fn eat(&self, forks: &Vec<Mutex<()>>) {
        let _left = forks[self.left].lock();
        println!("{} picked up fork #{}", self.name, self.left);

        let _right = forks[self.right].lock();
        println!("{} picked up fork #{}", self.name, self.right);

        println!("{} begins eating", self.name);

        thread::sleep_ms(100);

        println!("{} is done eating", self.name);
    }
}

fn main() {
    let philosophers = vec![
        Philosopher::new("Arne Næss",         0, 1),
        Philosopher::new("Gunnar Skirbekk",   1, 2),
        Philosopher::new("Søren Kierkegaard", 2, 3),
        Philosopher::new("Ludvig Holberg",    0, 3),
    ];

    // create a fork (mutex) for each philosopher
    let forks : Vec<_> = philosophers.iter().map(|_| Mutex::new(())).collect();
    let forks = Arc::new(forks);

    let handles : Vec<_> = philosophers.into_iter().map(|philosopher| {
        let t = forks.clone();
        thread::spawn(move || { philosopher.eat(&t); })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

