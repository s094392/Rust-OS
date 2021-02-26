SHELL = /bin/sh
ARM = aarch64-linux-gnu
LD = $(ARM)-ld
OBJCOPY = $(ARM)-objcopy
LINKER = src/link.ld
KERNEL = target/aarch64-unknown-none-softfloat/release/osc

all: clean kernel8.img 
       
kernel8.img: kernel8.elf
	$(OBJCOPY) -O binary kernel8.elf kernel8.img    
     
kernel8.elf: $(LINKER)
	$(LD) -T $(LINKER) -o kernel8.elf $(KERNEL)

clean:
	rm -rf kernel8.elf kernel8.img

run: all
	qemu-system-aarch64 -M raspi3 -kernel kernel8.img -serial null -serial mon:stdio

debug: all
	tilix -a app-new-session -e "qemu-system-aarch64 -M raspi3 -kernel kernel8.img -serial null -serial mon:stdio -s -S" 
	tilix -a app-new-session -e "./debug.sh"
