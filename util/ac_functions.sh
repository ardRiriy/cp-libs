#!/bin/zsh

acsub() {
    echo "usage: acsub <language> <problem> <url>"
    rm -rf test
    oj d $3
    if [ $1 = "rs" ]; then
        oj t -c "cargo run --bin $2" || return 1
        python3 ../../util/file_merger.py $2
        oj s $3 src/bin/$2.rs -y
        python3 ../../util/after_submission.py $2
    elif [ $1 = "py" ]; then
        oj t -c "python3 src/bin/$2.py" || return 1
        # pypyで提出
        oj s $3 src/bin/$2.py -l 5078
    else
        echo "language $1 is not exist!"
    fi
}

actest() {
    roj test -p $1 -l $2
}

acinit(){
    roj init -c $1
    cd $1
    code .
}
