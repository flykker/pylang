class dict:
    def __init__(self):
        self._keys: list[str] = []
        self._values: list[int] = []

    def __setitem__(self, key: str, val: int) -> None:
        if key in self._keys:
            let idx: int = self._keys.index(key)
            self._values[idx] = val
        else:
            self._keys.append(key)
            self._values.append(val)

    def __getitem__(self, key: str) -> int:
        let idx: int = self._keys.index(key)
        return self._values[idx]

    def __len__(self) -> int:
        return len(self._keys)

    def keys(self) -> list[str]:
        return self._keys

    def values(self) -> list[int]:
        return self._values

    def get(self, key: str, default: int) -> int:
        if key in self._keys:
            return self[key]
        return default

    def pop(self, key: str) -> int:
        let idx: int = self._keys.index(key)
        self._keys.pop(idx)
        return self._values.pop(idx)

    def clear(self) -> None:
        self._keys.clear()
        self._values.clear()