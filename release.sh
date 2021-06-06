cargo build --release

[ ! -d "build/" ] && mkdir build || echo "'build' directory present, over-writing"

cp target/release/main.exe build/sorter.exe