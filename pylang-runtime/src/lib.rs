#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

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

#[no_mangle]
pub extern "C" fn exit(code: i32) -> ! {
    unsafe { syscall3(60, code as usize, 0, 0); }
    // syscall never returns; unreachable_unchecked avoids clippy empty_loop warning
    unsafe { core::hint::unreachable_unchecked() }
}

struct PrintBuf(UnsafeCell<[u8; 64]>);
unsafe impl Sync for PrintBuf {}

static PRINT_BUF: PrintBuf = PrintBuf(UnsafeCell::new([0u8; 64]));

static DIGITS: [u8; 10] = *b"0123456789";

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

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { syscall3(60, 1, 0, 0); }
    loop {}
}
