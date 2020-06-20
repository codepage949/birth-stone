#include <linux/init.h>
#include <linux/kernel.h>
#include <linux/module.h>
#include <linux/printk.h>

MODULE_LICENSE("GPL");

extern int rust_module_start(void);
extern void rust_module_end(void);

int module_start(void) {
    return rust_module_start();
}

void module_end(void) {
    rust_module_end();
}

module_init(module_start);
module_exit(module_end);