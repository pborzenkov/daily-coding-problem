#!/usr/bin/env python3

import collections

def p0023(board, start, end):
    if len(board) == 0 or len(board[0]) == 0:
        return None

    width = len(board[0])
    height = len(board)

    queue = collections.deque()
    queue.append((start, 0))
    visited = set()

    while len(queue) > 0:
        (p, s) = queue.popleft()
        if p == end:
            return s
        if board[p[0]][p[1]]:
            continue

        visited.add(p)
        for x in [(p[0]-1, p[1]), (p[0]+1, p[1]), (p[0], p[1]-1), (p[0], p[1]+1)]:
            if x[0] >= 0 and x[1] >= 0 and x[0] < height - 1 and x[1] < width - 1 and x not in visited:
                queue.append((x, s + 1))

    return None

import unittest

class TestP0023(unittest.TestCase):
    def test_example(self):
        self.assertEqual(7, p0023([
            [False, False, False, False],
            [True,  True,  False, True ],
            [False, False, False, False],
            [False, False, False, False],
        ], (3, 0), (0, 0)))

if __name__ == '__main__':
    unittest.main()
