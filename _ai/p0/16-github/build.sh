rustc src/p0c.rs -o bin/p0c
rustc src/vm0.rs -o bin/vm0
rustc src/ir0c.rs -o bin/ir0c
rustc --crate-type=staticlib src/lib0.rs -o bin/lib0.a