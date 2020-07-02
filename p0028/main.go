package p0028

import "strings"

func p0028(words []string, k int) []string {
	var result []string

	var line []string
	var length int
	for i := 0; i < len(words); {
		w := words[i]

		if length+len(line)+len(w) <= k {
			line = append(line, w)
			length += len(w)
			i += 1
			if i < len(words) {
				continue
			}
		}

		places := len(line) - 1
		spaces := k - length
		var r string
		for _, w := range line {
			var toInsert int

			r += w
			if places > 0 {
				toInsert += spaces / places
				if spaces%places != 0 {
					toInsert += 1
				}
			} else {
				toInsert = spaces
			}
			r += strings.Repeat(" ", toInsert)

			spaces -= toInsert
			places -= 1
		}
		result = append(result, r)

		line = make([]string, 0)
		length = 0
	}

	return result
}
