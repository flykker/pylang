#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

#[cfg(test)]
fn main() {}

use core::arch::asm;
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicUsize, Ordering};

const HEAP_SIZE: usize = 16 * 1024 * 1024; // 16MB

struct Heap(UnsafeCell<[u8; HEAP_SIZE]>);
unsafe impl Sync for Heap {}

static HEAP: Heap = Heap(UnsafeCell::new([0; HEAP_SIZE]));
static HEAP_POS: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    let pos = HEAP_POS.fetch_add(size, Ordering::SeqCst);
    if pos + size > HEAP_SIZE {
        return core::ptr::null_mut();
    }
    unsafe { (*HEAP.0.get()).as_mut_ptr().add(pos) }
}

#[no_mangle]
pub extern "C" fn alloc_copy(src: *const u8, len: usize) -> *mut u8 {
    let ptr = alloc(len);
    let mut i: usize = 0;
    while i < len {
        unsafe { ptr.add(i).write_volatile(src.add(i).read_volatile()) }
        i += 1;
    }
    ptr
}

#[no_mangle]
pub extern "C" fn dealloc(_ptr: *mut u8, _size: usize) {
    // Bump allocator — no free.
}

unsafe fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> usize {
    let ret: usize;
    asm!(
        "syscall",
        in("rax") n,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        lateout("rax") ret,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags),
    );
    ret
}

unsafe fn syscall6(n: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize, a6: usize) -> usize {
    let ret: usize;
    asm!(
        "syscall",
        in("rax") n,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") a4,
        in("r8") a5,
        in("r9") a6,
        lateout("rax") ret,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags),
    );
    ret
}

#[no_mangle]
pub extern "C" fn exit(code: i64) -> i64 {
    unsafe { syscall3(60, code as usize, 0, 0); }
    unsafe { core::hint::unreachable_unchecked() }
}

struct PrintBuf64(UnsafeCell<[u8; 64]>);

unsafe impl Sync for PrintBuf64 {}

static PRINT_BUF: PrintBuf64 = PrintBuf64(UnsafeCell::new([0u8; 64]));

static DIGITS: [u8; 10] = *b"0123456789";
static STRINGS: [&[u8]; 2] = [b"0.0.0.0\0", b"127.0.0.1\0"];

#[no_mangle]
pub extern "C" fn string_ptr(idx: usize) -> *const u8 {
    if idx < STRINGS.len() { STRINGS[idx].as_ptr() }
    else { core::ptr::null() }
}

#[no_mangle]
pub extern "C" fn string_to_sockaddr(ip_ptr: *const u8, port: i32) -> *mut u8 {
    let mut addr: [u8; 16] = [0; 16];
    addr[0] = 2;  // AF_INET
    let port_be = (port as u16).to_be_bytes();
    addr[2] = port_be[0];
    addr[3] = port_be[1];
    
    // Check IP string to get correct sin_addr at offset 4
    let ip = unsafe { core::slice::from_raw_parts(ip_ptr, 8) };
    if ip[0] == b'0' && ip[1] == b'.' {
        // "0.0.0.0" -> sin_addr = 0.0.0.0 (already zeros)
    } else if ip[0] == b'1' && ip[1] == b'2' && ip[2] == b'7' {
        // "127.0.0.1"
        addr[4] = 127;
        addr[7] = 1;
    }
    // Copy to fixed buffer
    let buf = SOCKADDR_BUF.0.get() as *mut u8;
    unsafe { core::ptr::copy_nonoverlapping(addr.as_ptr(), buf, 16) };
    buf
}

struct SockAddrBuf(UnsafeCell<[u8; 16]>);
unsafe impl Sync for SockAddrBuf {}
static SOCKADDR_BUF: SockAddrBuf = SockAddrBuf(UnsafeCell::new([0u8; 16]));

#[no_mangle]
pub extern "C" fn print_int_raw(x: i64) {
    let buf = unsafe { &mut *PRINT_BUF.0.get() };
    let ptr = buf.as_mut_ptr();
    let mut i: usize = 0;

    if x < 0 {
        unsafe { ptr.add(0).write_volatile(b'-'); }
        i = 1;
    }

    let abs_x = x.unsigned_abs() as usize;
    let mut digits = [0u8; 20];
    let mut j = 0;
    let mut n = abs_x;
    
    if n == 0 {
        unsafe { ptr.add(i).write_volatile(b'0'); }
        i += 1;
    } else {
        while n > 0 {
            digits[j] = DIGITS[n % 10];
            n /= 10;
            j += 1;
        }
        while j > 0 {
            j -= 1;
            unsafe { ptr.add(i).write_volatile(digits[j]); }
            i += 1;
        }
    }

    unsafe { syscall3(1, 1, ptr as usize, i); }
}

#[no_mangle]
pub extern "C" fn print_int(x: i64) {
    let buf = unsafe { &mut *PRINT_BUF.0.get() };
    let ptr = buf.as_mut_ptr();
    let mut i: usize = 0;

    if x < 0 {
        unsafe { ptr.add(0).write_volatile(b'-'); }
        i = 1;
    }

    let abs_x = x.unsigned_abs() as usize;
    let mut digits = [0u8; 20];
    let mut j = 0;
    let mut n = abs_x;
    
    if n == 0 {
        unsafe { ptr.add(i).write_volatile(b'0'); }
        i += 1;
    } else {
        while n > 0 {
            digits[j] = DIGITS[n % 10];
            n /= 10;
            j += 1;
        }
        while j > 0 {
            j -= 1;
            unsafe { ptr.add(i).write_volatile(digits[j]); }
            i += 1;
        }
    }

    unsafe { ptr.add(i).write_volatile(b'\n'); }
    i += 1;

    unsafe { syscall3(1, 1, ptr as usize, i); }
}

#[no_mangle]
pub extern "C" fn print_str(ptr: *const u8, len: usize) {
    unsafe { syscall3(1, 1, ptr as usize, len); }
}

#[no_mangle]
pub extern "C" fn int_to_str(buf: *mut u8, val: i64) -> i64 {
    let mut i: usize = 0;
    if val < 0 {
        unsafe { buf.add(0).write_volatile(b'-'); }
        i = 1;
    }
    let abs_x = val.unsigned_abs() as usize;
    let mut digits = [0u8; 20];
    let mut j: usize = 0;
    let mut n = abs_x;
    if n == 0 {
        unsafe { buf.add(i).write_volatile(b'0'); }
        i += 1;
    } else {
        while n > 0 {
            digits[j] = b'0' + (n % 10) as u8;
            n /= 10;
            j += 1;
        }
        while j > 0 {
            j -= 1;
            unsafe { buf.add(i).write_volatile(digits[j]); }
            i += 1;
        }
    }
    i as i64
}

#[no_mangle]
pub extern "C" fn dict_new(capacity: i64) -> *mut u8 {
    let cap = if capacity < 4 { 4 } else { capacity as usize };
    let size = 16 + cap * 16;
    let ptr = alloc(size);
    unsafe {
        *(ptr as *mut i64) = 0;
        *((ptr as *mut i64).add(1)) = cap as i64;
    }
    ptr
}

#[no_mangle]
pub extern "C" fn dict_set(dict: *mut u8, key: i64, val: i64) -> i64 {
    let num = unsafe { *(dict as *const i64) };
    let cap = unsafe { *((dict as *const i64).add(1)) };
    for i in 0..(num as usize) {
        let k = unsafe { *((dict as *const i64).add(2 + i * 2)) };
        if k == key {
            unsafe { *((dict as *mut i64).add(2 + i * 2 + 1)) = val; }
            return 0;
        }
    }
    if num < cap {
        unsafe {
            *((dict as *mut i64).add(2 + num as usize * 2)) = key;
            *((dict as *mut i64).add(2 + num as usize * 2 + 1)) = val;
            *(dict as *mut i64) = num + 1;
        }
        return 0;
    }
    -1
}

#[no_mangle]
pub extern "C" fn str_copy(dst: *mut u8, src: *const u8) -> i64 {
    let len = unsafe { *(src as *const i64) };
    let src_data = unsafe { src.add(8) };
    let mut i: usize = 0;
    while i < len as usize {
        unsafe { dst.add(i).write_volatile(src_data.add(i).read_volatile()) }
        i += 1;
    }
    len
}

#[no_mangle]
pub extern "C" fn dict_read(dict: *const u8, key: i64) -> i64 {
    let num = unsafe { *(dict as *const i64) };
    if !(0i64..=1000000).contains(&num) { return 0; }
    for i in 0..(num as usize) {
        let k = unsafe { *((dict as *const i64).add(2 + i * 2)) };
        if k == key {
            return unsafe { *((dict as *const i64).add(2 + i * 2 + 1)) };
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn socket(domain: i32, r#type: i32, protocol: i32) -> i32 {
    unsafe { syscall3(41, domain as usize, r#type as usize, protocol as usize) as i32 }
}

#[no_mangle]
pub extern "C" fn setsockopt(fd: i32, level: i32, optname: i32, optval: *const u8, optlen: i32) -> i32 {
    // syscall 54: setsockopt(fd, level, optname, optval, optlen)
    unsafe { syscall6(54, fd as usize, level as usize, optname as usize, optval as usize, optlen as usize, 0) as i32 }
}

#[no_mangle]
pub extern "C" fn bind(fd: i32, sockaddr_ptr: *mut u8, addrlen: usize) -> i32 {
    // Enable SO_REUSEADDR to avoid EADDRINUSE on restart
    let optval: i32 = 1;
    unsafe { syscall6(54, fd as usize, 1, 2, &optval as *const _ as usize, 4, 0); }
    unsafe { syscall3(49, fd as usize, sockaddr_ptr as usize, addrlen) as i32 }
}

#[no_mangle]
pub extern "C" fn connect(fd: i32, sockaddr_ptr: *mut u8, addrlen: usize) -> i32 {
    unsafe { syscall3(42, fd as usize, sockaddr_ptr as usize, addrlen) as i32 }
}

#[no_mangle]
pub extern "C" fn listen(fd: i32, backlog: i32) -> i32 {
    unsafe { syscall3(50, fd as usize, backlog as usize, 0) as i32 }
}

#[no_mangle]
pub extern "C" fn accept(fd: i32) -> i32 {
    let mut addr: [usize; 3] = [2, 0, 0];
    let mut addr_len: usize = 16;
    unsafe {
        let ret = syscall3(43, fd as usize, &mut addr as *mut _ as usize, &mut addr_len as *mut _ as usize);
        ret as i32
    }
}

#[no_mangle]
pub extern "C" fn recv(fd: i32, buf: *mut u8, size: usize) -> isize {
    // buf layout: [len: i64][data: u8 * size]
    let data_ptr = unsafe { buf.add(8) };
    // recvfrom syscall (45): recvfrom(fd, buf, len, flags=0, src_addr=NULL, addrlen=NULL)
    let ret = unsafe { syscall6(45, fd as usize, data_ptr as usize, size, 0, 0, 0) as isize };
    if ret > 0 {
        unsafe { *(buf as *mut i64) = ret as i64 };
    } else if ret == 0 {
        unsafe { *(buf as *mut i64) = 0 };
    }
    ret
}

#[no_mangle]
pub extern "C" fn send(fd: i32, data: *const u8) -> isize {
    // data layout: [len: i64][data: u8 * len]
    let len = unsafe { *(data as *const i64) } as usize;
    let data_ptr = unsafe { data.add(8) };
    // sendto syscall (44): sendto(fd, buf, len, flags=0, dest_addr=NULL, addrlen=0)
    unsafe { syscall6(44, fd as usize, data_ptr as usize, len, 0, 0, 0) as isize }
}

#[no_mangle]
pub extern "C" fn close(fd: i32) -> i32 {
    unsafe { syscall3(3, fd as usize, 0, 0) as i32 }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { syscall3(60, 1, 0, 0); }
    loop {}
}
