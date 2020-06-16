package p0011

import (
	"testing"
	"sort"

	"github.com/google/go-cmp/cmp"
)

func equal(t *testing.T, want, got []string) {
	t.Helper()

	sort.Strings(want)
	sort.Strings(got)

	if cmp.Diff(want, got) != "" {
		t.Errorf("unexpected result, diff = \n%s", cmp.Diff(want, got))
	}
}

func TestExample(t *testing.T) {
	c := NewCompleter([]string{"dog", "deer", "deal"})
	equal(t, []string{"deer", "deal"}, c.Complete("de"))
}

func TestExact(t *testing.T) {
	c := NewCompleter([]string{"dog", "dogfood", "cat", "dogma"})
	equal(t, []string{"dog", "dogfood", "dogma"}, c.Complete("dog"))
}

func TestNone(t *testing.T) {
	c := NewCompleter([]string{"dog", "cat", "deer"})
	equal(t, nil, c.Complete("co"))
}

func TestSuffix(t *testing.T) {
	c := NewCompleter([]string{"dogma", "deer", "dog"})
	equal(t, nil, c.Complete("er"))
}
