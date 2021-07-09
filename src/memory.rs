use std::collections::HashMap;
use std::convert::TryInto;

use crate::ErrorStatus;
use crate::Num;
use crate::ParamMode;

// We only support a single page size.
#[derive(Debug, Clone, PartialEq)]
pub struct Memory {
    pub page_size: usize,
    pub relative_base: usize,
    pub page_table: HashMap<usize, Vec<Num>>,
    pub last_page: usize,
}

// TODO: Propogate error gracefully if attempt to write to
// negative address through positional/relative params occurs.
impl Memory {
    pub fn from_vec(page_size: usize, data: Vec<Num>) -> Memory {
        let mut memory = Memory {
            page_size,
            relative_base: 0,
            page_table: HashMap::new(),
            last_page: 0,
        };
        let mut chunks = data.chunks_exact(memory.page_size);
        let mut i = 0;
        for d in &mut chunks {
            memory.page_table.insert(i, Vec::from(d));
            i += 1;
        }

        let remainder = chunks.remainder();
        if !remainder.is_empty() {
            let mut page = Vec::from(remainder);
            page.resize_with(page_size, || 0);
            memory.page_table.insert(i, page);
        }
        memory.last_page = i;
        memory
    }
    // returns (page index, page offset)
    #[inline(always)]
    fn resolve_virtual_address(&self, address: usize) -> (usize, usize) {
        (address / self.page_size, address % self.page_size)
    }
    #[inline(always)]
    fn relative_address(&self, offset: Num) -> Result<usize, ErrorStatus> {
        match offset < 0 {
            true => self
                .relative_base
                .checked_sub(offset.abs() as usize)
                .ok_or(ErrorStatus::IllegalMemoryAccess),
            false => Ok(self.relative_base + offset as usize),
        }
    }
    pub fn adjust_relative_base(&mut self, offset: Num) -> Result<(), ErrorStatus> {
        if offset < 0 {
            self.relative_base -= offset.abs() as usize;
        } else {
            self.relative_base += offset as usize;
        }
        Ok(())
    }
    // TODO: Add an out of memory error? I doubt we'll ever have that issue,
    // but perhaps we could add a declarable memory limit to the Memory struct?
    pub fn write_raw(&mut self, address: usize, value: Num) -> Result<(), ErrorStatus> {
        // Relies on integer division.
        let (page_index, page_offset) = self.resolve_virtual_address(address);
        if page_index > self.last_page {
            self.last_page = page_index;
        }
        let page_size = self.page_size;
        let page = self
            .page_table
            .entry(address / self.page_size)
            .or_insert_with(|| vec![0; page_size]);
        page[page_offset] = value;
        Ok(())
    }

    pub fn write(
        &mut self,
        address: usize,
        value: Num,
        mode: ParamMode,
    ) -> Result<(), ErrorStatus> {
        // There isn't an immediate mode for writing. Either we have position or relative,
        // but the VM doesn't know or care about valid modes, as it simply propogates them to
        // the read/write functions.
        match mode {
            ParamMode::Immediate | ParamMode::Position => self.write_raw(
                self.read_raw(address)?
                    .try_into()
                    .map_err(|_| ErrorStatus::IllegalMemoryAccess)?,
                value,
            ),
            ParamMode::Relative => {
                self.write_raw(self.relative_address(self.read_raw(address)?)?, value)
            }
        }
    }
    // We don't even bother allocating the memory page if it doesn't exist, as it will return 0 anyway.
    // We have to check if the page exists when attempting to access it anyway.
    pub fn read_raw(&self, address: usize) -> Result<Num, ErrorStatus> {
        let (page_index, page_offset) = self.resolve_virtual_address(address);
        match self.page_table.get(&page_index) {
            Some(page) => Ok(page[page_offset]),
            None => Ok(0),
        }
    }
    pub fn read(&self, address: usize, mode: ParamMode) -> Result<Num, ErrorStatus> {
        match mode {
            ParamMode::Immediate => self.read_raw(address),
            ParamMode::Position => self.read_raw(
                self.read_raw(address)?
                    .try_into()
                    .map_err(|_| ErrorStatus::IllegalMemoryAccess)?,
            ),
            ParamMode::Relative => self.read_raw(self.relative_address(self.read_raw(address)?)?),
        }
    }
    pub fn size(&self) -> usize {
        self.page_table.len() * self.page_size
    }
    // This seems inefficient if we hit it a lot
    pub fn virtual_size(&self) -> usize {
        (self.last_page + 1) * self.page_size
    }
    pub fn flatten(&self) -> Vec<Num> {
        let v_size = self.virtual_size();
        let mut out = Vec::with_capacity(v_size);
        for i in 0..=(v_size / self.page_size) {
            match self.page_table.get(&i) {
                Some(p) => out.extend_from_slice(p.as_slice()),
                None => out.resize_with(self.page_size * i, || 0),
            }
        }
        out
    }
}
