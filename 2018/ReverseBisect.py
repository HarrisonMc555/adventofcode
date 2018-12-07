#pylint: disable=invalid-name

"""Reversed bisection algorithms."""

def reverse_insort_right(a, x, lo=0, hi=None):
    """Insert item x in list a, and keep it reverse sorted assuming a is reverse
    sorted.

    If x is already in a, insert it to the right of the rightmost x.

    Optional args lo (default 0) and hi (default len(a)) bound the
    slice of a to be searched.

    """

    index = reverse_bisect_right(a, x, lo, hi)
    a.insert(index, x)

def reverse_bisect_right(a, x, lo=0, hi=None):
    """Return the index where to insert item x in list a, assuming a is reverse
    sorted.

    The return value i is such that all e in a[:i] have e > x, and all e in
    a[i:] have e <= x.  So if x already appears in the list, a.insert(x) will
    insert just after the rightmost x already there.

    Optional args lo (default 0) and hi (default len(a)) bound the
    slice of a to be searched.

    """

    if lo < 0:
        raise ValueError('lo must be non-negative')
    if hi is None:
        hi = len(a)
    while lo < hi:
        mid = (lo + hi) // 2
        if a[mid] > x:
            lo = mid + 1
        else:
            hi = mid
    return lo

def reverse_insort_left(a, x, lo=0, hi=None):
    """Insert item x in list a, and keep it reverse sorted assuming a is reverse
    sorted.

    If x is already in a, insert it to the left of the leftmost x.

    Optional args lo (default 0) and hi (default len(a)) bound the
    slice of a to be searched.

    """

    index = reverse_bisect_left(a, x, lo, hi)
    a.insert(index, x)


def reverse_bisect_left(a, x, lo=0, hi=None):
    """Return the index where to insert item x in list a, assuming a is reverse
    sorted.

    The return value i is such that all e in a[:i] have e >= x, and all e in
    a[i:] have e < x.  So if x already appears in the list, a.insert(x) will
    insert just before the leftmost x already there.

    Optional args lo (default 0) and hi (default len(a)) bound the
    slice of a to be searched.

    """

    if lo < 0:
        raise ValueError('lo must be non-negative')
    if hi is None:
        hi = len(a)
    while lo < hi:
        mid = (lo + hi) // 2
        if x > a[mid]:
            hi = mid
        else:
            lo = mid + 1
    return lo

# Create aliases
reverse_bisect = reverse_bisect_right
reverse_insort = reverse_insort_right
