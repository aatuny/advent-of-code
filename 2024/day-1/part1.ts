export default async function solvePart1(r: ReadableStream) {
  const l1: number[] = [];
  const l2: number[] = [];

  for await (const line of r) {
    line
      .split(' ')
      .filter((v: string) => v !== '')
      .forEach((v: string, idx: number) => idx === 0 ? l1.push(Number(v)) : l2.push(Number(v)));
  }

  l1.sort();
  l2.sort();

  const result = l1.map((_, idx) => Math.abs(l1[idx] - l2[idx])).reduce((p, n) => p + n, 0);
  console.log(`part 1 solution is ${result}`);
}
