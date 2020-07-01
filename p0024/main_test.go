package p0024

import (
	"testing"


)

func assert(t *testing.T, cond bool) {
	t.Helper()

	if !cond {
		t.Fatal(t)
	}
}

func TestP0024(t *testing.T) {
	root := &node{}
	left := &node{parent: root}; root.left = left
	right := &node{parent: root}; root.right = right
	leftLeft := &node{parent: left}; left.left = leftLeft
	leftRight := &node{parent: left}; left.right = leftRight
	rightLeft := &node{parent: right}; right.left = rightLeft
	rightRight := &node{parent: right}; right.right = rightRight

	assert(t, root.lock())
	assert(t, !rightRight.lock())
	assert(t, root.unlock())
	assert(t, rightRight.lock())
	assert(t, rightLeft.lock())
	assert(t, leftRight.lock())
	assert(t, !left.lock())
	assert(t, !right.lock())
	assert(t, leftRight.unlock())
	assert(t, left.lock())
}
