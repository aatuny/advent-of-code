import { valueIsValid } from './part1.ts';

export default async function solvePart2(r: ReadableStream) {
  let result: number = 0;

  for await (const line of r) {
    const formattedLine: number[] = line
      .split(' ')
      .map((v: string) => Number(v));

    if (lineIsValid(formattedLine)) {
      result += 1;
    }
  }

  console.log(`part 2 solution is ${result}`);
}

function lineIsValid(nList: number[], falseValuesRemoved: boolean = false): boolean {
  let process: string | undefined;
  let prev: number | undefined;

  for (let idx = 1; idx < nList.length; idx++) {
    const cur = nList[idx];
    prev = nList[idx - 1];

    if (idx === 1) {
      process = cur < prev ? 'dec' : 'inc';
    }

    if (!valueIsValid(process, cur, prev)) {
      if (falseValuesRemoved) {
        return false;
      }

      return nList.some((_v1: number, nIdx: number) => lineIsValid(nList.filter((_v: number, nIdx2: number) => nIdx !== nIdx2), true));
    }
  }

  return true;
}
