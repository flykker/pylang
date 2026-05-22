# Epoll stdlib for Pylang — built on top of syscall3/syscall6

struct epoll_event:
    events: i32
    data: i64

def epoll_create1(flags: int) -> int:
    return syscall3(291, flags, 0, 0)

def epoll_ctl(epfd: int, op: int, fd: int, events: ptr) -> int:
    return syscall6(233, epfd, op, fd, events, 0, 0, 0)

def epoll_wait(epfd: int, events: ptr, maxevents: int, timeout: int) -> int:
    return syscall6(232, epfd, events, maxevents, timeout, 0, 0, 0)

def accept4(fd: int, flags: int) -> int:
    return syscall6(288, fd, 0, 0, flags, 0, 0, 0)
