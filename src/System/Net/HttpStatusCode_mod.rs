#[cfg(feature = "cordl_class_System+Net+HttpStatusCode")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Copy", derive(Copy))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[cfg_attr(feature = "derive_Eq", derive(Eq))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[repr(i32)]
pub enum HttpStatusCode {
    #[cfg_attr(feature = "derive_Default", default)]
    Accepted = 202i32,
    AlreadyReported = 208i32,
    Ambiguous = 300i32,
    BadGateway = 502i32,
    BadRequest = 400i32,
    Conflict = 409i32,
    Continue = 100i32,
    Created = 201i32,
    EarlyHints = 103i32,
    ExpectationFailed = 417i32,
    FailedDependency = 424i32,
    Forbidden = 403i32,
    Found = 302i32,
    GatewayTimeout = 504i32,
    Gone = 410i32,
    HttpVersionNotSupported = 505i32,
    IMUsed = 226i32,
    InsufficientStorage = 507i32,
    InternalServerError = 500i32,
    LengthRequired = 411i32,
    Locked = 423i32,
    LoopDetected = 508i32,
    MethodNotAllowed = 405i32,
    MisdirectedRequest = 421i32,
    Moved = 301i32,
    MultiStatus = 207i32,
    NetworkAuthenticationRequired = 511i32,
    NoContent = 204i32,
    NonAuthoritativeInformation = 203i32,
    NotAcceptable = 406i32,
    NotExtended = 510i32,
    NotFound = 404i32,
    NotImplemented = 501i32,
    NotModified = 304i32,
    OK = 200i32,
    PartialContent = 206i32,
    PaymentRequired = 402i32,
    PermanentRedirect = 308i32,
    PreconditionFailed = 412i32,
    PreconditionRequired = 428i32,
    Processing = 102i32,
    ProxyAuthenticationRequired = 407i32,
    RedirectKeepVerb = 307i32,
    RedirectMethod = 303i32,
    RequestEntityTooLarge = 413i32,
    RequestHeaderFieldsTooLarge = 431i32,
    RequestTimeout = 408i32,
    RequestUriTooLong = 414i32,
    RequestedRangeNotSatisfiable = 416i32,
    ResetContent = 205i32,
    ServiceUnavailable = 503i32,
    SwitchingProtocols = 101i32,
    TooManyRequests = 429i32,
    Unauthorized = 401i32,
    UnavailableForLegalReasons = 451i32,
    UnprocessableEntity = 422i32,
    UnsupportedMediaType = 415i32,
    Unused = 306i32,
    UpgradeRequired = 426i32,
    UseProxy = 305i32,
    VariantAlsoNegotiates = 506i32,
}
#[cfg(feature = "cordl_class_System+Net+HttpStatusCode")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::HttpStatusCode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "HttpStatusCode";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_System+Net+HttpStatusCode")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::Net::HttpStatusCode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+Net+HttpStatusCode")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::Net::HttpStatusCode {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_System+Net+HttpStatusCode")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::Net::HttpStatusCode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_System+Net+HttpStatusCode")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::Net::HttpStatusCode {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
