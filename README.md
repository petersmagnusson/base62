# base62

Typescript implementation of base62 encoding and decoding.

'base62' encodes binary data in (pure) alphanumeric format (0-9A-Za-z).

The algorithm encodes in chunks of 32, 16, 8, or 4 bytes and
can correctly reverse the operation.

The only constraint is that any binary input (our output) must be a
multiple of 4 bytes (32 bits). The reason for this is to assure that
the reverse operation is unambiguous.

## Usage

```typescript

    import { arrayBufferToBase62, base62ToArrayBuffer } from 'base62'
    const encoded = arrayBufferToBase62((new TextEncoder).encode('Hello World!'))
    const decoded = new TextDecoder().decode(base62ToArrayBuffer(encoded))
    console.log(decoded)

```

## Efficiency (briefly)

Each b62 character represents log2(62) or about 5.9542 bits. In principle
this would require 0.8% more characters than base64, but in practice
there is often no difference, in particular in crypto contexts.

Notably the resulting encoding lengths are the same for 128, 256, and 512 bits.
Base64 has a "sweetspot" with 192 bits (and multiples thereof such as 384) since
log2(64) has 3 as a prime factor. But for multiples of 192 up to 4x192, the
difference is only one character.

If we look at a larger set of common key sizes (such as 128, 160, 192, 224, 256,
320, 384, and 512) then unless they are a multiple of 192 bits (in the case of
this list 192 and 384), encoding lengths are same.

This is because, curiously, 43xlog2(62) is 256.03, an inefficiency of only 1/8000,
whereas 43xlog2(64) is 258.00, an inefficiency of 1/64, allowing b62 to "catch up".
