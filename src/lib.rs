/* FILL in the blanks
.
├── Cargo.lock
├── Cargo.toml
├── src
│   ├── main.rs
│   └── lib.rs*/

// in src/lib.rs

pub mod compute;

use std::collections::{HashMap, BTreeMap, HashSet};

fn task1() {
    let _c1:HashMap<&str, i32> = HashMap::new();
    let mut c2 = BTreeMap::new();
    c2.insert(1, "a");
    let _c3: HashSet<i32> = HashSet::new();
}

