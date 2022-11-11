/// Error comprises all error types used in the
/// Psoc6_hal. The errors are annotated with a comment
/// to indicate where they are used.
#[derive(Debug, PartialEq)]
pub enum Error{
    //ipc errors
    AcquisitionFailed,
    ReleaseFailed,
    SendFailed,
    ReceiveFailed,
    ChannelBusy,
    //Semaphore errors.
    FlagUnknown,
    FlagLocked,
    FlagCannotBeClearedIsNotSet,
    AttemptingToClearUnknownFlag,
    AttemptingToSetUnknownFlag,
    //drivers::clocks
    NoError,
    UnknownLfClkSource,
    UnknownPathSource,
    FllCouldNotBeConfigured,
    FllCouldNotBeStarted,
    FllStartupCouldNotBeCompletedBeforeTimeout,
    FllStartupFailedCcoNotReady,
    FllStartupFailedFllCouldNotBeLocked,
    //GPIO
    AttemptingToSetUnknownInterrupt,
    // cpuss erros
    VectorTableBaseAddressIncludesReservedRange,
}
