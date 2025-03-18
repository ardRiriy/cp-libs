#!/bin/zsh

gpp(){
	g++ $1 -std=c++23 -O2 -W -Wall || return 1
	./a.out
}

acsub() {
    echo "usage: acsub <language> <problem> <url>"
    rm -rf test
    uv run oj d $3
    if [ $1 = "rs" ]; then
        uv run oj t -c "cargo run --features local --bin $2" || return 1
        uv run python3 ../../util/file_merger.py $2
        uv run oj s $3 src/bin/$2.rs -y
        uv run python3 ../../util/after_submission.py $2
    elif [ $1 = "py" ]; then
        uv run oj t -c "uv run python3 src/bin/$2.py" || return 1
        # pypyで提出
        uv run oj s $3 src/bin/$2.py -l 5078
    else
        echo "language $1 is not exist!"
    fi
}

actest() {
    echo "usage: acsub <language> <problem> <url>"
    rm -rf test
    uv run oj d $3
    if [ $1 = "rs" ]; then
        uv run oj t -c "cargo run --features local --bin $2" || return 1
    elif [ $1 = "py" ]; then
        uv run oj t -c "uv run python3 src/bin/$2.py" || return 1
    else
        echo "language $1 is not exist!"
    fi
}

