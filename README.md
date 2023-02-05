# Meelf

Meelf is a simple web server implementation.

## Router

```rs
use matchit::Router;

let mut router = Router::new();
router.insert("/home", "Welcome!")?;
router.insert("/users/:id", "A User")?;

let matched = router.at("/users/978")?;
assert_eq!(matched.params.get("id"), Some("978"));
assert_eq!(*matched.value, "A User");
```

## TODO

- [ ] Implement http query params parsing.
  - Use [this](https://github.com/viz-rs/path-tree) library for reference.
- [ ] Implement generic http request handler.
- [ ] Support for different content types.
- [ ] user defined middlewares.
- [ ] Serve static files.

## Things to look at

1. https://docs.rs/gloo-events/0.1.2/gloo_events/struct.EventListener.html
2. https://app.codecrafters.io/vote/challenge-extension-ideas?course=redis
3. https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html
4. https://github.com/amirrezaask/khadem/blob/master/rust/src/http/request.rs
5. https://doc.rust-lang.org/nomicon/races.html
6. https://github.com/steveklabnik/simple-server/blob/master/src/lib.rs

## Credits

- https://github.com/amirrezaask/khadem
- https://github.com/ibraheemdev/matchit/blob/master/README.md
