#[cfg(feature = "LightGroupTranslationYTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct LightGroupTranslationYTransform {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
}
#[cfg(feature = "LightGroupTranslationYTransform")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LightGroupTranslationYTransform => ""
    ."LightGroupTranslationYTransform"
);
#[cfg(feature = "LightGroupTranslationYTransform")]
impl std::ops::Deref for LightGroupTranslationYTransform {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightGroupTranslationYTransform")]
impl std::ops::DerefMut for LightGroupTranslationYTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightGroupTranslationYTransform")]
impl LightGroupTranslationYTransform {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "LightGroupTranslationYTransform")]
impl quest_hook::libil2cpp::ObjectType for LightGroupTranslationYTransform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
