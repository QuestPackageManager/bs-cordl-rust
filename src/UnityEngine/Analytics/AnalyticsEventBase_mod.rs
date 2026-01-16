#[cfg(feature = "cordl_class_UnityEngine+Analytics+AnalyticsEventBase")]
#[repr(C)]
#[derive(Debug)]
pub struct AnalyticsEventBase {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub eventName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub eventVersion: i32,
    pub eventPrefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub sendEventOptions: crate::UnityEngine::Analytics::SendEventOptions,
}
#[cfg(feature = "cordl_class_UnityEngine+Analytics+AnalyticsEventBase")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Analytics::AnalyticsEventBase {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Analytics";
    const CLASS_NAME: &'static str = "AnalyticsEventBase";
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
#[cfg(feature = "UnityEngine+Analytics+AnalyticsEventBase")]
impl std::ops::Deref for crate::UnityEngine::Analytics::AnalyticsEventBase {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Analytics+AnalyticsEventBase")]
impl std::ops::DerefMut for crate::UnityEngine::Analytics::AnalyticsEventBase {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Analytics+AnalyticsEventBase")]
impl crate::UnityEngine::Analytics::AnalyticsEventBase {
    pub fn New(
        eventName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        eventVersion: i32,
        sendEventOptions: crate::UnityEngine::Analytics::SendEventOptions,
        eventPrefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (eventName, eventVersion, sendEventOptions, eventPrefix),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        eventName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        eventVersion: i32,
        sendEventOptions: crate::UnityEngine::Analytics::SendEventOptions,
        eventPrefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
                            i32,
                            crate::UnityEngine::Analytics::SendEventOptions,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (eventName, eventVersion, sendEventOptions, eventPrefix),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Analytics+AnalyticsEventBase")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Analytics::AnalyticsEventBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
