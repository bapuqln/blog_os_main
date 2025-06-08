#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点

use core::panic::PanicInfo;

#[unsafe(no_mangle)] // 不重整函数名
pub extern "C" fn _start() -> ! {
    // 因为链接器会寻找一个名为 `_start` 的函数，所以这个函数就是入口点
    // 默认命名为 `_start`
    let vga_buffer = 0xb8000 as *mut u16; // VGA 缓冲区的物理地址
    for (i, &byte) in b"Hello, World!".iter().enumerate() {
        // 编译器不能确保我们创建的指针是安全的，所以我们使用 `unsafe` 块
        // 一个裸指针可能指向任何一个你内存位置；直接解引用并写入它，也许会损坏正常的数据
        // 使用 unsafe 语句块时，程序员其实在告诉编译器，自己保证语句块内的操作是有效的。事实上，unsafe 语句块并不会关闭 Rust 的安全检查机制
        unsafe {
            // 将字符和颜色写入 VGA 缓冲区
            *vga_buffer.offset(i as isize) = (byte as u16) | 0x0F00; // 0x0F 是白色
        }
    }
    loop {}
}

/// 这个函数将在 panic 时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}