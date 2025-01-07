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
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::FtpStatusCode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "FtpStatusCode";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "System+Net+FtpStatusCode")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::Net::FtpStatusCode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Net+FtpStatusCode")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::Net::FtpStatusCode {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "System+Net+FtpStatusCode")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::Net::FtpStatusCode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "System+Net+FtpStatusCode")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::Net::FtpStatusCode {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
