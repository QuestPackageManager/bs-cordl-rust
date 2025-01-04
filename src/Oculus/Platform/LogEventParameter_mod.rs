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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::LogEventParameter =>
    "Oculus.Platform"."LogEventParameter"
);
