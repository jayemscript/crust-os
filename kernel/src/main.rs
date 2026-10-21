#![no_std]
#![no_main]

use bootloader_api::{entry_point, BootInfo};
use bootloader_api::info::PixelFormat;
use core::panic::PanicInfo;

use noto_sans_mono_bitmap::{
    get_raster,
    FontWeight,
    RasterHeight,
};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();
        let buffer = framebuffer.buffer_mut();

        let message = "Hello World from CrustOS!";

        let mut x = 30;
        let y = 100;

        for character in message.chars() {
            if let Some(glyph) = get_raster(
                character,
                FontWeight::Regular,
                RasterHeight::Size16,
            ) {
                for (row, pixels) in glyph.raster().iter().enumerate() {
                    for (col, intensity) in pixels.iter().enumerate() {
                        let px = x + col;
                        let py = y + row;

                        if px >= info.width || py >= info.height {
                            continue;
                        }

                        let offset =
                            (py * info.stride + px) * info.bytes_per_pixel;

                        if offset + info.bytes_per_pixel > buffer.len() {
                            continue;
                        }

                        let value = *intensity;

                        match info.pixel_format {
                            PixelFormat::Rgb | PixelFormat::Bgr => {
                                buffer[offset] = value;
                                buffer[offset + 1] = value;
                                buffer[offset + 2] = value;
                            }
                            PixelFormat::U8 => {
                                buffer[offset] = value;
                            }
                            _ => {}
                        }
                    }
                }

                x += glyph.width();
            }
        }
    }

    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}