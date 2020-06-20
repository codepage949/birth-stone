obj-m := birth_stone.o
birth_stone-objs := module.o libbirth_stone.o

all:
	cargo +nightly build --release -Z unstable-options --out-dir=$(pwd) -Z build-std=core --target=x86_64-linux-kernel
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) modules

%libbirth_stone.o: libbirth_stone.a
	ld -r -o $@ --whole-archive $^

clean:
	cargo clean
	rm -rf *.o *.a *.ko *.mod .*.*.cmd *.mod.c .tmp_versions *.symvers *.order