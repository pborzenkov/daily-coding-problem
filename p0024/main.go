package p0024

type node struct {
	parent *node
	left   *node
	right  *node

	locked         bool
	lockedChildren uint
}

func (n *node) is_locked() bool {
	return n.locked
}

func (n *node) lock() bool {
	if n.lockedChildren > 0 {
		return false
	}
	for p := n; p != nil; p = p.parent {
		if p.locked {
			return false
		}
	}

	for p := n.parent; p != nil; p = p.parent {
		p.lockedChildren += 1
	}

	n.locked = true
	return true
}

func (n *node) unlock() bool {
	if !n.locked {
		return false
	}

	for p := n.parent; p != nil; p = p.parent {
		p.lockedChildren -= 1
	}

	n.locked = false
	return true
}
