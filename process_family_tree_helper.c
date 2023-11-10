#include <linux/init.h>
#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/types.h>
#include <linux/list.h>
#include <linux/sched.h>

void do_list(
    const struct list_head *head,
    int indent, 
    void (*func)(const struct task_struct *, int)) 
{
    const struct task_struct *ptr;

    list_for_each_entry (ptr, head, sibling) {
        func(ptr, indent);
    }
}
