use r_efi::{system::SystemTable, efi::Handle};

pub fn print_display_info(image_handle: Handle, system_table: &mut SystemTable) {
    // 
    let boot_services = system_table.boot_services();

    let gop_handle = boot_services.get_handle_for_protocol::<GraphicsOutput>().unwrap();

    let gop = unsafe { system_table
        .boot_services()
        .open_protocol::<GraphicsOutput>(OpenProtocolParams {
            handle: gop_handle,
            agent: image_handle,
            controller: None
        }, OpenProtocolAttributes::GetProtocol
    ) }.unwrap();

    println!("Supported Modes:");
    for mode in gop.modes() {
        println!(
            "    {:4} x {:4} @ {:?}",
            mode.info().resolution().0,
            mode.info().resolution().1,
            mode.info().pixel_format()
        );
    };

    let current_mode = gop.current_mode_info();
    println!("Current Mode:");
    println!(
        "    {:4} x {:4} @ {:?}",
        current_mode.resolution().0,
        current_mode.resolution().1,
        current_mode.pixel_format()
    );
}
