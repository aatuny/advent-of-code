export default async function solvePart2(r: ReadableStream) {
  const l1: number[] = [];
  const l2: number[] = [];

  for await (const line of r) {
    line
      .split(' ')
      .filter((v: string) => v !== '')
      .forEach((v: string, idx: number) => idx === 0 ? l1.push(Number(v)) : l2.push(Number(v)));
  }

  const result = l1.map((v) => l2.filter((n) => n === v).length * v).reduce((p, n) => p + n, 0);
  console.log(`part 2 solution is ${result}`);
}
