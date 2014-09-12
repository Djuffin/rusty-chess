rustc -C target-feature=sse3,sse4.1,sse4.2 --opt-level=3 --cfg ndebug  main.rs
#  -emit asm
