package p0010

import (
	"sync/atomic"
	"testing"
	"time"
)

func TestScheduler(t *testing.T) {
	s := NewScheduler()

	var val int32
	s.Add(10*time.Millisecond, func() {
		atomic.AddInt32(&val, 3)
	})
	s.Add(20*time.Millisecond, func() {
		atomic.StoreInt32(&val, atomic.LoadInt32(&val)*3)
	})

	time.Sleep(100 * time.Millisecond)
	if want, got := int32(9), atomic.LoadInt32(&val); want != got {
		t.Errorf("unexpected value, want = %d, got = %d", want, got)
	}
}
