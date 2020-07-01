package p0026

type elem struct {
	val  int
	next *elem
}

func p0026(list *elem, k int) *elem {
	l1, l2 := list, list
	var l2Prev *elem

	for i := 0; i < k; i++ {
		l1 = l1.next
	}

	for l1.next != nil {
		l1 = l1.next
		l2Prev = l2
		l2 = l2.next
	}

	if l2Prev == nil {
		return l2.next
	}

	l2Prev.next = l2.next
	return list
}
