obj-m += process_family_tree.o

all:
	make -C $(HEADERS) M=$(PWD) modules
clean:
	make -C $(HEADERS) M=$(PWD) clean
