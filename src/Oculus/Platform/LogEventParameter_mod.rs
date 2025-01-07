#[cfg(feature = "Oculus+Platform+LogEventParameter")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LogEventParameter {
    #[default]
    AdType = 13i32,
    EventName = 15i32,
    ImplicitlyLogged = 17i32,
    InBackground = 18i32,
    LogTime = 16i32,
    SessionId = 30i32,
    Unknown = 0i32,
    VrContent = 4i32,
    VrContentId = 5i32,
    VrContentTitle = 22i32,
    VrContentType = 3i32,
    VrCurrency = 1i32,
    VrDescription = 12i32,
    VrIapHasFreeTrial = 27i32,
    VrIapIsStartTrial = 26i32,
    VrIapProductType = 21i32,
    VrIapSubsPeriod = 25i32,
    VrIapTrialPeriod = 28i32,
    VrIapTrialPrice = 29i32,
    VrLevel = 11i32,
    VrMaxRatingValue = 8i32,
    VrNumItems = 10i32,
    VrOrderId = 14i32,
    VrPaymentInfoAvailable = 9i32,
    VrPushAction = 20i32,
    VrPushCampaign = 19i32,
    VrRegistrationMethod = 2i32,
    VrSearchString = 6i32,
    VrSuccess = 7i32,
    VrTransactionDate = 24i32,
    VrTransactionId = 23i32,
}
#[cfg(feature = "Oculus+Platform+LogEventParameter")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Platform::LogEventParameter {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "LogEventParameter";
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
#[cfg(feature = "Oculus+Platform+LogEventParameter")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Oculus::Platform::LogEventParameter {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Oculus+Platform+LogEventParameter")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Oculus::Platform::LogEventParameter {
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
#[cfg(feature = "Oculus+Platform+LogEventParameter")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Oculus::Platform::LogEventParameter {
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
#[cfg(feature = "Oculus+Platform+LogEventParameter")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Oculus::Platform::LogEventParameter {
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
