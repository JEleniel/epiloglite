use flagset::flags;

flags! {
    pub enum DatabaseFlags: u8 {
        /// Temporarily locked for maintenance
        Maintenance = 0b00000001,
        /// Dirty (needs to be flushed)
        Dirty = 0b00000010,
    }
}

flags! {
    /// Flags describing the state of a page.
    pub enum PageFlags: u8 {
        /// Page is full
        FULL = 0b0000_0001,
        /// Page is dirty
        DIRTY = 0b0000_0010,
        /// Page is free
        FREE = 0b0000_0100,
    }
}
