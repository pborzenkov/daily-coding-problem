package p0011

type node struct {
	children map[rune]*node
	value    string
}

func newNode() *node {
	return &node{
		children: make(map[rune]*node),
	}
}

func (n *node) insert(word string) {
	for _, c := range word {
		if _, ok := n.children[c]; !ok {
			n.children[c] = newNode()
		}
		n = n.children[c]
	}
	n.value = word
}

func (n *node) getAll() []string {
	var res []string

	if n.value != "" {
		res = append(res, n.value)
	}
	for _, n := range n.children {
		res = append(res, n.getAll()...)
	}

	return res
}

func (n *node) getByPrefix(prefix string) []string {
	for _, c := range prefix {
		if _, ok := n.children[c]; !ok {
			return nil
		}
		n = n.children[c]
	}

	return n.getAll()
}

// Completer provides a set of completions for a given string.
type Completer struct {
	root *node
}

func NewCompleter(dict []string) *Completer {
	c := &Completer{
		root: newNode(),
	}

	for _, word := range dict {
		c.root.insert(word)
	}

	return c
}

// Complete returns all possible completions for the given prefix.
func (c *Completer) Complete(prefix string) []string {
	return c.root.getByPrefix(prefix)
}
