#[cfg(feature = "cordl_class_Oculus+Platform+LogEventName")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Copy", derive(Copy))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[cfg_attr(feature = "derive_Eq", derive(Eq))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[repr(i32)]
pub enum LogEventName {
    #[cfg_attr(feature = "derive_Default", default)]
    AdClick = 1i32,
    AdImpression = 2i32,
    Contact = 5i32,
    CustomizeProduct = 6i32,
    Donate = 7i32,
    FindLocation = 8i32,
    Schedule = 10i32,
    SmartTrial = 12i32,
    SubmitApplication = 13i32,
    Subscribe = 14i32,
    SubscriptionFailed = 29i32,
    SubscriptionInitiatedCheckout = 28i32,
    SubscriptionRestore = 30i32,
    Unknown = 0i32,
    VrAchievementUnlocked = 32i32,
    VrActivateApp = 36i32,
    VrAddPaymentInfo = 20i32,
    VrAddToCart = 21i32,
    VrAddToWishlist = 22i32,
    VrCatalogUpdate = 25i32,
    VrCompleteRegistration = 3i32,
    VrContentView = 15i32,
    VrDeactivateApp = 37i32,
    VrInitiatedCheckout = 23i32,
    VrLevelAchieved = 31i32,
    VrObtainPushToken = 34i32,
    VrPurchase = 24i32,
    VrPurchaseFailed = 26i32,
    VrPurchaseRestored = 27i32,
    VrPushOpened = 35i32,
    VrRate = 9i32,
    VrSdkBackgroundStatusAvailable = 17i32,
    VrSdkBackgroundStatusDenied = 18i32,
    VrSdkBackgroundStatusRestricted = 19i32,
    VrSdkInitialize = 16i32,
    VrSearch = 11i32,
    VrSpentCredits = 33i32,
    VrTutorialCompletion = 4i32,
}
#[cfg(feature = "cordl_class_Oculus+Platform+LogEventName")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Platform::LogEventName {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "LogEventName";
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
#[cfg(feature = "cordl_class_Oculus+Platform+LogEventName")]
unsafe impl quest_hook::libil2cpp::Argument for crate::Oculus::Platform::LogEventName {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Oculus+Platform+LogEventName")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::Oculus::Platform::LogEventName {
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
#[cfg(feature = "cordl_class_Oculus+Platform+LogEventName")]
unsafe impl quest_hook::libil2cpp::Returned for crate::Oculus::Platform::LogEventName {
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
#[cfg(feature = "cordl_class_Oculus+Platform+LogEventName")]
unsafe impl quest_hook::libil2cpp::Return for crate::Oculus::Platform::LogEventName {
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
