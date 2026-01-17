#[cfg(feature = "cordl_class_BeatGames+Analytics+AnalyticsEventsDispatcher")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct AnalyticsEventsDispatcher {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _navigationEventSender:
        quest_hook::libil2cpp::Gc<crate::BeatGames::Analytics::NavigationEventSender>,
    pub _sessionEventSender:
        quest_hook::libil2cpp::Gc<crate::BeatGames::Analytics::Events::SessionEventSender>,
    pub _analyticsManager: quest_hook::libil2cpp::Gc<crate::OSCE::Analytics::AnalyticsManager>,
    pub _telemetryEventSender:
        quest_hook::libil2cpp::Gc<crate::BeatGames::Analytics::Events::TelemetryEventSender>,
}
#[cfg(feature = "cordl_class_BeatGames+Analytics+AnalyticsEventsDispatcher")]
unsafe impl quest_hook::libil2cpp::Type for crate::BeatGames::Analytics::AnalyticsEventsDispatcher {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatGames.Analytics";
    const CLASS_NAME: &'static str = "AnalyticsEventsDispatcher";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "BeatGames+Analytics+AnalyticsEventsDispatcher")]
impl std::ops::Deref for crate::BeatGames::Analytics::AnalyticsEventsDispatcher {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatGames+Analytics+AnalyticsEventsDispatcher")]
impl std::ops::DerefMut for crate::BeatGames::Analytics::AnalyticsEventsDispatcher {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatGames+Analytics+AnalyticsEventsDispatcher")]
impl crate::BeatGames::Analytics::AnalyticsEventsDispatcher {
    pub fn New(
        analyticsManager: quest_hook::libil2cpp::Gc<crate::OSCE::Analytics::AnalyticsManager>,
        oculusAuthToken: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (analyticsManager, oculusAuthToken))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        analyticsManager: quest_hook::libil2cpp::Gc<crate::OSCE::Analytics::AnalyticsManager>,
        oculusAuthToken: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::OSCE::Analytics::AnalyticsManager>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (analyticsManager, oculusAuthToken))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Navigation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatGames::Analytics::NavigationEventSender>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::BeatGames::Analytics::NavigationEventSender,
                        >,
                        0usize,
                    >("get_Navigation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Navigation", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatGames::Analytics::NavigationEventSender,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Session(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatGames::Analytics::Events::SessionEventSender>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::BeatGames::Analytics::Events::SessionEventSender,
                    >, 0usize>("get_Session")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_Session",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatGames::Analytics::Events::SessionEventSender,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Telemetry(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatGames::Analytics::Events::TelemetryEventSender>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::BeatGames::Analytics::Events::TelemetryEventSender,
                    >, 0usize>("get_Telemetry")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_Telemetry",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatGames::Analytics::Events::TelemetryEventSender,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatGames+Analytics+AnalyticsEventsDispatcher")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatGames::Analytics::AnalyticsEventsDispatcher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
