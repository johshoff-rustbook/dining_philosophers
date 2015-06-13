Surprises when implementing:

- Mutex takes takes a type
- Type annotations for collect gets a little weird. There must be a better way (**update** there was; calling collect as `collect::<Vec<_>>()`):

        let forks : Vec<_> = philosophers.iter().map(|_| Mutex::new(())).collect();
        let forks = Arc::new(forks);

- I wanted to share the `forks` vector with the philosopher threads without cloning
  any data. I [asked the question on stack overflow](http://stackoverflow.com/questions/30795600/is-it-possible-to-share-data-with-threads-without-any-cloning)
  and unfortunately it looks like that's not possible.
