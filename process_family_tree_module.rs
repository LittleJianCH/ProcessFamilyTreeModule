//! Process Family Tree Module.

#![allow(improper_ctypes)]

use kernel::prelude::*;
use kernel::bindings::*;

extern "C" {
    #[link_name = "do_list"]
    fn do_list(
        ptr: *const list_head, 
        indent: core::ffi::c_int, 
        func: extern "C" fn (*const task_struct, core::ffi::c_int)
    );
}

struct ProcessFamilyTreeModule;

fn print_task(task: &task_struct, indent: usize) {
    let mut indent_str = Vec::new();

    if indent > 0 {
        for _ in 0..(indent - 1) * 4 {
            indent_str.try_push(b' ').unwrap();
        }
        
        indent_str.try_push(b'|').unwrap();
        indent_str.try_push(b'-').unwrap();
        indent_str.try_push(b'-').unwrap();
        indent_str.try_push(b' ').unwrap();
    }

    pr_info!("{}{}({})\n",
        core::str::from_utf8(indent_str.as_slice()).unwrap(),
        core::str::from_utf8(&unsafe {
            core::mem::transmute::<[i8; 16], [u8; 16]>(task.comm)
        }).unwrap_or("unable to display"),
        task.pid
    );
}

fn print_ancestors(_task: *const task_struct) {
    let task = unsafe { _task.as_ref().unwrap() };
    print_task(task, 0);

    let parent = unsafe { task.real_parent.as_ref() };

    if let Some(par) = parent {
        if par.pid != 0 {
            print_ancestors(par);
        }
    }
}

extern "C"
fn print_descendants(_task: *const task_struct, indent: core::ffi::c_int) {
    let task = unsafe { _task.as_ref().unwrap() };
    print_task(task, indent as usize);

    unsafe {
        do_list(&task.children, indent + 1, print_descendants);
    }
}

impl kernel::Module for ProcessFamilyTreeModule {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Process Family Tree Module (init)\n");

        let _pid = pid_arg.read();

        pr_info!("Print Ancestors Starts!\n");
        pr_info!("-----------------------------\n");

        let _task = unsafe { pid_task(find_vpid(*_pid), pid_type_PIDTYPE_PID) };

        print_ancestors(_task);

        pr_info!("-----------------------------\n");
        pr_info!("Print Ancestors Ends!\n");

        pr_info!("Print Descendants Starts!\n");
        pr_info!("-----------------------------\n");

        print_descendants(_task, 0);

        pr_info!("-----------------------------\n");
        pr_info!("Print Descendants Ends!\n");

        Ok(Self)
    }
}

impl Drop for ProcessFamilyTreeModule {
    fn drop(&mut self) {
        pr_info!("Process Family Tree Module (exit)\n");
    }
}



module! {
    type: ProcessFamilyTreeModule,
    name: "process_family_tree_module",
    author: "Jian",
    description: "Print family tree of a specific process(by pid)",
    license: "GPL",
    params: {
        pid_arg: i32 {
            default: 1,
            permissions: 0,
            description: "pid of the process",
        },
    },
}