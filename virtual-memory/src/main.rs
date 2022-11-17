// static GLOBAL: i32 = 1000;

// fn noop() -> *const i32 {
//     let noop_local = 12345;
//     &noop_local as *const i32
// }

// fn main() {
//     let local_str = "a";
//     let local_int = 123;
//     let boxed_str = Box::new('b');
//     let boxed_int = Box::new(789);
//     let fn_int = noop();

//     println!("GLOBAL:       {:p}", &GLOBAL as *const i32);
//     println!("local_str:    {:p}", local_str as *const str);
//     println!("local_int:    {:p}", local_int as *const i32);
//     println!("boxed_int:    {:p}", Box::into_raw(boxed_int));
//     println!("boxed_str:    {:p}", Box::into_raw(boxed_str));
//     println!("fn_int:       {:p}", fn_int);
// }

use kernel32;
use winapi;

use winapi::{
    DWORD,
    HANDLE,
    LPVOID,
    PVOID,
    SIZE_T,
    LPSYSTEM_INFO,
    SYSTEM_INFO,
    MEMORY_BASIC_INFORMATION as MEMINFO,
};

fn main() {
    let this_pid: DWORD;
    let this_proc: HANDLE;
    let min_addr: LPVOID;
    let max_addr: LPVOID;
    let mut base_addr: PVOID;
    let mut proc_info: SYSTEM_INFO;
    let mut mem_info: MEMINFO;

    const MEMINFO_SIZE: usize = std::mem::size_of::<MEMINFO>();

    unsafe {
        base_addr = std::mem::zeroed();
        proc_info = std::mem::zeroed();
        mem_info = std::mem::zeroed();
    }

    unsafe {
        this_pid = kernel32::GetCurrentProcessId();
        this_proc = kernel32::GetCurrentProcess();
        kernel32::GetSystemInfo(
            &mut proc_info as LPSYSTEM_INFO
        );
    }

    min_addr = proc_info.lpMinimumApplicationAddress;
    max_addr = proc_info.lpMaximumApplicationAddress;

    println!("{:?} @ {:p}", this_pid, this_proc);
    println!("{:?}", proc_info);
    println!("min: {:p}, max: {:p}", min_addr, max_addr);

    loop {
        let rc: SIZE_T = unsafe {
            kernel32::VirtualQueryEx(
                this_proc, base_addr,
                &mut mem_info, MEMINFO_SIZE as SIZE_T
            )
        };

        if rc == 0 {
            break
        }

        println!("{:#?}", mem_info);
        base_addr = ((base_addr as u64) + mem_info.RegionSize) as PVOID;
    }
}
