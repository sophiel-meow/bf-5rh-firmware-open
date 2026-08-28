use at32f421_pac as pac;
use cortex_m::peripheral::SCB;

use crate::board;
use crate::drivers::norflash::NorFlash;
use crate::hal::crc16;

const CMD_HANDSHAKE: u8 = 0x01;
const CMD_ERASE: u8 = 0x02;
const CMD_WRITE: u8 = 0x03;
const CMD_END: u8 = 0x45;

const ACK: u8 = 0x06;
const ERR_LEN: u8 = 0xE1;
const ERR_ADDR: u8 = 0xE2;
const ERR_CMD: u8 = 0xE5;

const STATE_IDLE: u8 = 0;
const STATE_HANDSHOOK: u8 = 1;

const CHUNK_LEN: usize = 512;
const MAX_DATA_LEN: usize = 4 + CHUNK_LEN;

const READ_TIMEOUT_SPINS: u32 = 200_000;

struct Frame {
    cmd: u8,
    len: u16,
}

struct Uart<'a> {
    usart1: &'a pac::Usart1,
}

impl<'a> Uart<'a> {
    fn read_byte_timeout(&mut self) -> Option<u8> {
        for _ in 0..READ_TIMEOUT_SPINS {
            let sts = self.usart1.sts().read();
            if sts.rdbf().bit_is_set() {
                return Some(self.usart1.dt().read().dt().bits() as u8);
            }
        }
        None
    }

    fn write_byte(&mut self, b: u8) {
        while self.usart1.sts().read().tdbe().bit_is_clear() {}
        self.usart1.dt().write(|w| unsafe { w.dt().bits(b as u16) });
    }

    fn flush(&mut self) {
        while self.usart1.sts().read().tdc().bit_is_clear() {}
    }
}

fn recv_frame(uart: &mut Uart, buf: &mut [u8; MAX_DATA_LEN]) -> Option<Frame> {
    loop {
        if uart.read_byte_timeout()? == 0xAA {
            break;
        }
    }
    let cmd = uart.read_byte_timeout()?;
    let cmdargs = uart.read_byte_timeout()?;
    let len_hi = uart.read_byte_timeout()?;
    let len_lo = uart.read_byte_timeout()?;
    let len = ((len_hi as u16) << 8) | len_lo as u16;

    let mut crc = crc16::update(0, cmd);
    crc = crc16::update(crc, cmdargs);
    crc = crc16::update(crc, len_hi);
    crc = crc16::update(crc, len_lo);

    if len as usize > MAX_DATA_LEN {
        return None;
    }
    for slot in buf.iter_mut().take(len as usize) {
        let byte = uart.read_byte_timeout()?;
        crc = crc16::update(crc, byte);
        *slot = byte;
    }

    let crc_hi = uart.read_byte_timeout()?;
    let crc_lo = uart.read_byte_timeout()?;
    let _footer = uart.read_byte_timeout()?;

    let got_crc = ((crc_hi as u16) << 8) | crc_lo as u16;
    if got_crc != crc {
        return None;
    }

    Some(Frame { cmd, len })
}

fn send_frame(uart: &mut Uart, cmd: u8, cmdargs: u8) {
    let (len_hi, len_lo) = (0u8, 0u8);
    let mut crc = crc16::update(0, cmd);
    crc = crc16::update(crc, cmdargs);
    crc = crc16::update(crc, len_hi);
    crc = crc16::update(crc, len_lo);

    uart.write_byte(0xAA);
    uart.write_byte(cmd);
    uart.write_byte(cmdargs);
    uart.write_byte(len_hi);
    uart.write_byte(len_lo);
    uart.write_byte((crc >> 8) as u8);
    uart.write_byte((crc & 0xFF) as u8);
    uart.write_byte(0xEF);
}

fn send_ack(uart: &mut Uart, cmd: u8) {
    send_frame(uart, cmd, ACK);
}

fn send_err(uart: &mut Uart, err_code: u8) {
    send_frame(uart, err_code, err_code);
}

fn le_addr(data: &[u8; MAX_DATA_LEN]) -> u32 {
    u32::from_le_bytes([data[0], data[1], data[2], data[3]])
}

pub fn run(gpioa: &pac::Gpioa, spi1: &pac::Spi1, usart1: &pac::Usart1) -> ! {
    board::init_usart1_rx_pin(gpioa);

    let mut norflash = NorFlash::new(gpioa, spi1);
    let mut uart = Uart { usart1 };
    let mut buf = [0u8; MAX_DATA_LEN];
    let mut state = STATE_IDLE;

    loop {
        let Some(frame) = recv_frame(&mut uart, &mut buf) else {
            continue;
        };

        match frame.cmd {
            CMD_HANDSHAKE => {
                state = STATE_HANDSHOOK;
                send_ack(&mut uart, CMD_HANDSHAKE);
            }
            CMD_ERASE => {
                if state != STATE_HANDSHOOK {
                    send_err(&mut uart, ERR_CMD);
                } else if frame.len != 4 {
                    send_err(&mut uart, ERR_LEN);
                } else {
                    let addr = le_addr(&buf);
                    if addr % crate::drivers::norflash::SECTOR_SIZE != 0 {
                        send_err(&mut uart, ERR_ADDR);
                    } else {
                        norflash.erase_sector(addr);
                        send_ack(&mut uart, CMD_ERASE);
                    }
                }
            }
            CMD_WRITE => {
                if state != STATE_HANDSHOOK {
                    send_err(&mut uart, ERR_CMD);
                } else if (frame.len as usize) < 4 {
                    send_err(&mut uart, ERR_LEN);
                } else {
                    let addr = le_addr(&buf);
                    let payload = &buf[4..frame.len as usize];
                    norflash.write_bytes(addr, payload);
                    send_ack(&mut uart, CMD_WRITE);
                }
            }
            CMD_END => {
                send_ack(&mut uart, CMD_END);
                uart.flush();
                SCB::sys_reset();
            }
            _ => {
                send_err(&mut uart, ERR_CMD);
            }
        }
    }
}
