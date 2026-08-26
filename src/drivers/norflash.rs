use crate::board;
use at32f421_pac as pac;

const CMD_WRITE_ENABLE: u8 = 0x06;
const CMD_READ_STATUS1: u8 = 0x05;
const CMD_READ_DATA: u8 = 0x03;
const CMD_PAGE_PROGRAM: u8 = 0x02;
const CMD_ERASE_SECTOR_4K: u8 = 0x20;
#[allow(dead_code)]
const CMD_ERASE_CHIP: u8 = 0xC7;

const STATUS_BUSY_BIT: u8 = 0x01;

pub const PAGE_SIZE: usize = 256;
pub const SECTOR_SIZE: u32 = 4096;

const BUSY_POLL_LIMIT: u32 = 1_000_000;

pub const CAL_BLOCK_ADDR: u32 = 0xF210;
pub const CAL_BLOCK_LEN: usize = 11;

pub struct NorFlash<'a> {
    gpioa: &'a pac::Gpioa,
    spi1: &'a pac::Spi1,
}

impl<'a> NorFlash<'a> {
    pub fn new(gpioa: &'a pac::Gpioa, spi1: &'a pac::Spi1) -> Self {
        NorFlash { gpioa, spi1 }
    }

    fn transfer_byte(&mut self, byte: u8) -> u8 {
        while self.spi1.sts().read().tdbe().bit_is_clear() {}
        self.spi1
            .dt()
            .write(|w| unsafe { w.dt().bits(byte as u16) });
        while self.spi1.sts().read().rdbf().bit_is_clear() {}
        self.spi1.dt().read().dt().bits() as u8
    }

    fn send_addr24(&mut self, addr: u32) {
        self.transfer_byte((addr >> 16) as u8);
        self.transfer_byte((addr >> 8) as u8);
        self.transfer_byte(addr as u8);
    }

    pub fn read_bytes(&mut self, addr: u32, buf: &mut [u8]) {
        board::set_norflash_cs(self.gpioa, false);
        self.transfer_byte(CMD_READ_DATA);
        self.send_addr24(addr);
        for b in buf.iter_mut() {
            *b = self.transfer_byte(0x00);
        }
        board::set_norflash_cs(self.gpioa, true);
    }

    pub fn read_status(&mut self) -> u8 {
        board::set_norflash_cs(self.gpioa, false);
        self.transfer_byte(CMD_READ_STATUS1);
        let status = self.transfer_byte(0x00);
        board::set_norflash_cs(self.gpioa, true);
        status
    }

    fn wait_until_ready(&mut self) {
        for _ in 0..BUSY_POLL_LIMIT {
            if self.read_status() & STATUS_BUSY_BIT == 0 {
                return;
            }
        }
    }

    fn write_enable(&mut self) {
        board::set_norflash_cs(self.gpioa, false);
        self.transfer_byte(CMD_WRITE_ENABLE);
        board::set_norflash_cs(self.gpioa, true);
    }

    fn write_page(&mut self, addr: u32, data: &[u8]) {
        self.write_enable();
        board::set_norflash_cs(self.gpioa, false);
        self.transfer_byte(CMD_PAGE_PROGRAM);
        self.send_addr24(addr);
        for &b in data {
            self.transfer_byte(b);
        }
        board::set_norflash_cs(self.gpioa, true);
        self.wait_until_ready();
    }

    pub fn write_bytes(&mut self, addr: u32, data: &[u8]) {
        let mut addr = addr;
        let mut remaining = data;
        while !remaining.is_empty() {
            let offset_in_page = (addr % PAGE_SIZE as u32) as usize;
            let chunk_len = (PAGE_SIZE - offset_in_page).min(remaining.len());
            let (chunk, rest) = remaining.split_at(chunk_len);
            self.write_page(addr, chunk);
            addr += chunk_len as u32;
            remaining = rest;
        }
    }

    pub fn erase_sector(&mut self, addr: u32) {
        self.write_enable();
        board::set_norflash_cs(self.gpioa, false);
        self.transfer_byte(CMD_ERASE_SECTOR_4K);
        self.send_addr24(addr);
        board::set_norflash_cs(self.gpioa, true);
        self.wait_until_ready();
    }

    #[allow(dead_code)]
    pub fn erase_chip(&mut self) {
        self.write_enable();
        board::set_norflash_cs(self.gpioa, false);
        self.transfer_byte(CMD_ERASE_CHIP);
        board::set_norflash_cs(self.gpioa, true);
        self.wait_until_ready();
    }
}
