#include <stddef.h>

#include "acutest.h"

struct list {
	struct list *link;
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
	l->link = NULL;
	if (h->prev)
		h->prev->link = l;
	h->prev = l;
	if (!h->next)
		h->next = l;
}

#define TAG(x) (struct *list)((uintptr_t)(x) | 0x1)
#define UNTAG(x) (struct *list)((uintptr_t)(x) & ~0x1)

struct list *list_intersection(struct list_head *h1, struct list_head *h2)
{
	struct list *inter = NULL;

	struct list *cur = h1->next;
	while (cur != NULL) {
		struct list *next = cur->link;
		cur->link = (struct list *)((uintptr_t)cur->link | 0x1);
		cur = next;
	}

	cur = h2->next; 
	while (cur != NULL) {
		if ((uintptr_t)cur->link & 0x1) {
			inter = cur;
			break;
		}
		cur = cur->link;
	}

	cur = h1->next;
	while (cur != NULL) {
		cur->link = (struct list *)((uintptr_t)cur->link & ~0x1);
		cur = cur->link;
	}

	return inter;
}

struct test_struct {
	struct list list;
	int payload;
};

static struct list *test_node(int payload)
{
	struct test_struct *p = malloc(sizeof(struct test_struct));
	p->payload = payload;
	return &p->list;
}

static void test_ok(void)
{
	LIST_HEAD(head1);
	LIST_HEAD(head2);

	list_add(&head1, test_node(3));
	list_add(&head1, test_node(7));
	struct list *i = test_node(8);
	list_add(&head1, i);
	list_add(&head1, test_node(10));

	list_add(&head2, test_node(99));
	list_add(&head2, test_node(1));
	list_add(&head2, i);

	TEST_CHECK(8 == list_entry(list_intersection(&head1, &head2), struct test_struct, list)->payload);

	/* don't bother with allocated memory, this is just a test */
}

static void test_no_intersection(void)
{
	LIST_HEAD(head1);
	LIST_HEAD(head2);

	for (int i = 0; i < 10; i++) {
		list_add(&head1, test_node(i));
		list_add(&head2, test_node(i));
	}
	
	TEST_CHECK(NULL == list_intersection(&head1, &head2));
}

TEST_LIST = {
	{ "ok", test_ok },
	{ "no_intersection", test_no_intersection },
	{ NULL, NULL },
};
