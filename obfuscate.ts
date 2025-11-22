#! /usr/bin/env bun
// https://github.com/altcha-org/altcha/blob/main/scripts/obfuscate.ts
/*
  MIT License

  Copyright (c) 2023 Daniel Regeci, BAU Software s.r.o.

  Permission is hereby granted, free of charge, to any person obtaining a copy
  of this software and associated documentation files (the "Software"), to deal
  in the Software without restriction, including without limitation the rights
  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
  copies of the Software, and to permit persons to whom the Software is
  furnished to do so, subject to the following conditions:

  The above copyright notice and this permission notice shall be included in all
  copies or substantial portions of the Software.

  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
  SOFTWARE.
*/
/**
 * Run this script with Bun:
 * 
 * bun run scripts/obfuscate.ts "mailto:..."
 * 
 * or with Node:
 *
 * npx tsx scripts/obfuscate.ts "mailto:..."
 */

const MAX_NUMBER = parseInt(process.env.MAX_NUMBER || '10000', 10);
const NUMBER = process.env.NUMBER ? parseInt(process.env.NUMBER || '0', 10) : undefined;
const KEY = process.env.KEY || '';

function randomInt(max: number) {
  const ab = new Uint32Array(1);
  crypto.getRandomValues(ab);
  const randomNumber = ab[0] / (0xffffffff + 1);
  return Math.floor(randomNumber * max + 1);
}

async function uInt8ArrayToBase64(ua: Uint8Array) {
  return Buffer.from(ua).toString('base64');
}

function numberToUint8Array(num: number, len: number = 12) {
  const ua = new Uint8Array(len);
  for (let i = 0; i < len; i++) {
    ua[i] = num % 256;
    num = Math.floor(num / 256);
  }
  return ua;
}

async function obfuscateData(
  raw: string,
  key: string = '',
  number: number = NUMBER || randomInt(MAX_NUMBER),
) {
  const encoder = new TextEncoder();
  const encodedData = encoder.encode(raw);
  const algorithm = { name: 'AES-GCM', iv: numberToUint8Array(number) };
  const keyHash = await crypto.subtle.digest(
    'SHA-256',
    encoder.encode(key)
  );
  const keyData = await crypto.subtle.importKey(
    'raw',
    keyHash,
    algorithm,
    false,
    ['encrypt']
  );
  const encryptedData = await crypto.subtle.encrypt(
    algorithm,
    keyData,
    encodedData
  );
  return uInt8ArrayToBase64(new Uint8Array(encryptedData));
}

console.log(await obfuscateData(process.argv[process.argv.length - 1], KEY));

export {};