use std::time::{SystemTime, UNIX_EPOCH};

wit_bindgen::generate!({
    world: "my-world",
    generate_all,
});

struct MyHost;

impl Guest for MyHost {
    fn run() -> String {
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        println!("before sleep: {:?}", start);

        my_sleep().block();

        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        println!("after sleep: {:?}", end);

        "Hello, world!".to_string()
    }
}

export!(MyHost);
