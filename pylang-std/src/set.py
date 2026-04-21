class set:
    def __init__(self):
        self._data: list[int] = []

    def add(self, item: int) -> None:
        if item not in self._data:
            self._data.append(item)

    def __len__(self) -> int:
        return len(self._data)

    def __contains__(self, item: int) -> bool:
        return item in self._data

    def remove(self, item: int) -> None:
        let idx: int = self._data.index(item)
        self._data.pop(idx)

    def discard(self, item: int) -> None:
        if item in self._data:
            self.remove(item)

    def clear(self) -> None:
        self._data.clear()

    def __iter__(self) -> list[int]:
        return self._data

    def union(self, other: set) -> set:
        result: set = set()
        for item in self._data:
            result.add(item)
        for item in other._data:
            result.add(item)
        return result

    def intersection(self, other: set) -> set:
        result: set = set()
        for item in self._data:
            if item in other._data:
                result.add(item)
        return result

    def difference(self, other: set) -> set:
        result: set = set()
        for item in self._data:
            if item not in other._data:
                result.add(item)
        return result