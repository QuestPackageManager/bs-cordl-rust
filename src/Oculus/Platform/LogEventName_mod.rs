#[cfg(feature = "Oculus+Platform+LogEventName")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogEventName {
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
#[cfg(feature = "Oculus+Platform+LogEventName")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::LogEventName =>
    "Oculus.Platform"."LogEventName"
);
