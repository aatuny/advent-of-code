import { TextLineStream } from 'jsr:@std/streams@1.0.8/text-line-stream';

export async function getReadableStream() {
  const f = await Deno.open('./input');
  return f.readable
    .pipeThrough(new TextDecoderStream())
    .pipeThrough(new TextLineStream());
}
