#!/usr/bin/env python3

def cons(a, b):
    def pair(f):
        return f(a, b)
    return pair

def car(pair):
    return pair(lambda a, _: a)

def cdr(pair):
    return pair(lambda _, b: b)

import unittest

class TestCarCdr(unittest.TestCase):
    def test_example(self):
        self.assertEqual(3, car(cons(3, 4)))
        self.assertEqual(4, cdr(cons(3, 4)))

    def test_long_list(self):
        l = cons(1, cons(2, cons(3, cons(4, cons(5, None)))))

        self.assertEqual(1, car(l))
        self.assertEqual(2, car(cdr(l)))
        self.assertEqual(3, car(cdr(cdr(l))))
        self.assertEqual(4, car(cdr(cdr(cdr(l)))))
        self.assertEqual(5, car(cdr(cdr(cdr(cdr(l))))))

if __name__ == '__main__':
    unittest.main()
