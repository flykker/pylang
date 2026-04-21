# TODO: Rewrite in Pylang when compiler is complete
# Currently this is a reference for API design

# class list:
#     def __init__(self):
#         self._data: list[int] = []
#
#     def append(self, item: int) -> None:
#         self._data.append(item)
#
#     def __len__(self) -> int:
#         return len(self._data)
#
#     def __getitem__(self, i: int) -> int:
#         return self._data[i]
#
#     def __setitem__(self, i: int, val: int) -> None:
#         self._data[i] = val
#
#     def pop(self) -> int:
#         return self._data.pop()
#
#     def clear(self) -> None:
#         self._data.clear()
#
#     def __iter__(self) -> list[int]:
#         return self._data