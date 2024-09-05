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

const [n, x] = nextArray(2);
const a = nextArray(n);

const check = new Array(n).fill(false);
let current = x-1;

while(!check[current]) {
  check[current] = true;
  current = a[current]-1;
}

console.log(check.filter(b => b).length);
