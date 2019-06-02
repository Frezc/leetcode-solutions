use std::collections::HashMap;

struct Cache {
    cache: HashMap<String, bool>
}

impl Cache {
    fn new() -> Self {
        Cache {
            cache: HashMap::new()
        }
    }

    fn solution(&mut self) -> bool {
        false
    }
}