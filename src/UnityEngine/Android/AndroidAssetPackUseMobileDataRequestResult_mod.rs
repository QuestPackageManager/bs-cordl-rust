#[cfg(feature = "UnityEngine+Android+AndroidAssetPackUseMobileDataRequestResult")]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidAssetPackUseMobileDataRequestResult {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _allowed_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPackUseMobileDataRequestResult")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Android::AndroidAssetPackUseMobileDataRequestResult =>
    "UnityEngine.Android"."AndroidAssetPackUseMobileDataRequestResult"
);
#[cfg(feature = "UnityEngine+Android+AndroidAssetPackUseMobileDataRequestResult")]
impl std::ops::Deref
for crate::UnityEngine::Android::AndroidAssetPackUseMobileDataRequestResult {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPackUseMobileDataRequestResult")]
impl std::ops::DerefMut
for crate::UnityEngine::Android::AndroidAssetPackUseMobileDataRequestResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPackUseMobileDataRequestResult")]
impl crate::UnityEngine::Android::AndroidAssetPackUseMobileDataRequestResult {
    pub fn New(
        allowed: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (allowed))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        allowed: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (allowed))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPackUseMobileDataRequestResult")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Android::AndroidAssetPackUseMobileDataRequestResult {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
