package p0028

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func TestP0028(t *testing.T) {
	got := p0028(
		[]string{"the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog", "again"},
		16,
	)

	want := []string{
		"the  quick brown",
		"fox  jumps  over",
		"the   lazy   dog",
		"again           ",
	}

	if cmp.Diff(want, got) != "" {
		t.Errorf("unexpected output, diff = \n%s", cmp.Diff(want, got))
	}
}
