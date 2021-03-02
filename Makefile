SHELL = /bin/sh
ARM = aarch64-linux-gnu
LD = $(ARM)-ld
OBJCOPY = $(ARM)-objcopy
LINKER = src/link.ld
KERNEL = target/aarch64-unknown-none-softfloat/release/osc

all: clean kernel8.img 
       
kernel8.img: $(KERNEL)
	$(OBJCOPY) -O binary $(KERNEL) kernel8.img    
     
$(KERNEL): $(LINKER)
	RUSTFLATS="-C link-arg=$(LINKER) -C target-cpu=cortex-a53 -D warnings" cargo rustc --target=aarch64-unknown-none-softfloat --release

clean:
	rm -rf kernel8.img
	cargo clean

run: all
	qemu-system-aarch64 -M raspi3 -kernel kernel8.img -serial null -serial stdio

debug: all
	tilix -a app-new-session -e "qemu-system-aarch64 -M raspi3 -kernel kernel8.img -serial null -serial stdio -s -S" 
	tilix -a app-new-session -e "./debug.sh"
