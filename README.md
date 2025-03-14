# base62 reference implementation

base62 ``[A-Za-z0-9]`` encoding and decoding. Base62 remedies a few issues that
we've learned about base64 over the years (see below). 

Reference implementation with extensive test suite in TypeScript 3.2 / ECMAScript 2015 (ES6) or later.
Please note that the Go and Rust versions are more or less direct AI translations
of the TypeScript. Please open an issue if you need these improved or if you
need other programming language versions.

    import { arrayBufferToBase62, base62ToArrayBuffer } from 'base62'
    const encoded = arrayBufferToBase62((new TextEncoder).encode('Hello World!'))
    const decoded = new TextDecoder().decode(base62ToArrayBuffer(encoded))
    console.log(decoded)

This algorithm has no restrictions on the input size. The resulting length is
only a function of the length of the input (not the contents). It works
with whole bytes only, using integer modulus operations, in big-endian order,
and default chunk size is 32 bytes. Default character set is base64
compatible ``[A-Za-z0-9]``. Both chunk size and choice of character set 
can be configured.

Performance of base62 is generally worse than base64. This
implementation is fast as base62 systems go, but the focus has been on quality
of encoded results, in particular for smaller sizes, and correctness.

The algorithm is close to theoretical optimum for base62 (eg if the entire
binary content were treated as a single integer). Notably, for several common
smaller sizes of bit strings, optimal base62 encoding results in the same
lengths as base64.



## Background

In contexts where we have a restricted set of characters to
choose from, base64 is suitable for large amounts of binary data, both for density
and speed of encoding/decoding. However, the need to encode
large amounts of binary data in 'printable character' format has
become less of a concern over time, while we have had an increase in
situations where we need to encode small amounts of binary data that are
then directly handled by humans (copy-pasted, or printed, or outright memorized).

The largest set of printable characters that are 
almost uniformly permitted are the 62 alphanumerics (``[A-Za-z0-9]``).

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

Both chunk size and choice of character set is easily modified (there are
a handful of incompatible choices for character set, see below).

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

# Appendix: A Short History of Encoding and Base64

_Apologies for any remaining errors and omissions in this history section,
please let us know if we have missed influences, earlier important work, etc._

A (very) brief history. Base64 as defined today (RFC 4648) traces back to
Privacy Enhanced Mail (PEM) in the early 1990s, which therefore predates
the web (eg URLs etc). PEM was designed to encode (encrypted) binary data as
well as cryptographic keys etc in a format that could be transmitted in
email messages. At the time, the required subsets of US-ASCII was referred
to as "printable characters" - as opposed to control characters, eg the
C0 and C1 sets. US-ASCII in turn dates back to ISO/IEC 646 for 6-bit
character set ... which in turn dates to the 1960s. (Trivia: it also
became ECMA-6, which in fact predates ECMA-9 and ECMA-10 ... which is FORTRAN
and punched tape, respectively.)

You've probably seen things like this:

```
-----BEGIN ENCRYPTED PRIVATE KEY-----
MIHNMEAGCSqGSIb3DQEFDTAzMBsGCSqGSIb3DQEFDDAOBAghhICA6T/51QICCAAw
FAYIKoZIhvcNAwcECBCxDgvI59i9BIGIY3CAqlMNBgaSI5QiiWVNJ3IpfLnEiEsW
Z0JIoHyRmKK/+cr9QPLnzxImm0TR9s4JrG3CilzTWvb0jIvbG3hu0zyFPraoMkap
8eRzWsIvC5SVel+CSjoS2mVS87cyjlD+txrmrXOVYDE+eTgMLbrLmsWh3QkCTRtF
QC7k0NNzUHTV9yGDwfqMbw==
-----END ENCRYPTED PRIVATE KEY----
```

That's a PKCS#8 private key encoded in PEM format (RFC 7468); similarly
for things like X.509 (PKIX) and S/MIME (CMS) certificates, and so on.

Base64 defines the encoding as being done in 3-byte chunks, which is
24 bits, and with log2(64) being 6 bits that means 4 characters per chunk.
The standards also dictate that the base64 encoding must be line-wrapped
at 64 characters. PEM in fact used 76 characters per line, but MIME constrained
this to 64.

Recapitulating some of these steps, we trace the "constrained resource"
nature of 7-bit ASCII. ASCII "control" characters were defined as the
first 32 characters, and the last character was DEL (127). The rest
were "printable" characters (32-126). The alphanumerics (0-9A-Za-z)
are interspersed with a quickly diminishing supply of symbols:

```
 !"#$%&'()*+,-./0123456789:;<=>?
@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_
`abcdefghijklmnopqrstuvwxyz{|}~
```

The first (#32, "space") is excluded quickly from any encoding format,
and the symbols '"' (#34 or DQUOTE), '(' (#40), ')' (#41), ',' (#44),
'.' (#46), ':' (#58), ';' (#59), '<' (#60), '>' (#62), '@' (#64),
'[' (#91), '\\' (#92), and ']' (#93) are all grabbed by RFC 822/2822, for
a total of 14 absorbed back in the day when there seemed to be an infinite
supply of symbols. Not counting '"' (DQUOTE), these are called "specials" in
RFC 822/2822. Some of these can be, and are, resurrected.

SQUOTE ''' and DQUOTE '"' are excluded as they are used as string delimeters
of various types.

At this point we need to mention non-English languages. Though the
Phoeneician alphabet is pretty common, the English language doesn't make
use of diacritics for "meaning", mostly for pronunciation hints - eg cooed
vs coördinate. Other Latin-script languages may use them to distinguish
between homonyms, eg French "ou" (or) vs "où" (where).

This meant that over time, and before the development of ASCII 'extensions',
characters like '^' (#94), '`' (#96), and '~' (#126) became commonly used
for diacritics. This relates to "deadkeys" on typewriters, which were
used to type diacritics. The "deadkey" was a key that didn't print anything
by itself, but modified the next key pressed. For example, on a French
typewriter, the 'a' key would print 'a' by itself, but if the 'a' key was
pressed after the '^' deadkey, it would print 'â'. The 'a' key was also
used for 'à' and 'ä', and the 'e' key was used for 'ê', 'é', 'è', and 'ë'
(to get 'ä', you would press the 'a' key after the '"' deadkey).

Along similar lines, whereas '$' (#36) was used for currency, it is the
dollar. In some cases, '$' would instead become the local currency
symbol such as '£' (#163) for British pounds, but in other cases,
'#' (#35) would used (since US dollar was pretty universal).

So for these reasons, symbols like '^' (#94), '`' (#96), and '~' (#126)
were in practice excluded from "reuse". Similarly, '{' (#123), '|' (#124),
'}' (#125), '[' (#91), '\\' (#92), and ']' (#93) were excluded from
"reuse" as they were often used for 'national' characters, for example
in Swedish keyboards, layouts would map "åÅäÄöÖ" to "{}[]|\\". So a Swedish
programmer had to switch keyboard mode to go between programming and
writing emails.

This is all reflected in International Alphabet No. 5 ("IA5"), which
was later defined in ISO/IEC 646:1991. IA5 is a subset of 7-bit ASCII.

Below I'm using '#' to indicate all these "national" characters, either
because they are undefined in IA5, or because they are used for
diacriticals. (Of course, '#' itself is one such character.)

```
    IA5 character set
       0 1 2 3 4 5 6 7 8 9 A B C D E F
    2x   ! # # # % & # ( ) * + # - . /
    3x 0 1 2 3 4 5 6 7 8 9 : ; < = > ?
    4x # A B C D E F G H I J K L M N O
    5x P Q R S T U V W X Y Z # # # # #
    6x # a b c d e f g h i j k l m n o
    7x p q r s t u v w x y z # # # #
```

The first (#32, "space") is excluded quickly from any encoding format,
and the symbols '"' (#34 or DQUOTE), '(' (#40), ')' (#41), ',' (#44),
'.' (#46), ':' (#58), ';' (#59), '<' (#60), '>' (#62), '@' (#64),
'[' (#91), '\\' (#92), and ']' (#93) are absorbed back in the day.
Not counting '"' (DQUOTE), these are called "specials" in
RFC 822/2822. Some of these would be resurrected.

SQUOTE ''' and DQUOTE '"' are in any case excluded as they are used as
string delimeters of various types.

If we exclude RFC 822 "specials" from the characters that are encoded
the same in ASCII and IA5, we are left with just these symbols:

```
   ! % & * + - / = ?
```

PEM picked their 64-character subset from this - the alphanumerics are
identical in ASCII and IA5, and then they grabbed '/', '+', and '='.
I have not been able to find documentation on why these choices in
particular. But dating back to at least ECMA-1 (1963, see references)
and 6-bit character sets, the only unambiguous symbols were ''( ) * + , - /'',
and '=' and '%' were interchangeable so either could serve as padding
(back then). 

Moving on. With MIME (RFC 1341) we get "tspecials" which are the specials
plus '/' (#47), '?' (#63), and '=' (#61).

So the Base64 standard chooses the same as PEM.

Now comes World Wide Web, starting in 1990. RFC 1630 defines URLs, and
RFC 1738 defines URLs in more detail. The URL syntax is based on RFC 822,
and so inherits the specials and tspecials. We lose '%' (#37)
for escaping, '+' (#43) for spaces, and '#' (#35) for fragment identifiers.
And '!' (#33) and '*' (#42) are reserved for use as having
"special significance" in certain contexts.

With WWW comes HTML, as defined in RFC 1866 and based on SGML
(ISO 8879:1986), and certain characters are treated as special due to their
roles in markup syntax. SGML designates '<' (#60), '>' (#62), and '&' (#38)
as special characters for defining tags and entities. HTML, while
inheriting these special characters from SGML, also commonly uses double
quotes '"' (DQUOTE) and single quotes ''' (SQUOTE) for delimiting attribute
values within element tags, and '#' (#35) to precede numeric character
references.

The language issues were of course intimately understood by the WWW pioneers,
since they were literally based in Geneva. In RFC 1630 the category 'national'
thus includes '{', '}', '|' (VLINE), '[', ']', '\\', '^', and '~'. Though
they don't seem to have been money grubbers so they ignored dual use of '#'.

So to summarize at this point:

```
   symbols = "!" | "%" | "&" | "*" | "+" | "-" | "/" | "=" | "?"
   base64  =                         "+" |       "/" | "="
   URI     = "!" | "%" | "&" | "*" | "+" |     | "/" | "=" | "?"
```

So standards for "content" collide with standards for "addressing" (*).

This gives birth to Base64URL, which replaces '+' with '-', and '/' with '\_'.
Note that '\_' is not ideal, since it was used as national character. But
it was the least bad.

The final collision is with graphical user interfaces, in particular smaller
devices like phones and tablets. Double-clicking will select a "word", which
by convention includes the underscore '\_' but not '-', so double tapping on
a base64url would absorb one of the symbols, but not both. The origin for this
distinction, in turn, is that the underscore is used in programming languages
as a valid character in identifiers, which thus make up "words" in the context
of programming, whereas '-' is not, since that's an operator (minus) and a
math symbol, whereas '\_' had no real corresponding convention in writing.

But of course standard-based base64 decoders do not accept 'base64url'.
Yet because of the above issues, newer standards have been forced to
use 'base64URL' instead of 'base64', eg JSON Web Token (JWT, RFC 7519).
And notably in javascript and web pages, things like btoa(), atob(), and
Data URLs work with standard base64 not base64url. Instead, encodeURIcomponent()
provides percentage escaping of all characters except:

```
A–Z a–z 0–9 - _ . ! ~ * ' ( )
```

Which safeguards any string of characters ... except for a www form
submission spaces should be "+" and "%20" needs to be post-recoded to "+".

And of course IPv6 URI syntax reserves "[" and "]".

And the newest URI standard (RFC 3986) reserves ''! ' ( ) *'' ...

So, yeah, base62 comes in handy.
  
## References

* <https://ecma-international.org/wp-content/uploads/ECMA-1_1st_edition_march_1963.pdf> April 1963, ECMA-1

* <https://datatracker.ietf.org/doc/html/rfc822> August 1982
* <https://datatracker.ietf.org/doc/html/rfc1341> June 1992
* <https://datatracker.ietf.org/doc/html/rfc1630> June 1994
* <https://datatracker.ietf.org/doc/html/rfc1738> December 1994, obsoleted by RFC 3986
* <https://datatracker.ietf.org/doc/html/rfc1866> November 1995
* <https://datatracker.ietf.org/doc/html/rfc2822> Obsoletes RFC 822 (get it? "version 2"? nerds.) April 2001
* <https://datatracker.ietf.org/doc/html/rfc4627> July 2006, obsoleted by RFC 7159
* <https://datatracker.ietf.org/doc/html/rfc4648> October 2006
* <https://datatracker.ietf.org/doc/html/rfc8259> December 2017, obsoletes RFC 7159

* Standard Generalized Markup Language (ISO 8879:1986 SGML) now published under
  <https://www.iso.org/standard/16387.html>

* <https://ecma-international.org/wp-content/uploads/ECMA-262_3rd_edition_december_1999.pdf> December 1999

* <https://datatracker.ietf.org/doc/html/rfc7519> May 2015, JWT

## Footnotes

(*) Something analogous happened in the Middle Ages when arithmetic
and geometry started to merge, and hence standards for "distances"
collided with standards for "surface area", which is the short answer
to the question of why there are 5280 feet in a mile (the surface area
standards were much more important).
