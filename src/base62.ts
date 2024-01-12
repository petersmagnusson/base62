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

export const base62 = '0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz';
export const b62regex = /^[0-9A-Za-z]*$/;

const N = 32; // max chunk size, design point. 

function generateMap(N: number): Map<number, number> {
  const map = new Map<number, number>();
  for (let X = 1; X <= N; X += 1)
    map.set(X, Math.ceil((X * 8) / Math.log2(62)));
  return map;
}

const numberMap = generateMap(N);
const inverseNumberMap = new Map(Array.from(numberMap, ([key, value]) => [value, key]));
const maxChunk = numberMap.get(N)!;

function _arrayBufferToBase62(buffer: ArrayBuffer, c: number): string {
  let result = '';
  for (let n = BigInt('0x' + Array.from(new Uint8Array(buffer)).map(b => b.toString(16).padStart(2, '0')).join(''));
    n > 0n;
    n = n / 62n)
    result = base62[Number(n % 62n)] + result;
  return result.padStart(numberMap.get(c)!, '0');
}

/**
 * Converts any array buffer to base62. Size must be a multiple of 4 bytes.
 */
export function arrayBufferToBase62(buffer: ArrayBuffer): string {
  let l = buffer.byteLength;
  let i = 0;
  let result = '';
  while (l > 0) {
    let c = l >= N ? N : l
    let chunk = buffer.slice(i, i + c);
    result += _arrayBufferToBase62(chunk, c);
    i += c;
    l -= c;
  }
  return result
}

function _base62ToArrayBuffer(s: string, t: number): ArrayBuffer {
  let n = 0n;
  try {
    for (let i = 0; i < s.length; i++) {
      const digit = BigInt(base62.indexOf(s[i]));
      n = n * 62n + digit;
    }
    if (n > 2n ** BigInt(t * 8) - 1n)
      throw new Error(`base62ToArrayBuffer: value exceeds ${t * 8} bits.`);
    const buffer = new ArrayBuffer(t);
    const view = new DataView(buffer);
    for (let i = 0; i < t; i++, n >>= 8n)
      view.setUint8(t - i - 1, Number(n & 0xFFn));
    return buffer;
  } catch (e) {
    console.error("[_base62ToArrayBuffer] Error: ", e); throw (e)
  }
}

/**
 * Converts a base62 string to matching ArrayBuffer.
 */
export function base62ToArrayBuffer(s: string): ArrayBuffer {
  if (!b62regex.test(s)) throw new Error('base62ToArrayBuffer32: must be alphanumeric (0-9A-Za-z).');
  let i = 0, j = 0, c
  let result = new Uint8Array(s.length); // more than we need
  try {
    while (i < s.length) {
      c = (s.length - i) >= maxChunk ? maxChunk : s.length - i
      let chunk = s.slice(i, i + c);
      const newBuf = new Uint8Array(_base62ToArrayBuffer(chunk, inverseNumberMap.get(c)!))
      result.set(newBuf, j);
      i += c;
      j += newBuf.byteLength
    }
    return result.buffer.slice(0, j);
  } catch (e) {
    console.error("[base62ToArrayBuffer] Error:", e); throw (e)
  }
}