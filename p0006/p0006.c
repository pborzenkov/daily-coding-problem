#include <stddef.h>

#include "acutest.h"

struct list {
	uintptr_t link;
};

struct list_head {
	struct list *next;
	struct list *prev;
};

#define LIST_INIT() { NULL, NULL }
#define LIST_HEAD(name) struct list_head name = LIST_INIT()

#define list_entry(ptr, type, member) ({				\
		const typeof( ((type *)0)->member ) *__ptr = (ptr);	\
		(type *)( (char *)__ptr - offsetof(type,member) );	\
	})

// list_add adds `l` to the end of `h`.
void list_add(struct list_head *h, struct list *l)
{
	l->link = (uintptr_t)h->prev;
	if (h->prev)
		h->prev->link = (uintptr_t)l ^ (uintptr_t)h->prev->link;
	h->prev = l;
	if (!h->next)
		h->next = l;
}

// list_get returns the `idx`th element of the list (indexed from 0), or NULL.
struct list *list_get(struct list_head *h, int idx)
{
	struct list *p = NULL;
	struct list *c = h->next;

	for (int i = 0; i < idx && c; i++) {
		struct list *tmp = c;
		c = (struct list *)((uintptr_t)p ^ c->link);
		p = tmp;
	}

	return c;
}

struct test_struct {
	struct list list;
	int payload;
};

static void test_ok(void) {
	LIST_HEAD(head);

	int i;
	for (i = 0; i < 10; i++) {
		struct test_struct *p = malloc(sizeof(struct test_struct));
		p->payload = i;
		list_add(&head, &p->list);
	}

	for (i = 0; i < 10; i++)
		TEST_CHECK(i == list_entry(list_get(&head, i), struct test_struct, list)->payload);

	/* don't bother with allocated memory, this is just a test */
}

static void test_empty(void) {
	LIST_HEAD(head);

	TEST_CHECK(NULL == list_get(&head, 0));
	TEST_CHECK(NULL == list_get(&head, 1));
}

static void test_oob(void) {
	LIST_HEAD(head);

	for (int i = 0; i < 10; i++) {
		struct test_struct *p = malloc(sizeof(struct test_struct));
		p->payload = i;
		list_add(&head, &p->list);
	}

	TEST_CHECK(NULL == list_get(&head, 10));
}

static void test_single(void) {
	LIST_HEAD(head);

	struct test_struct *p = malloc(sizeof(struct test_struct));
	p->payload = 42;
	list_add(&head, &p->list);

	TEST_CHECK(42 == list_entry(list_get(&head, 0), struct test_struct, list)->payload);
	TEST_CHECK(NULL == list_get(&head, 1));
}

TEST_LIST = {
	{ "ok", test_ok },
	{ "empty", test_empty },
	{ "oob", test_oob },
	{ "single", test_single },
	{ NULL, NULL, },
};
