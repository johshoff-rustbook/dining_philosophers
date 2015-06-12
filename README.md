Surprises when implementing:

- Mutex takes takes a type
- Type annotations for collect gets a little weird. There must be a better way:

    let forks : Vec<_> = philosophers.iter().map(|_| Mutex::new(())).collect();
    let forks = Arc::new(forks);

