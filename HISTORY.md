# A Short History of Encoding and Base64

_Still researching this, apologies for errors and omissions_

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
