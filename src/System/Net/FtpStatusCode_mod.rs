#[cfg(feature = "System+Net+FtpStatusCode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FtpStatusCode {
    #[default]
    AccountNeeded = 532i32,
    ActionAbortedLocalProcessingError = 451i32,
    ActionAbortedUnknownPageType = 551i32,
    ActionNotTakenFileUnavailable = 550i32,
    ActionNotTakenFileUnavailableOrBusy = 450i32,
    ActionNotTakenFilenameNotAllowed = 553i32,
    ActionNotTakenInsufficientSpace = 452i32,
    ArgumentSyntaxError = 501i32,
    BadCommandSequence = 503i32,
    CantOpenData = 425i32,
    ClosingControl = 221i32,
    ClosingData = 226i32,
    CommandExtraneous = 202i32,
    CommandNotImplemented = 502i32,
    CommandOK = 200i32,
    CommandSyntaxError = 500i32,
    ConnectionClosed = 426i32,
    DataAlreadyOpen = 125i32,
    DirectoryStatus = 212i32,
    EnteringPassive = 227i32,
    FileActionAborted = 552i32,
    FileActionOK = 250i32,
    FileCommandPending = 350i32,
    FileStatus = 213i32,
    LoggedInProceed = 230i32,
    NeedLoginAccount = 332i32,
    NotLoggedIn = 530i32,
    OpeningData = 150i32,
    PathnameCreated = 257i32,
    RestartMarker = 110i32,
    SendPasswordCommand = 331i32,
    SendUserCommand = 220i32,
    ServerWantsSecureSession = 234i32,
    ServiceNotAvailable = 421i32,
    ServiceTemporarilyNotAvailable = 120i32,
    SystemType = 215i32,
    Undefined = 0i32,
}
#[cfg(feature = "System+Net+FtpStatusCode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::FtpStatusCode => "System.Net"
    ."FtpStatusCode"
);
