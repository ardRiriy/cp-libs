#!/bin/zsh
util="$(cd "$(dirname "$0")" && pwd)"

acsub() {
  # contest_name = $(basename "$PWD")
  # number = $1
  # if [ $2 = "rs" ]; then
  python3 $util/file_marger.py $1
  cargo compete submit $1 || true
  python3 $util/after_submission.py $1
}

actest() {
  cargo compete test $1
}

acinit(){
  cargo new $1 --bin
  cd $1

  # rustファイルの準備
  for name in {a..g}; 
  do
    touch "src/bin/${name}.rs"
    cat ../templates/template.rs >> "src/bin/${name}.rs"
  done

  # pythonファイルの準備
  for name in {a..c};
  do
    touch "src/bin/${name}.py"
    cat ../templates/template.py >> "src/bin/${name}.py"
  done
}
