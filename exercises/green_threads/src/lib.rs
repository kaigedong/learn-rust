use core::arch::asm;

// 我们设置一个很小的栈，以便切换之前查看它。
// 48 bytes
const SSIZE: isize = 48;

// 记录CPU状态。我们目前只关注存储栈指针的寄存器
#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    rsp: u64,
}

fn hello() -> ! {
    println!("I LOVE WAKING UP ON A NEW STACK!");
    // loop {}
    panic!()
}

unsafe fn gt_switch(new: *const ThreadContext) {
    asm!(
        "mov rsp, [{0} + 0x00]",
         "ret",  // ret关键字让CPU从栈顶弹出一个位置，并无条件跳转到该位置
         in(reg) new, // 提供一个地址给第一个语句，使GPU跳转
    );
}

#[allow(dead_code)]
fn main2() {
    let mut ctx = ThreadContext::default();
    let mut stack = vec![0_u8; SSIZE as usize];

    unsafe {
        let stack_bottom = stack.as_mut_ptr().offset(SSIZE);
        // 为了使栈对齐
        let sb_aligned = (stack_bottom as usize & !15) as *mut u8;
        // 将hello的地址写到栈上
        // 在64位系统上是64位指针，因此也可以as u64
        std::ptr::write(sb_aligned.offset(-16) as *mut usize, hello as usize);
        ctx.rsp = sb_aligned.offset(-16) as u64;

        for i in (0..SSIZE).rev() {
            println!(
                "mem: {}, val: {}",
                sb_aligned.offset(i as isize) as usize,
                *sb_aligned.offset(i as isize)
            );
        }

        gt_switch(&ctx);
    }
}
