#[cfg(feature = "UnityEngine+UIElements+UIR+ShaderInfoStorageRGBAFloat")]
#[repr(C)]
#[derive(Debug)]
pub struct ShaderInfoStorageRGBAFloat {
    __cordl_parent: crate::UnityEngine::UIElements::UIR::ShaderInfoStorage_1<
        crate::UnityEngine::Color,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+ShaderInfoStorageRGBAFloat")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::ShaderInfoStorageRGBAFloat =>
    "UnityEngine.UIElements.UIR"."ShaderInfoStorageRGBAFloat"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+ShaderInfoStorageRGBAFloat")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::UIR::ShaderInfoStorageRGBAFloat {
    type Target = crate::UnityEngine::UIElements::UIR::ShaderInfoStorage_1<
        crate::UnityEngine::Color,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+ShaderInfoStorageRGBAFloat")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::UIR::ShaderInfoStorageRGBAFloat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+ShaderInfoStorageRGBAFloat")]
impl crate::UnityEngine::UIElements::UIR::ShaderInfoStorageRGBAFloat {
    #[cfg(feature = "UnityEngine+UIElements+UIR+ShaderInfoStorageRGBAFloat+__c")]
    pub type __c = crate::UnityEngine::UIElements::UIR::ShaderInfoStorageRGBAFloat___c;
    pub fn New(
        initialSize: i32,
        maxSize: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initialSize, maxSize))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        initialSize: i32,
        maxSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initialSize, maxSize))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+ShaderInfoStorageRGBAFloat")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::ShaderInfoStorageRGBAFloat {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
