#include <linux/init.h>
#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/types.h>
#include <linux/list.h>
#include <linux/sched.h>

void do_list(
    const struct list_head *head,
    unsigned int indent, 
    unsigned int bin_vec,
    void (*func)(const struct task_struct *, unsigned int, unsigned int)) 
{
    const struct list_head *ptr;

    list_for_each (ptr, head) {
        const struct task_struct *task = list_entry(ptr, struct task_struct, sibling);
        func(task, indent + 1, bin_vec | (list_is_head(ptr, head) << indent));
    }
}
