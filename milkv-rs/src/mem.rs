


#[repr(C, align(4096))]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PageTable{
    pub entries: [PageTableEntry; 512]
}


// ----------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct PhyPtr<P: ?Sized>(pub *mut P);
impl<P> PhyPtr<P>{
    pub fn to_virt<T: PhysToVirt>(self, t: T) -> Result<VirtPtr<P>, T::Error>{
        t.phys_to_virt(self)
    }

    pub const fn cast<T>(self) -> PhyPtr<T>{
        PhyPtr(self.0.cast())
    }
}
impl<P: ?Sized> From<*mut P> for PhyPtr<P>{
    fn from(value: *mut P) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct VirtPtr<P: ?Sized>(pub *mut P);
impl<P> VirtPtr<P>{
    pub fn to_phys<T: VirtToPhys>(self, t: T) -> Result<PhyPtr<P>, T::Error>{
        t.virt_to_phys(self)
    }

    pub const fn cast<T>(self) -> VirtPtr<T>{
        VirtPtr(self.0.cast())
    }
}
impl<P: ?Sized> From<*mut P> for VirtPtr<P>{
    fn from(value: *mut P) -> Self {
        Self(value)
    }
}

// ----------------------------

pub trait VirtToPhys{
    type Error;
    fn virt_to_phys<T>(&self, virt: VirtPtr<T>) -> Result<PhyPtr<T>, Self::Error>;
}
impl<'a, T: VirtToPhys> VirtToPhys for &'a T{
    type Error = T::Error;
    #[inline(always)]
    fn virt_to_phys<P>(&self, virt: VirtPtr<P>) -> Result<PhyPtr<P>, Self::Error> {
        (*self).virt_to_phys(virt)
    }
}
pub trait PhysToVirt{
    type Error;
    fn phys_to_virt<T>(&self, phys: PhyPtr<T>) -> Result<VirtPtr<T>, Self::Error>;
}
impl<'a, T: PhysToVirt> PhysToVirt for &'a T{
    type Error = T::Error;
    #[inline(always)]
    fn phys_to_virt<P>(&self, phys: PhyPtr<P>) -> Result<VirtPtr<P>, Self::Error> {
        (*self).phys_to_virt(phys)
    }
}

// ----------------------------

pub struct Identity;

impl PhysToVirt for Identity{
    type Error = core::convert::Infallible;
    #[inline(always)]
    fn phys_to_virt<T: ?Sized>(&self, phys: PhyPtr<T>) -> Result<VirtPtr<T>, Self::Error> {
       Ok(VirtPtr(phys.0)) 
    }
}

impl VirtToPhys for Identity{
    type Error = core::convert::Infallible;
    #[inline(always)]
    fn virt_to_phys<T: ?Sized>(&self, virt: VirtPtr<T>) -> Result<PhyPtr<T>, Self::Error> {
        Ok(PhyPtr(virt.0))
    }
}
// ----------------------------

pub struct PhysToVirtOffsetTranslation{
    pub range: core::ops::Range<usize>,
    pub offset: usize
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OutsideRange;

impl PhysToVirt for PhysToVirtOffsetTranslation{
    type Error = OutsideRange;

    fn phys_to_virt<T>(&self, phys: PhyPtr<T>) -> Result<VirtPtr<T>, Self::Error> {
        if self.range.contains(&(phys.0 as usize)){
            Err(OutsideRange)
        }else{
            let addr = (phys.0 as usize).wrapping_add(self.offset);
            Ok(VirtPtr(addr as *mut T))
        }
    }
}

impl PhysToVirtOffsetTranslation{
    pub const fn new(input_range: core::ops::Range<usize>, offset: usize) -> Self{
        Self { range: input_range, offset }
    }
}

// ----------------------------

pub struct VirtToPhysPageTranslation<T: PhysToVirt>{
    root: PhyPtr<PageTable>,
    trans: T,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PageTableAddressTranslationError<PE>{
    PhysicalToVirtualTranslationError(PE),
    PageTableMalformed,
    NotMapped,
}
impl<T: PhysToVirt> VirtToPhys for VirtToPhysPageTranslation<T>{
    type Error = PageTableAddressTranslationError<T::Error>;

    fn virt_to_phys<P>(&self, virt: VirtPtr<P>) -> Result<PhyPtr<P>, Self::Error> {
        let virt = virt.0 as *mut () as usize;
        let ppn2 = (virt >> (9 + 9 + 12)) & ((1 << 9) - 1);
        let ppn1 = (virt >> (9 + 12)) & ((1 << 9) - 1);
        let ppn0 = (virt >> (12)) & ((1 << 9) - 1);
        
        let root = unsafe{&*self.root.to_virt(&self.trans).map_err(PageTableAddressTranslationError::PhysicalToVirtualTranslationError)?.0};
        let lvl1 = root.entries[ppn2];
        if !lvl1.valid(){
            return Err(PageTableAddressTranslationError::NotMapped)
        }
        if lvl1.is_leaf(){
            let addr = extract_bits(lvl1.ppn() << 12, 12+9+9, 9) | extract_bits(virt as u64, 0, 12+9+9);
            return Ok(PhyPtr(addr as *mut P))
        }

        let curr = unsafe{&*PhyPtr((lvl1.ppn() << 12) as *mut PageTable).to_virt(&self.trans).map_err(PageTableAddressTranslationError::PhysicalToVirtualTranslationError)?.0};
        let lvl2 = curr.entries[ppn1];
        if !lvl2.valid(){
            return Err(PageTableAddressTranslationError::NotMapped)
        }
        if lvl2.is_leaf(){
            let addr = extract_bits(lvl1.ppn() << 12, 12+9+9, 9+9) | extract_bits(virt as u64, 0, 12+9);
            return Ok(PhyPtr(addr as *mut P))
        }

        let curr = unsafe{&*PhyPtr((lvl2.ppn() << 12) as *mut PageTable).to_virt(&self.trans).map_err(PageTableAddressTranslationError::PhysicalToVirtualTranslationError)?.0};
        let lvl3 = curr.entries[ppn0];
        if !lvl3.valid(){
            return Err(PageTableAddressTranslationError::NotMapped)
        }
        if lvl3.is_leaf(){
            let addr = extract_bits(lvl1.ppn() << 12, 12, 9+9+9) | extract_bits(virt as u64, 0, 12);
            Ok(PhyPtr(addr as *mut P))
        }else{
            Err(PageTableAddressTranslationError::PageTableMalformed)
        }
    }
}

impl<T: PhysToVirt> VirtToPhysPageTranslation<T>{
    /// # Safety 
    /// The provided root page pointer must not be valid, and not modified for the lifetime of this translator
    pub unsafe fn new(root: PhyPtr<PageTable>, trans: T) -> Self{
        Self { root, trans }
    }
}

// ----------------------------

impl PageTable{
    pub unsafe fn disp_table<T: core::fmt::Write>(root: *const PageTable, offset: u64, asid: usize, mut out: T) -> core::fmt::Result{
        let level = (root as u64 + offset) as *const PageTable;
        writeln!(out, "root:0x{root:p} asid:0x{asid:x}")?;
        fn disp_entry<T: core::fmt::Write>(out: &mut T, vpt: usize, entry: PageTableEntry, level: usize) -> core::fmt::Result{
            for _ in 0..=level{
                out.write_char('|')?;
            }
            let range = [1 << (9+9+12), 1<<(9+12), 1<<(12)];
            write!(out, "0x{:016x} -> 0x{:08x}..0x{:08x} ", vpt, entry.ppn() << 12, (entry.ppn() << 12) + range[level])?;
            out.write_str(if entry.strong_order() {"S0"} else {"--"})?;
            out.write_char(if entry.bufferable() {'b'} else {'_'})?;
            out.write_char(if entry.cacheable() {'c'} else {'_'})?;
            out.write_char(if entry.shareable() {'s'} else {'_'})?;
            out.write_char(if entry.trustable() {'t'} else {'_'})?;

            out.write_str(if entry.rsw() & 1 == 1 {"R0"} else {"--"})?;
            out.write_str(if entry.rsw() & 2 == 2 {"R1"} else {"--"})?;

            out.write_char(if entry.accessed() {'d'} else {'_'})?;
            out.write_char(if entry.dirty() {'a'} else {'_'})?;
            out.write_char(if entry.global() {'g'} else {'_'})?;
            out.write_char(if entry.user() {'u'} else {'_'})?;
            out.write_char(if entry.executable() {'x'} else {'_'})?;
            out.write_char(if entry.writable() {'w'} else {'_'})?;
            out.write_char(if entry.readable() {'r'} else {'_'})?;
            writeln!(out)?;
            Ok(())
        }
        for (i, entry) in (*level).entries.iter().enumerate(){
            if !entry.valid(){
                continue;
            }
            let vpt = i << (9+9+12);
            let vpt = (((vpt as isize) << (64-39)) >> (64-39)) as usize;
            if entry.is_leaf(){
                disp_entry(&mut out, vpt, *entry, 0)?;
                continue;
            }
            writeln!(out, "|0x{:08x}", entry.ppn() << 12)?;
            let level = unsafe { &*(((entry.ppn() << 12) + offset) as *const PageTable) };
            for (i, entry) in level.entries.iter().enumerate(){
                if !entry.valid(){
                    continue;
                }
                let vpt = vpt + (i << (9+12));
                if entry.is_leaf(){
                    disp_entry(&mut out, vpt, *entry, 1)?;
                    continue;
                }
                writeln!(out, "||0x{:08x}", entry.ppn() << 12)?;
                let level = unsafe { &*(((entry.ppn() << 12) + offset) as *const PageTable) };
                for (i, entry) in level.entries.iter().enumerate(){
                    if !entry.valid(){
                        continue;
                    }
                    let vpt = vpt + (i << 12);
                    if entry.is_leaf(){
                        disp_entry(&mut out, vpt, *entry, 2)?;
                        continue;
                    }else{
                        writeln!(out, "||invalid :3")?;
                    }
                }
            }
        }
        Ok(())
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct PageTableEntry(pub u64);

impl core::fmt::Debug for PageTableEntry{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PageTableEntry")
            .field("ppn", &self.ppn())
            .field("rsw", &self.rsw())
            .field("dirty", &self.dirty())
            .field("accessed", &self.accessed())
            .field("global", &self.global())
            .field("user", &self.user())
            .field("executable", &self.executable())
            .field("writable", &self.writable())
            .field("readable", &self.readable())
            .field("valid", &self.valid())
            .finish()
    }
}


impl PageTableEntry{

    pub const NON_LEAF: PageTableEntry = PageTableEntry::new().set_valid(true);

    pub const COM_EXEC: PageTableEntry = PageTableEntry::CACHEABLE_MEM.set_readable(true).set_executable(true).set_valid(true);
    pub const COM_RO: PageTableEntry = PageTableEntry::CACHEABLE_MEM.set_readable(true).set_valid(true);
    pub const COM_RW: PageTableEntry = PageTableEntry::CACHEABLE_MEM.set_readable(true).set_writable(true).set_valid(true);
    pub const COM_DEV: PageTableEntry = PageTableEntry::NON_BUFFERABLE_DEV.set_readable(true).set_writable(true).set_valid(true);

    pub const ACCESSED: PageTableEntry =  PageTableEntry::new().set_accessed(true);
    pub const DIRTY: PageTableEntry =  PageTableEntry::new().set_dirty(true);
    pub const DIRTY_ACCESSED: PageTableEntry = PageTableEntry::new().set_dirty(true).set_accessed(true);

    pub const CACHEABLE_MEM: PageTableEntry =  PageTableEntry::new().set_cacheable(true).set_bufferable(true);
    pub const NON_CACHEBLE_MEM: PageTableEntry =  PageTableEntry::new().set_bufferable(true).set_shareable(true);

    pub const BUFFERABLE_DEV: PageTableEntry =  PageTableEntry::new().set_bufferable(true).set_strong_order(true).set_shareable(true);
    pub const NON_BUFFERABLE_DEV: PageTableEntry =  PageTableEntry::new().set_strong_order(true).set_shareable(true);

    pub const fn new() -> Self{
        Self(0)
    }

    pub const fn ppn(&self) -> u64{
        (self.0 >> 10) & ((1<<50) - 1)
    }

    pub const fn rsw(&self) -> u64{
        (self.0 >> 8) & 0b11
    }

    pub const fn dirty(&self) -> bool{
        (self.0 >> 7) & 0b1 == 1
    }

    pub const fn accessed(&self) -> bool{
        (self.0 >> 6) & 0b1 == 1
    }

    pub const fn global(&self) -> bool{
        (self.0 >> 5) & 0b1 == 1
    }

    pub const fn user(&self) -> bool{
        (self.0 >> 4) & 0b1 == 1
    }

    pub const fn executable(&self) -> bool{
        (self.0 >> 3) & 0b1 == 1
    }
    
    pub const fn writable(&self) -> bool{
        (self.0 >> 2) & 0b1 == 1
    }

    pub const fn readable(&self) -> bool{
        (self.0 >> 1) & 0b1 == 1
    }

    pub const fn valid(&self) -> bool{
        (self.0 >> 0) & 0b1 == 1
    }

    pub const fn perms(&self) -> u64{
        (self.0 >> 1) & 0b111
    }

    pub const fn strong_order(&self) -> bool{
        (self.0 >> 63) & 0b1 == 1
    }

    pub const fn cacheable(&self) -> bool{
        (self.0 >> 62) & 0b1 == 1
    }

    pub const fn bufferable(&self) -> bool{
        (self.0 >> 61) & 0b1 == 1
    }

    pub const fn shareable(&self) -> bool{
        (self.0 >> 60) & 0b1 == 1
    }

    pub const fn trustable(&self) -> bool{
        (self.0 >> 59) & 0b1 == 1
    }

    pub const fn is_leaf(self) -> bool{
        self.perms() != 0
    }




    pub const fn set_ppn(self, ppn: u64) -> Self {
        Self(set_bits(self.0, 10, 48, ppn))
    }

    pub const fn set_rsw(self, rsw: u64) -> Self {
        Self(set_bits(self.0, 8, 2, rsw))
    }

    pub const fn set_dirty(self, dirty: bool) -> Self {
        Self(set_bits(self.0, 7, 1, if dirty {1} else {0}))
    }

    pub const fn set_accessed(self, accessed: bool) -> Self {
        Self(set_bits(self.0, 6, 1, if accessed {1} else {0}))
    }

    pub const fn set_global(self, global: bool) -> Self {
        Self(set_bits(self.0, 5, 1, if global {1} else {0}))
    }

    pub const fn set_user(self, user: bool) -> Self {
        Self(set_bits(self.0, 4, 1, if user {1} else {0}))
    }

    pub const fn set_executable(self, executable: bool) -> Self {
        Self(set_bits(self.0, 3, 1, if executable {1} else {0}))
    }
    
    pub const fn set_writable(self, writable: bool) -> Self {
        Self(set_bits(self.0, 2, 1, if writable {1} else {0}))
    }

    pub const fn set_readable(self, readable: bool) -> Self {
        Self(set_bits(self.0, 1, 1, if readable {1} else {0}))
    }

    pub const fn set_valid(self, valid: bool) -> Self {
        Self(set_bits(self.0, 0, 1, if valid {1} else {0}))
    }

    pub const fn set_perms(self, perms: u64) -> Self {
        Self(set_bits(self.0, 1, 3, perms))
    }

    pub const fn set_strong_order(self, strong_order: bool) -> Self {
        Self(set_bits(self.0, 63, 1, if strong_order {1} else {0}))
    }

    pub const fn set_cacheable(self, cacheable: bool) -> Self {
        Self(set_bits(self.0, 62, 1, if cacheable {1} else {0}))
    }

    pub const fn set_bufferable(self, bufferable: bool) -> Self {
        Self(set_bits(self.0, 61, 1, if bufferable {1} else {0}))
    }

    pub const fn set_shareable(self, shareable: bool) -> Self {
        Self(set_bits(self.0, 60, 1, if shareable {1} else {0}))
    }

    pub const fn set_trustable(self, trustable: bool) -> Self {
        Self(set_bits(self.0, 59, 1, if trustable {1} else {0}))
    }
}

const fn set_bits(val: u64, start: u64, len: u64, bits: u64) -> u64{
    let mask = ((1<<len)-1) << start;    
    (val & !mask) | (bits << start) & mask 
}

const fn extract_bits(val: u64, start: u64, len: u64) -> u64{
    let mask  = ((1<<len)-1) << start;
    val & mask
}

impl core::ops::BitOr for PageTableEntry{
    type Output = PageTableEntry;

    fn bitor(self, rhs: PageTableEntry) -> Self::Output {
        PageTableEntry(self.0 | rhs.0)
    }
}