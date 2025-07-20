#[cfg(feature = "UnityEngine+AddressableAssets+Utility+DiagnosticInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct DiagnosticInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub DisplayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub ObjectId: i32,
    pub Dependencies: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
}
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+DiagnosticInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::AddressableAssets::Utility::DiagnosticInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.AddressableAssets.Utility";
    const CLASS_NAME: &'static str = "DiagnosticInfo";
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
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+DiagnosticInfo")]
impl std::ops::Deref for crate::UnityEngine::AddressableAssets::Utility::DiagnosticInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+DiagnosticInfo")]
impl std::ops::DerefMut
for crate::UnityEngine::AddressableAssets::Utility::DiagnosticInfo {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+DiagnosticInfo")]
impl crate::UnityEngine::AddressableAssets::Utility::DiagnosticInfo {
    pub fn CreateEvent(
        &mut self,
        category: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        eventType: crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventType,
        frame: i32,
        val: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEvent,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventType,
                            i32,
                            i32,
                        ),
                        crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEvent,
                        4usize,
                    >("CreateEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateEvent", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEvent = unsafe {
            cordl_method_info.invoke_unchecked(self, (category, eventType, frame, val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+DiagnosticInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::Utility::DiagnosticInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
