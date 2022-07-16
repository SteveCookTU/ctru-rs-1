use ctru::console::Console;
use ctru::services::hid::KeyPad;
use ctru::services::{Apt, Hid};
use ctru::Gfx;

fn main() {
    //Initialize ctru
    ctru::init();

    //Initialize used services
    let apt = Apt::init().unwrap();
    let hid = Hid::init().unwrap();
    let gfx = Gfx::init().unwrap();

    //Initialize console with top screen
    let _console = Console::init(gfx.top_screen.borrow_mut());

    //Print general usage
    println!("APT Chainload example");
    println!("Press A to chainload to USA camera app");
    println!("Press START to exit");

    while apt.main_loop() {
        hid.scan_input();

        let keys_down = hid.keys_down();

        //Exit on START press
        if keys_down.contains(KeyPad::KEY_START) {
            break;
        }

        if keys_down.contains(KeyPad::KEY_A) {
            apt.set_chainloader(0x0004001000021400, 0); //Chainload USA Camera App
            break;
        }

        //Flush buffers and wait for vblank
        gfx.flush_buffers();
        gfx.swap_buffers();
        gfx.wait_for_vblank();
    }
}
