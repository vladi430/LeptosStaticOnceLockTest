This project started with the actix template:
```
cargo leptos new --git https://github.com/leptos-rs/start-actix
```

1. Created `src/components`, `src/api`, `src/oncelock`, `src/pages`
2. Moved `HomePage` and `NotFound` inside `src/pages`
3. Added  
    ```rust
    pub mod pages;
    pub mod components;
    pub mod oncelock;
    pub mod api;
    ```  
    in `lib.rs`
4. Added  
    ```rust
    pub mod oncelock;
    ```  
    in `main.rs`, which initializes the OnceLock.
5. Added `connection_pool_test.rs` to `src/components`, `src/api`, `src/oncelock`, `src/pages`, which tries to access the value inside the OnceLock
6. The button in the `HomePage` is executing all test server functions



### Expected behavior
The goal is that the program prints:
```
[api]: Some(42)
```

### Actual behavior
The program prints:
```
[api]: None
[components]: None
[oncelock]: Some(42)
[pages]: None
```
