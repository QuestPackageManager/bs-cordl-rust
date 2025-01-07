#[cfg(feature = "ToneMappingExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct ToneMappingExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ToneMappingExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ToneMappingExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ToneMappingExtensions";
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
#[cfg(feature = "ToneMappingExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::ToneMappingExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ToneMappingExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::ToneMappingExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ToneMappingExtensions")]
impl crate::GlobalNamespace::ToneMappingExtensions {
    pub fn SetShaderKeyword(
        toneMapping: crate::GlobalNamespace::ToneMapping,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetShaderKeyword", (toneMapping))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ToneMappingExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ToneMappingExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
