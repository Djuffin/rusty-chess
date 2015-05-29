rustc -L ./target/debug/deps/ --extern rand=./target/debug/deps/librand-b924d9fc5b3eb5b8.rlib -C target-feature=sse3,sse4.1,sse4.2 -C opt-level=3 -g --cfg ndebug -o tests_rchess  --test src/main.rs 
