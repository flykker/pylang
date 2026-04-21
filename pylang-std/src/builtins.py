def len(x: list[int]) -> int:
    return x.__len__()

def print(x: int) -> None:
    pass

def input() -> str:
    return ""

def range(n: int) -> list[int]:
    result: list[int] = []
    i: int = 0
    while i < n:
        result.append(i)
        i = i + 1
    return result

def zip(a: list[int], b: list[int]) -> list[tuple[int, int]]:
    result: list[tuple[int, int]] = []
    i: int = 0
    while i < len(a):
        result.append((a[i], b[i]))
        i = i + 1
    return result

def enumerate(items: list[int]) -> list[tuple[int, int]]:
    result: list[tuple[int, int]] = []
    i: int = 0
    while i < len(items):
        result.append((i, items[i]))
        i = i + 1
    return result

def map(func: int, items: list[int]) -> list[int]:
    result: list[int] = []
    for item in items:
        result.append(func)
    return result

def filter(func: int, items: list[int]) -> list[int]:
    result: list[int] = []
    for item in items:
        if func:
            result.append(item)
    return result

def sum(items: list[int]) -> int:
    total: int = 0
    for item in items:
        total = total + item
    return total

def min(items: list[int]) -> int:
    if len(items) == 0:
        return 0
    result: int = items[0]
    i: int = 1
    while i < len(items):
        if items[i] < result:
            result = items[i]
        i = i + 1
    return result

def max(items: list[int]) -> int:
    if len(items) == 0:
        return 0
    result: int = items[0]
    i: int = 1
    while i < len(items):
        if items[i] > result:
            result = items[i]
        i = i + 1
    return result