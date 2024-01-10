/*

    (c) 2023-2024, 384 (tm) Inc.

    This program is free software: you can redistribute it and/or
    modify it under the terms of the GNU Affero General Public License
    as published by the Free Software Foundation, either version 3 of
    the License, or (at your option) any later version.

    This program is distributed in the hope that it will be useful, but
    WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
    Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public
    License along with this program.  If not, see www.gnu.org/licenses/

    */


// Define the base62 dictionary (alphanumeric)
// We want the same sorting order as ASCII, so we go with 0-9A-Za-z
export const base62 = '0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz';
const b62regex = /^[0-9A-Za-z]*$/;

const intervals = new Map<number, number>([
  [32, 43],
  [16, 22],
  [8, 11],
  [4, 6],
]);
const inverseIntervals = new Map(Array.from(intervals, ([key, value]) => [value, key]));
const inverseKeys = Array.from(inverseIntervals.keys()).sort((a, b) => a - b);

function _arrayBufferToBase62(buffer: ArrayBuffer, c: number): string {
  if (buffer.byteLength !== c || !intervals.has(c)) throw new Error("[arrayBufferToBase62] Decoding error")
  let result = '';
  for (let n = BigInt('0x' + Array.from(new Uint8Array(buffer)).map(b => b.toString(16).padStart(2, '0')).join(''));
    n > 0n;
    n = n / 62n)
    result = base62[Number(n % 62n)] + result;
  return result.padStart(intervals.get(c)!, '0');
}

/**
 * Converts any array buffer to base62. Size must be a multiple of 4 bytes.
 */
export function arrayBufferToBase62(buffer: ArrayBuffer): string {
  let L = buffer.byteLength, i = 0, result = '';
  if (L % 4 !== 0) throw new Error('arrayBufferToBase62: buffer size must be a multiple of 4 bytes.');
  while (L > 0) {
    let c = 2 ** Math.min(Math.floor(Math.log2(L)), 5); // next chunk
    let chunk = buffer.slice(i, i + c);
    result += _arrayBufferToBase62(chunk, c);
    i += c;
    L -= c;
  }
  return result
}

// t is number of (8-bit) bytes and either 32, 16, 8, or 4
function _base62ToArrayBuffer(s: string, t: number): ArrayBuffer {
  let n = 0n;
  try {
    for (let i = 0; i < s.length; i++) {
      const digit = BigInt(base62.indexOf(s[i]));
      n = n * 62n + digit;
    }
    if (n > 2n ** BigInt(t * 8) - 1n) // check overflow
      throw new Error(`base62ToArrayBuffer: value exceeds ${t * 8} bits.`);
    const buffer = new ArrayBuffer(t);
    const view = new DataView(buffer);
    for (let i = 0; i < (t / 4); i++) {
      const uint32 = Number(BigInt.asUintN(32, n));
      view.setUint32(((t / 4) - i - 1) * 4, uint32);
      n = n >> 32n;
    }
    return buffer;
  } catch (e) {
    console.error("[_base62ToArrayBuffer] Error: ", e); throw (e)
  }
}

/**
 * Converts a base62 string to matching ArrayBuffer. The original (and hence resulting)
 * array buffer size must have been a multiple of 4 bytes.
 */
export function base62ToArrayBuffer(s: string): ArrayBuffer {
  if (!b62regex.test(s)) throw new Error('base62ToArrayBuffer32: must be alphanumeric (0-9A-Za-z).');
  let i = 0, j = 0, c, oldC = 43
  let result = new Uint8Array(s.length); // more than we need
  try {
    while (i < s.length) {
      c = inverseKeys.filter(num => num <= (s.length - i)).pop()!;
      if (oldC < 43 && c >= oldC) throw new Error('invalid b62 (original size was not a multiple of 4 bytes)')
      oldC = c // decoding check: other than with 43, should be decreasing
      let chunk = s.slice(i, i + c);
      const newBuf = new Uint8Array(_base62ToArrayBuffer(chunk, inverseIntervals.get(c)!))
      result.set(newBuf, j);
      i += c;
      j += newBuf.byteLength
    }
    return result.buffer.slice(0, j);
  } catch (e) {
    console.error("[base62ToArrayBuffer] Error:", e); throw (e)
  }
}

