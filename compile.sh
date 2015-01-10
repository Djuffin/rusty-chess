rustc -C target-feature=sse3,sse4.1,sse4.2 -C opt-level=3 -g --cfg ndebug -o tests_rchess  --test main.rs 
