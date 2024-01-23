# base62

base62 (A-Za-z0-9) encoding and decoding. Typescript implementation.

For several smaller sizes, base62 encoding is the same length as base64,
including many sizes relevant in crypto contexts.

This algorithm has no restrictions on the input. The resulting length is
only a function of the length of the input (not the contents).

The algorithm is close to theoretical optimum for base62 - eg if the entire
binary content were treated as a single integer. For input sizes less than
227 bytes, it is optimal; if we restrict input sizes to be a multiple of
4 bytes (32 bits), then for sizes up to 812 bytes it is optimal, and for
sizes up to 7051 bytes it is behind optimum by at most one character.

Performance of base62 is generally much worse than base64. This
implementation is fairly fast, but the focus has been on optimality
of encoding, in particular for smaller sizes, and correctness.

```typescript

    import { arrayBufferToBase62, base62ToArrayBuffer } from 'base62'
    const encoded = arrayBufferToBase62((new TextEncoder).encode('Hello World!'))
    const decoded = new TextDecoder().decode(base62ToArrayBuffer(encoded))
    console.log(decoded)

The algorithm encodes and decodes data using a base62 encoding scheme, with a defined chunk size of 32 bytes. Each chunk, up to 32 bytes, is first converted into a BigInt (eg up to 2^256-1 in size), and then iteratively divided by 62 to encode it into a base62 string, zero-padded with the character representing zero in our base62 dictionary ('A'). We maintain maps (M and invM) to correlate the length of byte sequences with their corresponding base62 string lengths, and vice versa. The algorithm operates in big-endian format. It includes checks to validate the correctness of the base62 strings, ensuring they are valid outputs of the base62 encoding process.

## Efficiency (briefly)

Generally for base62, each character represents log2(62) or about 5.9542 bits. In principle
this would require 0.8% more characters than base64, but in practice
there is often no difference, in particular in crypto contexts.

Notably the resulting encoding lengths are the same for 128, 256, and 512 bits.
Base64 has a "sweetspot" with 192 bits (and multiples thereof such as 384) since
log2(64) has 3 as a prime factor. But even then, for multiples of 192 up to 4x192, the
difference is only one character.

If we look at a larger set of common key sizes (such as 128, 160, 192, 224, 256,
320, 384, and 512) then unless they are a multiple of 192 bits (in the case of
this list 192 and 384), encoding lengths are same.

This is because, curiously, 43xlog2(62) is 256.03, an inefficiency of only 1/8000,
whereas 43xlog2(64) is 258.00, an inefficiency of 1/64, allowing b62 to "catch up".

Hence the chunking to 32 bytes or smaller. This dramatically improves performance
compared to larger chunks, of course, and with minimal impact on quality - in fact you would
need to go to chunk sizes well above 512 bytes to see much difference. Conversely,
smaller chunks lead to significantly worse encoding.

## Issues with Base64

TBW.

## Other Implementations

Unfortunately, there is no standard for base62, so various implementations
that are in circulation are not compatible.

Differences include:

* The character set used, or rather, the order of the characters. Unfortunately,
  four variations are in circulation: Base64 ordering (A-Za-z0-9), lexicographic
  (ASCII) order (0-9A-Za-z), "BaseN" (**) ordering (0-9a-zA-Z), and finally
  but least commonly (a-zA-Z0-9).

* Many "base62" implementations only encode a number, not an arbitrary
  binary object.

* Some approaches lead to variable-length encoding, eg the length
  of the result depends on the contents of the input (*). For various reasons,
  in many cases this is not desirable - the length of the base62 output
  should be predictable from the length of the (binary) input.

* Few (if any) approaches appear to be close to the theoretical optimum
  for base62 (at least from the limited testing we've done).

Implementations we are currently looking at for comparison include
(this list will grow as we find more, then hopefully curated down
to keep 'canonical' implementations for different approaches):

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

* https://github.com/suminb/base62
  Variable results, not guaranteed optimal. For example, byte length 32
  results in 42, 43, or 44 characters.

* https://github.com/keybase/saltpack/tree/master/encoding/basex

* https://github.com/eknkc/basex

* https://github.com/KvanTTT/BaseNcoding

* https://github.com/seruco/base62

* https://github.com/jxskiss/base62



## Footnotes
  
(*) Variable length output means that it's possible some inputs will
result encodings that are shorter than what is 'theoretically possible'.
Since binary data in these encoding contexts are typically 'random',
taking any sort of compression approach for base62 is a bit misguided.

(**) 'BaseN' approaches will prefer to pick 'less ambiguous' characters,
and in that contexts lowercase is considered preferable. Of course, base62
is precisely the point on that continuum where both lower and upper case
are included, and no additional symbols beyond alphanumeric. But to our
knowledge, common 'baseN' code do not 'catch' this special case for base62.
The Base64 ordered version of Base62 is sometimes referred to as 'truncated
'base64'. To help clarify things, Wikipedia has chimed in and kept changing
the order of the characters in their article on base62. The first table
added to the 'base62' article was in 2020, and that showed A-Za-z0-9.
then in 2021 it was changed to 0-9A-Za-z, then some edit wars and it was
changed back and forth a few times, eventually back to the current
0-9a-zA-Z, at no point does the article appear to have mentioned that
there are in fact multiple versions and no standard.

## History

Separate document will be added.

## References

To be added.
