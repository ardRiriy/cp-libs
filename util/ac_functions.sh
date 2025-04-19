#!/bin/zsh

gpp(){
	g++ $1 -std=c++23 -W -Wall -fsanitize=undefined
	./a.out
}

acsub() {
    echo "usage: acsub <language> <problem> <url>"
    rm -rf test
    uv run oj d $3
    if [ $1 = "rs" ]; then
        uv run oj t -c "cargo run --features local --bin $2" || return 1
        uv run python3 ../../util/file_merger.py $2
        # uv run oj s $3 src/bin/$2.rs -y
        uv run python3 ../../util/after_submission.py $2
    elif [ $1 = "py" ]; then
        uv run oj t -c "uv run python3 src/bin/$2.py" || return 1
        # pypyで提出
        # uv run oj s $3 src/bin/$2.py -l 5078
    elif [ $1 = "cpp" ]; then
        g++ $2.cpp -std=c++23 -O2 -W -Wall
        uv run oj t || return 1
        # uv run oj s $3 $2.cpp
	uv run oj-bundle -I/home/ardririy/repos/cp-libs/libraries-cpp -I/usr/local/include/ $2.cpp | clip
	echo "\nAll Tests passed. Code was copied to clipboard!"
    else
        echo "language $1 is not exist!"
    fi
}

