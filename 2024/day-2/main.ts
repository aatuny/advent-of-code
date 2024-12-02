import { getReadableStream } from '../common/utils.ts';

import solvePart1 from './part1.ts';
import solvePart2 from './part2.ts';

if (import.meta.main) {
  await solvePart1(await getReadableStream());
  await solvePart2(await getReadableStream());
}
