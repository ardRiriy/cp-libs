import * as fs from "fs";
import { exit } from "process";

// 入力の処理
const inputs = fs.readFileSync("/dev/stdin", "utf8");
const inputs_array = inputs.split(/\s/);
let inputs_array_idx = 0;
function next() {
  return inputs_array[inputs_array_idx++];
}
function nextNum() {
  return Number(next());
}
function nextArray(N: number) {
  const res = [];
  for (let i = 0; i < N; i++) res[i] = nextNum();
  return res;
}

const [s, t, x] = nextArray(3);
let c = s;

while(c != t) {
  if(c == x) {
    console.log("Yes");
    exit(0);
  }
  c = (c + 1) % 24;
}
console.log("No");
