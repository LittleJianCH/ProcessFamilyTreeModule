obj-m += process_family_tree.o
process_family_tree-objs := process_family_tree_module.o process_family_tree_helper.o

all:
	make -C $(HEADERS) M=$(PWD) modules
clean:
	make -C $(HEADERS) M=$(PWD) clean
