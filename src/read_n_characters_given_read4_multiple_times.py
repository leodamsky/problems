# The read4 API is already defined for you.
# def read4(buf4: List[str]) -> int:
import collections
from typing import List


def read4(buf4: List[str]) -> int:
    buf4.extend(['a', 'b', 'c', ''])
    return 3


class Solution:
    def __init__(self):
        self.queue = collections.deque()

    def read(self, buf, n):
        read = 0
        while read < n:
            buf4 = [''] * 4
            _ = read4(buf4)
            self.queue.extend(buf4)
            count = min(len(self.queue), n - read)
            if not count:
                break
            buf[read:] = [self.queue.popleft() for _ in range(count)]
            read += count
        return read
