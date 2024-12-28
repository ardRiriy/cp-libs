import * as fs from "fs";

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

type Student = {
  sum: number,
  idx: number,
}

const [N, K] = nextArray(2);
const p :Student[] = [];
for (let i = 0; i < N; i ++) {
  const points = nextArray(3);
  p[i] = {
    sum: points.reduce((acc, x) => acc + x, 0),
    idx: i,
  };
}

p.sort((a, b) => b.sum - a.sum);
const ans :boolean[] = new Array(N).fill(false);

for (const stu of p) {
  const key = stu.sum + 300;
  let ok = -1;
  let ng = N;

  while(Math.abs(ok-ng) > 1) {
    const mid = Math.ceil((ok + ng) / 2);
    if (key >= p[mid].sum ){
      ng = mid;
    } else {
      ok = mid;
    }
  }
  const place = ok + 2;
  if (place <= K) {
    ans[stu.idx] = true;
  }
}

const res = ans.map(b => b ? "Yes" : "No").join("\n");
console.log(res);
