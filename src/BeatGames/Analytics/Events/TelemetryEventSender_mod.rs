#[cfg(feature = "cordl_class_BeatGames+Analytics+Events+TelemetryEventSender")]
#[repr(C)]
#[derive(Debug)]
pub struct TelemetryEventSender {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _analyticsManager: quest_hook::libil2cpp::Gc<
        crate::OSCE::Analytics::AnalyticsManager,
    >,
}
#[cfg(feature = "cordl_class_BeatGames+Analytics+Events+TelemetryEventSender")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatGames::Analytics::Events::TelemetryEventSender {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatGames.Analytics.Events";
    const CLASS_NAME: &'static str = "TelemetryEventSender";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "BeatGames+Analytics+Events+TelemetryEventSender")]
impl std::ops::Deref for crate::BeatGames::Analytics::Events::TelemetryEventSender {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatGames+Analytics+Events+TelemetryEventSender")]
impl std::ops::DerefMut for crate::BeatGames::Analytics::Events::TelemetryEventSender {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatGames+Analytics+Events+TelemetryEventSender")]
impl crate::BeatGames::Analytics::Events::TelemetryEventSender {
    pub fn New(
        analyticsManager: quest_hook::libil2cpp::Gc<
            crate::OSCE::Analytics::AnalyticsManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (analyticsManager))?;
        Ok(__cordl_object.into())
    }
    pub fn SendTelemetryEvent(
        &mut self,
        eventName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        eventFamily: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        stringValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        intValue: i32,
        durationMS: i32,
        tags: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppString,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("SendTelemetryEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SendTelemetryEvent", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (eventName, eventFamily, stringValue, intValue, durationMS, tags),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        analyticsManager: quest_hook::libil2cpp::Gc<
            crate::OSCE::Analytics::AnalyticsManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::OSCE::Analytics::AnalyticsManager,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (analyticsManager))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatGames+Analytics+Events+TelemetryEventSender")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatGames::Analytics::Events::TelemetryEventSender {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
