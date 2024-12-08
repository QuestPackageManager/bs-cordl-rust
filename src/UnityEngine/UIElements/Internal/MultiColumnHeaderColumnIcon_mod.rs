#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnHeaderColumnIcon")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiColumnHeaderColumnIcon {
    __cordl_parent: crate::UnityEngine::UIElements::Image,
    pub _isImageInline_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnHeaderColumnIcon")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::Internal::MultiColumnHeaderColumnIcon =>
    "UnityEngine.UIElements.Internal"."MultiColumnHeaderColumnIcon"
);
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnHeaderColumnIcon")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumnIcon {
    type Target = crate::UnityEngine::UIElements::Image;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnHeaderColumnIcon")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumnIcon {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnHeaderColumnIcon")]
impl crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumnIcon {
    pub fn __ctor_b__5_0(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::CustomStyleResolvedEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<.ctor>b__5_0", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn get_isImageInline(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isImageInline", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateClassList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateClassList", ())?;
        Ok(__cordl_ret)
    }
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
    pub fn set_isImageInline(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isImageInline", (value))?;
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
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnHeaderColumnIcon")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumnIcon {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
