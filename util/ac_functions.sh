#!/bin/zsh
util="$(cd "$(dirname "$0")" && pwd)"

acsub() {
  python3 $util/file_marger.py $1
  cargo compete submit $1 || true
  python3 $util/after_submission.py $1
}

actest() {
  cargo compete test $1
}

acinit(){
  cargo compete new $1
  cd $1
  cp ../template.py ./src/bin/a.py
  cp ../template.py ./src/bin/b.py
  cp ../template.py ./src/bin/c.py
}

pt() {
  xsel -b | pypy3 src/bin/$1.py
}