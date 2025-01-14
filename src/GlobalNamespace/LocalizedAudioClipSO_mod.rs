#[cfg(feature = "LocalizedAudioClipSO")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalizedAudioClipSO {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _localizedAudioClipInfo: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::LocalizedAudioClipSO_LocalizedAudioClipInfo,
            >,
        >,
    >,
    pub _lastLocalizedAudioClipInfo: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LocalizedAudioClipSO_LocalizedAudioClipInfo,
    >,
}
#[cfg(feature = "LocalizedAudioClipSO")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::LocalizedAudioClipSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LocalizedAudioClipSO";
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
#[cfg(feature = "LocalizedAudioClipSO")]
impl std::ops::Deref for crate::GlobalNamespace::LocalizedAudioClipSO {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LocalizedAudioClipSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::LocalizedAudioClipSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LocalizedAudioClipSO")]
impl crate::GlobalNamespace::LocalizedAudioClipSO {
    #[cfg(feature = "LocalizedAudioClipSO+LocalizedAudioClipInfo")]
    pub type LocalizedAudioClipInfo = crate::GlobalNamespace::LocalizedAudioClipSO_LocalizedAudioClipInfo;
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_localizedAudioClip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
                0usize,
            >("get_localizedAudioClip")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_localizedAudioClip", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip> = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LocalizedAudioClipSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::LocalizedAudioClipSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LocalizedAudioClipSO+LocalizedAudioClipInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalizedAudioClipSO_LocalizedAudioClipInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub language: crate::BGLib::Polyglot::Language,
    pub localizedAudioClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
}
#[cfg(feature = "LocalizedAudioClipSO+LocalizedAudioClipInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::LocalizedAudioClipSO_LocalizedAudioClipInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LocalizedAudioClipSO/LocalizedAudioClipInfo";
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
#[cfg(feature = "LocalizedAudioClipSO+LocalizedAudioClipInfo")]
impl std::ops::Deref
for crate::GlobalNamespace::LocalizedAudioClipSO_LocalizedAudioClipInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LocalizedAudioClipSO+LocalizedAudioClipInfo")]
impl std::ops::DerefMut
for crate::GlobalNamespace::LocalizedAudioClipSO_LocalizedAudioClipInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LocalizedAudioClipSO+LocalizedAudioClipInfo")]
impl crate::GlobalNamespace::LocalizedAudioClipSO_LocalizedAudioClipInfo {
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LocalizedAudioClipSO+LocalizedAudioClipInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LocalizedAudioClipSO_LocalizedAudioClipInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
