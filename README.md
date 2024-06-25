# base62. Arbitrary inputs, deterministic and close to optimal output. Typescript, Go, and Rust versions.

base62 ``[A-Za-z0-9]`` encoding and decoding. Extensive test suite.

This algorithm has no restrictions on the input. The resulting length is
only a function of the length of the input (not the contents). It works
with whole bytes only, using integer modulus operations, in big-endian order,
and default chunk size is 32 bytes. Default character set is base64
compatible ``[A-Za-z0-9]``. Both chunk size and choice of character set 
can be configured.

Performance of base62 is generally much worse than base64. This
implementation is fairly fast, but the focus has been on optimality
of encoding, in particular for smaller sizes, and correctness.

The algorithm is close to theoretical optimum for base62 (eg if the entire
binary content were treated as a single integer).

TypeScript 3.2 / ECMAScript 2015 (ES6) or later.

Please note that the Go and Rust versions are direct translations of the TypeScript.
Happy to take PRs to improve them.

## Background

In contexts where we have restricted set of characters to
choose from, base64 is suitable for large amounts of binary data, both for density
and speed of encoding/decoding. However, the need to encode
large amounts of binary data in 'printable character' format has
become less of a concern over time, while we have had an increase in
situations where we need to encode smaller amounts of binary data.
The largest set of printable characters that are 
permitted in most common contexts is alphanumerics.

Notably, for several common smaller sizes of bit strings, optimal
base62 encoding results in the same lengths of characters as base64.

Usage:

    import { arrayBufferToBase62, base62ToArrayBuffer } from 'base62'
    const encoded = arrayBufferToBase62((new TextEncoder).encode('Hello World!'))
    const decoded = new TextDecoder().decode(base62ToArrayBuffer(encoded))
    console.log(decoded)

The algorithm encodes and decodes data using a base62 encoding scheme,
with a preferred chunk size of 32 bytes. Each chunk is
first converted into a BigInt (eg up to 2^256-1 in size), and then
iteratively divided by 62 to encode it into a base62 string, zero-padded
with the character representing zero in our default base62 character set ('A'). We
maintain maps (M and invM) to correlate the length of byte sequences with
their corresponding base62 string lengths, and vice versa. The algorithm
operates in big-endian format. It includes checks to validate the correctness
of the base62 strings, ensuring they are valid outputs of the same base62
encoding process.

As noted above, both chunk size and choice of character set is easily modified.

## Efficiency (briefly)

Generally for base62, each character represents log2(62) or about 5.9542 bits. In principle
this would require 0.8% more characters than base64, but in practice
there is often no difference, in particular in crypto contexts.

Notably the resulting encoding lengths are the same for 128, 256, and 512 bits.
Base64 has a "sweetspot" with 192 bits (and multiples thereof such as 384) since
log2(64) has 3 as a prime factor. But even then, for multiples of 192 up to 4x192, the
difference is only one character.

In fact, for bit lengths that are multiples of 32 (4 bytes),
unless the bit length is also evenly divisible by 3 (in other words unless the total
amount of data in bits is divisible by 96), then b62 and b64 result
in the same encoding lengths for all cases shorter than 352 bits.

If we look at a larger set of common key sizes (such as 128, 160, 192, 224, 256,
320, 384, and 512) then unless they are a multiple of 192 bits (in the case of
this list 192 and 384), encoding lengths are same.

This is because, curiously, 43xlog2(62) is 256.03, an inefficiency of only 1/8000,
whereas 43xlog2(64) is 258.00, an inefficiency of 1/64, allowing b62 to "catch up".

Hence the default chunking of 32 bytes. This dramatically improves performance
compared to larger chunks, with minimal impact on quality. In fact you would
need to go to chunk sizes well above 512 bytes to see much difference. Conversely,
smaller chunks lead to significantly worse encoding.

Given our chunking of 256 bits, if we compare with theoretically optimal b62
encoding, and we as above restrict input sizes
to be a multiples of 4 bytes (32 bits), then for sizes up to 812 bytes
(6496 bits) this algorithm is optimal, and for sizes up to 7052 bytes (56416 bits)
it is behind optimum by at most one character.

## Issues with Base64

Base64 is the main standard for encoding binary data in printable format.
There are only 62 alphanumeric (A-Za-z0-9) characters, so any base64
design needs to pick two symbols. For historical reasons (see the separate
"HISTORY.md" document), Base64 uses '+' and '/', and '=' for padding.

Unfortunately, these choices predate the World Wide Web. Uniform Resource
Identifiers (URIs) reserve all of those symbols. Base64 can work without '=',
but URIs reserve '+' for spaces and '/' for path separators.

This leads to Base64URL, which uses '-' and '_' instead of '+' and '/',
allowing the encoding to be used in URIs. But this is not a standard,
so for example, web APIs like atob() and btoa() in browsers do not support
Base64URL. Conversely, JWT (JSON Web Tokens), which itself is a standard,
uses Base64URL.

This in turn leads to things like encodeURIComponent() and decodeURIComponent()
being applied to Base64 tokens - sometimes. This bridging between Base64
and Base64URL depending on context, or wrapping on or the other, is a constant
source of bugs and confusion.

Historically, binary-data-as-readable-string was something that occurred
"internally" in systems, for example to handle arbitrary binary attachments
to email. But with the rise of various cryptographic features, items
like tokens and keys often parts of "text" that an end-user is directly
manipulating - copying, pasting, etc.

Whereas the earlier issues mostly impact developers, the
symbols '-' and '_' introduce issues for users. Especially on
mobile devices, "selecting" parts of text can be difficult. The symbols '-' and '_'
are treated differently: '-' is generally treated as a word separator,
whereas '_' is not. Thus, for example, a 256-bit value encoded as Base64URL
may or may not include '-', in fact it's about a 50/50 chance. So half the time
a user needs to copy-paste such a token, they can just double-tap to select
all of the characters, and about half the time they can't.

Base62 has always existed as an option, but since it amounts to encoding
using fractional bits, it has two challenges: it is likely to be much
slower, and there are corner cases that would require general agreement.

Two things have changed in recent years. First, BigInt support is now
pervasive in programming languages, so can be viewed as a primitive
in any common environment, and for the situations where end-users are
directly invovled, by definition the amount of data is small.
Secondly, the universe of encodings have
been so expanded, that, increasingly, environments have ways of expressing
what encoding is being used (e.g. https://github.com/multiformats/multibase).

So it would seem that the obvious approach is to use Base64 wherever it
involves large amounts of data, and Base64URL wherever interoperability
with standards like JWT is required, and Base62 for anything that is
exposed to end-users. 

## Other Implementations

Unfortunately, there is no standard for base62, so various implementations
that are in circulation are often not compatible.

Differences include:

* The character set used, or rather, the order of the characters. Unfortunately,
  four variations are in circulation: Base64 ordering (A-Za-z0-9), lexicographic
  (ASCII) order (0-9A-Za-z), "BaseN" (**) ordering (0-9a-zA-Z), and finally
  but least commonly (a-zA-Z0-9). We chose 'Base64 ordering' to be aligned
  with the base64 standard (e.g. A-Za-z0-9).

* Many "base62" implementations only encode a number, not an arbitrary
  binary object.

* Some approaches lead to variable-length encoding, eg the length
  of the result depends on the contents of the input (*). For various reasons,
  in many cases this is not desirable - the length of the base62 output
  should be predictable from the length of the (binary) input.

* Few (if any) approaches appear to be close to the theoretical optimum
  for base62 (at least from the limited testing we've done).

Implementations we are currently looking at for comparison include
the below. This list will grow as we find more, then hopefully curated down
to keep 'canonical' implementations for different approaches. Let us
know what we're missing. Principal programming language is in parentheses.

* (Go) https://github.com/marksalpeter/token/tree/master/v2 - token/uin64 only

* (Java) https://github.com/glowfall/base62 - base64 ordering, variable length
  with non-optimal results. Some examples (results are formatted as bufferSize:min/avg/max):

```
     16:   22 /    22.01 /   23 (optimum is   22)
     32:   43 /    43.5  /   45 (optimum is   43)
     40:   54 /    54.24 /   56 (optimum is   54)
     64:   86 /    86.65 /   89 (optimum is   89)
   2048: 2751 /  2759.82 / 2769 (optimum is 2752) (*)
   4096: 5507 /  5519.23 / 5534 (optimum is 5504)
   6240: 8393 /  8408    / 8428 (optimum is 8385)
   5280: 7101 /  7114.5  / 7129 (optimum is 7138)
```

* (Python) https://github.com/suminb/base62
  Variable results, not guaranteed optimal. For example, byte length 32
  results in 42, 43, or 44 characters.

* (Go) https://github.com/keybase/saltpack/tree/master/encoding/basex

* (Go) https://github.com/eknkc/basex
  Generic base. A port of https://github.com/cryptocoinjs/base-x (from JavaScript),
  which in turn is a derivation of bitcoin/src/base58.cpp (generalized for variable alphabets).
  For base62 uses (0-9a-zA-Z).

* (C# and Javascript) https://github.com/KvanTTT/BaseNcoding

* (Java) https://github.com/seruco/base62

* (Go) https://github.com/jxskiss/base62
  Inspired by glowfall. Variadic length encoding, not optimal but avoids bigint.

* (Rust) https://github.com/fbernier/base62



## Footnotes
  
(*) Variable length output means that it's possible some inputs will
result in encodings that are shorter than what is 'theoretically possible'.
Since binary data in these encoding contexts are typically 'random',
taking any sort of compression approach for base62 doesn't lead to
any benefits on efficiency (to the contrary), but allows for faster algorithms.
A common approach is a sliding mask to decide on encoding either five
bits or six bits at a time.

(**) 'BaseN' approaches will prefer to pick 'less ambiguous' characters,
and in that context, lowercase is considered preferable. Of course, base62
is precisely the point on that continuum where both lower and upper case
are included, and no additional symbols beyond alphanumeric. But to our
knowledge, common 'baseN' implementations do not 'catch' this special case for base62.
The Base64 ordered version of Base62 is sometimes referred to as 'truncated
base64'. To (not) help clarify things, the Wikipedia article on base62 has chimed in
with different versions depending on the year. The first table
added to the 'base62' article was in 2020, and that showed A-Za-z0-9.
Then in 2021 it was changed to 0-9A-Za-z, then some edit wars, it was
changed back and forth a few times, currently it's showing 0-9a-zA-Z.
At no point does the article appear to have mentioned that
there are in fact multiple versions (and no standard).


## History

See the separate HISTORY document.
