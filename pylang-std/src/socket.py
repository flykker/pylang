# Socket stdlib for Pylang — built on top of syscall3/syscall6
# All functions return i64 (fd, result, or error code)

def socket_sys(domain: int, type: int, protocol: int) -> int:
    return syscall3(41, domain, type, protocol)

def bind_sys(fd: int, sockaddr_ptr: int, addrlen: int) -> int:
    return syscall3(49, fd, sockaddr_ptr, addrlen)

def listen_sys(fd: int, backlog: int) -> int:
    return syscall3(50, fd, backlog, 0)

def accept_sys(fd: int) -> int:
    return syscall3(43, fd, 0, 0)

def recv_sys(fd: int, buf: ptr, len: int) -> int:
    return syscall6(45, fd, buf, len, 0, 0, 0, 0)

def send_sys(fd: int, buf: int, len: int) -> int:
    return syscall6(44, fd, buf, len, 0, 0, 0, 0)

def close_sys(fd: int) -> int:
    return syscall3(3, fd, 0, 0)

def exit_sys(code: int):
    syscall3(60, code, 0, 0)

def signal_sys(signum: int, handler: int) -> int:
    return syscall3(13, signum, handler, 0)

def waitpid_sys(pid: int, options: int) -> int:
    return syscall6(61, pid, 0, options, 0, 0, 0, 0)

def fork_sys() -> int:
    return syscall3(57, 0, 0, 0)

def alloc_sys(size: int) -> ptr:
    return syscall6(9, 0, size, 3, 34, -1, 0)
