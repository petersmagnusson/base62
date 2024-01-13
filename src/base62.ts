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

// there are three common variations.
const base62b64 = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
const base62lex = '0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz';
const base62baseN = '0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';

// we use 'base64' ordering, also known as 'truncated base64'
const base62 = base62b64;
const b62regex = /^[0-9A-Za-z]*$/;
export { base62, b62regex, base62b64, base62lex, base62baseN };

// if you wish to be interoperable, you can append '62' to the end of the Uint8Array before
// encoding it, that will cause the string to end with 'M' (b64), 'C' (lex), or 'c' (baseN)

const N = 32; // max chunk size, design point. 

const M = new Map<number, number>(), invM = new Map<number, number>();
for (let X = 1; X <= N; X++) {
  const Y = Math.ceil((X * 8) / Math.log2(62));
  M.set(X, Y);
  invM.set(Y, X);
}
const maxChunk = M.get(N)!; // this will be 43

function _arrayBufferToBase62(buffer: Uint8Array, c: number, b62 = base62): string {
  let result = '', n = 0n, b62zero = b62[0];
  for (const byte of buffer)
    n = (n << 8n) | BigInt(byte);
  for (; n > 0n; n = n / 62n)
    result = b62[Number(n % 62n)] + result;
  return result.padStart(M.get(c)!, b62zero);
}

/** Converts any array buffer to base62. */
export function arrayBufferToBase62(buffer: ArrayBuffer | Uint8Array, b62 = base62): string {
  const buf = buffer instanceof ArrayBuffer ? new Uint8Array(buffer) : buffer
  let result = '';
  for (let l = buf.byteLength, i = 0, c; l > 0; i += c, l -= c) {
    c = l >= N ? N : l;
    result += _arrayBufferToBase62(buf.slice(i, i + c), c, b62);
  }
  return result;
}

function _base62ToArrayBuffer(s: string, t: number, b62 = base62): Uint8Array {
  try {
    let n = 0n, buffer = new Uint8Array(t);
    for (let i = 0; i < s.length; i++)
      n = n * 62n + BigInt(b62.indexOf(s[i]));
    if (n > 2n ** BigInt(t * 8) - 1n)
      throw new Error('base62ToArrayBuffer: Invalid Base62 string.'); // exceeds (t * 8) bits
    for (let i = t - 1; i >= 0; i--, n >>= 8n)
      buffer[i] = Number(n & 0xFFn);
    return buffer;
  } catch (e) {
    throw new Error('base62ToArrayBuffer: Invalid Base62 string.'); // 'NaN' popped up
  }
}

/** Converts a base62 string to matching ArrayBuffer. */
export function base62ToArrayBuffer(s: string, b62 = base62): ArrayBuffer {
  if (!b62regex.test(s)) throw new Error('base62ToArrayBuffer32: must be alphanumeric.');
  try {
    let j = 0, result = new Uint8Array(s.length * 6 / 8); // we know we're less than 6
    for (let i = 0, c, newBuf; i < s.length; i += c, j += newBuf.byteLength) {
      c = Math.min(s.length - i, maxChunk);
      newBuf = _base62ToArrayBuffer(s.slice(i, i + c), invM.get(c)!, b62)
      result.set(newBuf, j);
    }
    return result.buffer.slice(0, j);
  } catch (e) { throw e; }
}
