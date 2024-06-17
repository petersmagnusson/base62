
// Please note, this is a rudimentary translation of the Typescript version.
// Happy to accept PRs to improve this code.

package base62

import (
	"errors"
	"math"
	"math/big"
	"regexp"
	"strings"
)

const (
	base62b64   = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
	base62lex   = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
	base62baseN = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
	b62         = base62b64 // using 'base64' ordering, also known as 'truncated base64'
	N           = 32        // max chunk size, design point
)

var (
	M        = make(map[int]int)
	invM     = make(map[int]int)
	maxChunk int
	b62regex = regexp.MustCompile("^[0-9A-Za-z]*$")	
)

func init() {
	for X := 1; X <= N; X++ {
		Y := int(math.Ceil(float64(X*8) / math.Log2(62)))
		M[X] = Y
		invM[Y] = X
	}
	maxChunk = M[N]
}

func _arrayBufferToBase62(buffer []byte, c int, b62 string) string {
	n := big.NewInt(0)
	b62zero := b62[0]
	result := ""
	for _, b := range buffer {
		n.Lsh(n, 8)
		n.Or(n, big.NewInt(int64(b)))
	}
	bigBase := big.NewInt(62)
	mod := &big.Int{}
	for n.Cmp(big.NewInt(0)) > 0 {
		n.DivMod(n, bigBase, mod)
		result = string(b62[mod.Int64()]) + result
	}
	return strings.Repeat(string(b62zero), M[c]-len(result)) + result
}

// ArrayBufferToBase62 converts any array buffer to base62.
func ArrayBufferToBase62(buffer []byte) string {
	result := ""
	bufLen := len(buffer)
	for i := 0; i < bufLen; {
		c := N
		if bufLen-i < N {
			c = bufLen - i
		}
		result += _arrayBufferToBase62(buffer[i:i+c], c, b62)
		i += c
	}
	return result
}

func _base62ToArrayBuffer(s string, t int, b62 string) ([]byte, error) {
	n := big.NewInt(0)
	buffer := make([]byte, t)
	bigBase := big.NewInt(62)
	for i := 0; i < len(s); i++ {
		n.Mul(n, bigBase)
		n.Add(n, big.NewInt(int64(strings.IndexRune(b62, rune(s[i])))))
	}
	maxValue := new(big.Int).Lsh(big.NewInt(1), uint(t*8))
	if n.Cmp(maxValue) >= 0 {
		return nil, errors.New("base62ToArrayBuffer: Invalid Base62 string.") // exceeds (t * 8) bits
	}
	for i := t - 1; i >= 0; i-- {
		buffer[i] = byte(n.Int64() & 0xFF)
		n.Rsh(n, 8)
	}
	return buffer, nil
}

// Base62ToArrayBuffer converts a base62 string to matching array buffer.
func Base62ToArrayBuffer(s string) ([]byte, error) {
	if !b62regex.MatchString(s) {
		return nil, errors.New("base62ToArrayBuffer: must be alphanumeric.")
	}
	result := make([]byte, len(s)*6/8)
	var j int
	for i := 0; i < len(s); {
		c := maxChunk
		if len(s)-i < maxChunk {
			c = len(s) - i
		}
		newBuf, err := _base62ToArrayBuffer(s[i:i+c], invM[c], b62)
		if err != nil {
			return nil, err
		}
		copy(result[j:], newBuf)
		j += len(newBuf)
		i += c
	}
	return result[:j], nil
}
