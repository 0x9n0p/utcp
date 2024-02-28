use std::arch::global_asm;
use std::io;

global_asm!(include_str!("queue.asm"));

extern {
    fn on_rcvd_frame(frame: *const u8, n: u64);
}

fn main() -> io::Result<()> {
    let device = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;

    let mut buf = [0u8; 1504];
    loop {
        let n = device.recv(&mut buf[..])?;
        eprintln!("{} {:x?}", n, &buf[..n]);

        unsafe { on_rcvd_frame(&buf as *const u8, n as u64); }
    }
}
