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
            ($ptr as *const u8).offset(-(offset as isize)) as *const $struct_type
        }
    };
}

// Because of orphan rule, 
// we have to warp list_head in a struct.
struct NewListHead(*const list_head);

struct ListHeadIterator {
    ptr: *const list_head,
    head: *const list_head,
}

impl Iterator for ListHeadIterator {
    type Item = *const list_head;

    fn next(&mut self) -> Option<Self::Item> {
        let next = unsafe { (*self.ptr).next };
        if core::ptr::eq(next, self.head) {
            None
        } else {
            self.ptr = next;
            Some(next)
        }
    }
}

impl IntoIterator for NewListHead {
    type Item = *const list_head;
    type IntoIter = ListHeadIterator;

    fn into_iter<'a>(self) -> Self::IntoIter {
        let p = self.0 as *const _;
        ListHeadIterator {
            ptr: p,
            head: p,
        }
    }
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
    
    for child_ptr in NewListHead(&task.children as *const _) {
        let child_task = unsafe { 
            &*list_entry!(child_ptr, task_struct, sibling)
        };

        let is_last_child = core::ptr::eq(unsafe { (*child_ptr).next }, &task.children);
        bin_vec.try_push(!is_last_child).unwrap();
        print_descendants(child_task, indent + 1, bin_vec);
        bin_vec.pop();
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