package p0029

import (
	"fmt"
	"strings"
)

func p0029_encode(str string) string {
	var last rune
	var length int

	out := new(strings.Builder)
	for _, char := range str {
		if char != last && length > 0 {
			fmt.Fprintf(out, "%d%c", length, last)
			length = 0
		}
		last = char
		length += 1
	}
	if length > 0 {
		fmt.Fprintf(out, "%d%c", length, last)
	}

	return out.String()
}

func p0029_decode(str string) string {
	var length int

	out := new(strings.Builder)
	for _, char := range str {
		if char >= '0' && char <= '9' {
			length = length * 10 + int(char - '0')
			continue
		} 
		out.WriteString(strings.Repeat(string(char), length))
		length = 0
	}

	return out.String()
}
