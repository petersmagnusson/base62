# base62

base62 encoding and decoding. Typescript implementation.

'base62' encodes binary data in (pure) alphanumeric format (0-9A-Za-z).

The algorithm has no restrictions on the input. The resulting length is
only a function of the length of the input (not the contents).

The algorithm is close to theoretical optimum for b62 - eg if the entire
binary content were treated as one integer: for input sizes less than
812 bytes, it is optimal; for input sizes 812-7051 bytes, it is at most
one character longer than theoretical optimum.

For several smaller sizes the encoding is the same length as base64.

Performance is much worse than base64, and this format is not yet suitable
for large amounts of data.

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

The algorithm chunks to 32 bytes or smaller. This dramatically improves performance
compared to larger chunks, with minimal impact on quality - in fact you would
need to go to chunk sizes well above 512 bytes to see much difference.

## Issues with Base64

TBW.

## Other Implementations

Unfortunately, there is no standard for base62, so various implementations
that are in circulation are not compatible.

Differences include:

* The character set used (0-9A-Za-z vs 0-9a-zA-Z). We use the former, since
  that corresponds to sorting order in ASCII, but some implementations use
  the latter.

* Many "base62" implementations only encode a number, not an arbitrary
  binary object.

* Some approaches lead to variable-length encoding, eg the length
  of the result depends on the contents of the input (*). For various reasons,
  in many cases this is not desirable - the length of the base62 output
  should be predictable from the length of the (binary) input.

* Few (if any) approaches appear to be close to the theoretical optimum
  for base62 (at least from the limited testing we've done).

Implementations we are currently looking for comparisons include:

* https://github.com/marksalpeter/token/tree/master/v2 - token/uin64 only

* https://github.com/glowfall/base62 - variable length and non-optimal results.
  Some examples (results are formatted as bufferSize:min/avg/max):

```text
     16:   22 /    22.01 /   23 (optimum is   22)
     32:   43 /    43.5  /   45 (optimum is   43)
     40:   54 /    54.24 /   56 (optimum is   54)
     64:   86 /    86.65 /   89 (optimum is   89)
   2048: 2751 /  2759.82 / 2769 (optimum is 2752) (*)
   4096: 5507 /  5519.23 / 5534 (optimum is 5504)
   6240: 8393 /  8408    / 8428 (optimum is 8385)
   5280: 7101 /  7114.5  / 7129 (optimum is 7138)

* https://github.com/keybase/saltpack/tree/master/encoding/basex

* https://github.com/eknkc/basex

  
(*) Variable length output means that it's possible some inputs will
result encodings that are shorter than what is 'theoretically possible'.
Since binary data in these encoding contexts are typically 'random',
taking any sort of compression approach for base62 is a bit misguided.
  
## History

Separate document will be added.

## References

To be added.
