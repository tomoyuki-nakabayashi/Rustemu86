//! Processor state stored in the EFLAGS register.

bitflags! {
    /// The EFLAGS register.
    pub struct EFlags: u32 {
        /// Determines the order in which strings are processed.
        const DIRECTION_FLAG = 1 << 10;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bitflag() {
        let mut eflags = EFlags::empty();
        eflags.set(EFlags::DIRECTION_FLAG, true);
        assert!(!(eflags & EFlags::DIRECTION_FLAG).is_empty());

        eflags.set(EFlags::DIRECTION_FLAG, false);
        assert!((eflags & EFlags::DIRECTION_FLAG).is_empty());
    }
}
