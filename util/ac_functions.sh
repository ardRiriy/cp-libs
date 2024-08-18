acsub() {
  python3 /Users/ardririy/Work/cp-libs/util/file_marger.py $1
  cargo compete submit $1 || true
  python3 /Users/ardririy/Work/cp-libs/util/after_submission.py $1
}

actest() {
  cargo compete test $1
}

acinit(){
  cargo compete new $1
  cd $1
}
