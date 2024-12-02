export default async function solvePart1(r: ReadableStream) {
  let result: number = 0;

  for await (const line of r) {
    const formattedLine: number[] = line
      .split(' ')
      .map((v: string) => Number(v));

    if (lineIsValid(formattedLine)) {
      result += 1;
    }
  }

  console.log(`part 1 solution is ${result}`);
}

function lineIsValid(nList: number[]): boolean {
  let process: string | undefined;
  let prev: number | undefined;

  for (let idx = 1; idx < nList.length; idx++) {
    const cur = nList[idx];
    prev = nList[idx - 1];

    if (idx === 1) {
      process = cur < prev ? 'dec' : 'inc';
    }

    if (!valueIsValid(process, cur, prev)) {
      return false;
    }
  }

  return true;
}

export function valueIsValid(process: string | undefined, cur: number, prev: number | undefined): boolean {
  if (process === undefined || prev === undefined) {
    return false;
  }

  if (process === 'inc' && (cur < prev)) {
    return false;
  }

  if (process === 'dec' && (cur > prev)) {
    return false;
  }

  const dist = Math.abs(cur - prev);
  return dist >= 1 && dist <= 3;
}
