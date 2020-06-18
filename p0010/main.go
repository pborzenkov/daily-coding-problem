package p0010

import (
	"container/heap"
	"time"
)

type job struct {
	when time.Time
	what func()
}

type jobQueue []*job

func (q jobQueue) Len() int {
	return len(q)
}

func (q jobQueue) Less(i, j int) bool {
	return q[i].when.Before(q[j].when)
}

func (q jobQueue) Swap(i, j int) {
	q[i], q[j] = q[j], q[i]
}

func (q *jobQueue) Push(x interface{}) {
	*q = append(*q, x.(*job))
}

func (q *jobQueue) Pop() interface{} {
	j := (*q)[len(*q)-1]
	*q = (*q)[:len(*q)-1]
	return j
}

type Scheduler struct {
	queue jobQueue
	add   chan *job
}

func NewScheduler() *Scheduler {
	s := &Scheduler{
		add: make(chan *job),
	}
	s.run()

	return s
}

func (s *Scheduler) run() {
	var tmr <-chan time.Time
	go func() {
		for {
			if len(s.queue) > 0 {
				tmr = time.After(s.queue[0].when.Sub(time.Now()))
			} else {
				tmr = nil
			}

			select {
			case <-tmr:
				job := heap.Pop(&s.queue).(*job)
				go func() {
					job.what()
				}()
			case job := <-s.add:
				heap.Push(&s.queue, job)
			}
		}
	}()
}

func (s *Scheduler) Add(after time.Duration, fn func()) {
	s.add <- &job{
		when: time.Now().Add(after),
		what: fn,
	}
}
