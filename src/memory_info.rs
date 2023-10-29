use r_efi::system::{SystemTable, MemoryType};
use uefi_services::{println, print};

pub fn print_memory_info(system_table: &mut SystemTable<Boot>) {
    // fetch the memory layout
    let mut buf = [0u8; 16_384];
    let buf_ptr = buf.as_ptr() as usize;

    let memory_map = system_table.boot_services().memory_map(&mut buf).unwrap();

    // print the memory layout
    println!("Memory map:");
    println!("{:16} {:16} {:12} {:8} {}", "Physical Addr", "Virtual Addr", "Num Pages", "Size", "Type");

    let mut i = 0;
    let mut total_pages = 0;
    let mut usable_pages = 0;

    for descriptor in memory_map.entries() {
        total_pages += descriptor.page_count;
        if descriptor.ty == MemoryType::CONVENTIONAL {
            usable_pages += descriptor.page_count;
        }

        if i != 0 && (i % 39) == 0 {
            println!("--- MORE ---");
            wait_for_any_key(system_table);
        }

        print!(
            "{:016X} {:016X} {:12} ",
            descriptor.phys_start,
            descriptor.virt_start,
            descriptor.page_count,
        );

        print_size_of_pages(descriptor.page_count as usize);

        println!(" {:?} ({:?})", descriptor.ty, descriptor.att);

        i += 1;
    }

    println!("--- END ---");
    print!("Total: ");
    print_size_of_pages(total_pages as usize);
    print!(", Usable: ");
    print_size_of_pages(usable_pages as usize);
    println!();
    println!();

    println!("buf (stack) is located at {:016X}, section:", buf_ptr);
    print_pointer_section(buf_ptr, &memory_map);

    let heap_buf = system_table.boot_services().allocate_pool(MemoryType::LOADER_DATA, 1024).unwrap();
    let heap_buf_ptr = heap_buf as usize;
    println!("heap_buf is located at {:016X}, section:", heap_buf_ptr);
    print_pointer_section(heap_buf_ptr, &memory_map);
    system_table.boot_services().free_pool(heap_buf).unwrap();    
}
