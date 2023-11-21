//! Process Family Tree Module.

#![allow(improper_ctypes)]

use kernel::prelude::*;
use kernel::bindings::*;

struct ProcessFamilyTreeModule;

macro_rules! offset_of {
    ($struct_type:ty, $field:ident) => {
        {
            let instance = core::mem::MaybeUninit::<$struct_type>::uninit();
            let field_ptr = &(*instance.as_ptr()).$field as *const _;
            let base_ptr = instance.as_ptr();
            (field_ptr as usize) - (base_ptr as usize)
        }
    };
}

macro_rules! list_entry {
    ($ptr:expr, $struct_type:ty, $filed:ident) => {
        {
            let offset = offset_of!($struct_type, $filed);
            let ptr = ($ptr as *const _ as *const u8).offset(-(offset as isize)) as *const $struct_type;
            ptr.as_ref().unwrap()
        }
    };
}

fn print_task(task: &task_struct, indent: usize, bin_vec: &Vec<bool>) {
    let mut indent_str = Vec::new();

    if indent > 0 {
        for i in 0..indent - 1{
            if bin_vec[i] {
                indent_str.try_push(b'|').unwrap();
            } else {
                indent_str.try_push(b' ').unwrap();
            }
            indent_str.try_push(b' ').unwrap();
            indent_str.try_push(b' ').unwrap();
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
    let mut task = unsafe { _task.as_ref().unwrap() };

    loop {
        let parent = unsafe { task.parent.as_ref().unwrap() };
        // parent couldn't be null here

        let v = Vec::new();
        print_task(task, 0, &v);

        if parent.pid == 0 {
            break;
        } else {
            task = parent;
        }
    }
}

fn print_descendants(_task: *const task_struct, indent: usize, bin_vec: &mut Vec<bool>) {
    let task = unsafe { _task.as_ref().unwrap() };
    print_task(task, indent, bin_vec);
    
    let mut child_ptr = unsafe {&*task.children.next};

    while !core::ptr::eq(child_ptr, &task.children) {
        let child_task = unsafe { 
            list_entry!(child_ptr, task_struct, sibling)
        };

        bin_vec.try_push(!core::ptr::eq(child_ptr.next, &task.children)).unwrap();
        print_descendants(child_task, indent + 1, bin_vec);
        bin_vec.pop();
        
        child_ptr = unsafe {&*child_ptr.next};
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

        let mut bin_vec = Vec::new();
        print_descendants(_task, 0, &mut bin_vec);

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