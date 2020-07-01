package p0026

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func setup() *elem {
	return &elem{
		val: 1,
		next: &elem{
			val: 2,
			next: &elem{
				val: 3,
				next: &elem{
					val: 4,
					next: &elem{
						val: 5,
					},
				},
			},
		},
	}
}

func toSlice(list *elem) []int {
	ret := make([]int, 0)
	for l := list; l != nil; l = l.next {
		ret = append(ret, l.val)
	}

	return ret
}

func TestP0026(t *testing.T) {
	t.Run("first", func(t *testing.T) {
		list := setup()
		list = p0026(list, 4)

		if want, got := []int{2, 3, 4, 5}, toSlice(list); cmp.Diff(want, got) != "" {
			t.Errorf("unexpected result, diff = \n%s", cmp.Diff(want, got))
		}
	})

	t.Run("middle", func(t *testing.T) {
		list := setup()
		list = p0026(list, 2)

		if want, got := []int{1, 2, 4, 5}, toSlice(list); cmp.Diff(want, got) != "" {
			t.Errorf("unexpected result, diff = \n%s", cmp.Diff(want, got))
		}
	})

	t.Run("last", func(t *testing.T) {
		list := setup()
		list = p0026(list, 0)

		if want, got := []int{1, 2, 3, 4}, toSlice(list); cmp.Diff(want, got) != "" {
			t.Errorf("unexpected result, diff = \n%s", cmp.Diff(want, got))
		}
	})
}
