use core::fmt::Debug;

/// General interface for syncing state
pub trait SyncState {
    type Error: Debug;

    /// (Re)writes the internal state (mode, polarity, output state) of all banks to the configuration registers.
    /// May be useful after power resenting the expander IC to ensure the software matches the
    /// hardware state.
    fn sync_state(&self) -> Result<(), Self::Error>;
}
