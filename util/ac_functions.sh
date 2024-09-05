root=$(git rev-parse --show-toplevel)

acsub() {
  python3 $root/util/file_marger.py $1
  cargo compete submit $1 || true
  python3 $root/util/after_submission.py $1
}

actest() {
  cargo compete test $1
}

acinit(){
  cargo compete new $1
  cd $1
}
