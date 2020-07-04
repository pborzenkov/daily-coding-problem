package p0029

import (
	"testing"
)

func TestP0029_encode(t *testing.T) {
	in := "AAAABBBCCDAA"
	want := "4A3B2C1D2A"

	if got := p0029_encode(in); want != got {
		t.Errorf("unexpected output, want = %q, got = %q", want, got)
	}
}

func TestP0029_decode(t *testing.T) {
	in := "4A3B2C1D2A"
	want := "AAAABBBCCDAA"

	if got := p0029_decode(in); want != got {
		t.Errorf("unexpected output, want = %q, got = %q", want, got)
	}
}
