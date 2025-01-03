#[cfg(feature = "HardwareCategories")]
#[repr(C)]
#[derive(Debug)]
pub struct HardwareCategories {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HardwareCategories")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::HardwareCategories => ""
    ."HardwareCategories"
);
#[cfg(feature = "HardwareCategories")]
impl std::ops::Deref for crate::GlobalNamespace::HardwareCategories {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HardwareCategories")]
impl std::ops::DerefMut for crate::GlobalNamespace::HardwareCategories {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HardwareCategories")]
impl crate::GlobalNamespace::HardwareCategories {
    pub fn GetHardwareCategory() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::HardwareCategory,
    > {
        let __cordl_ret: crate::GlobalNamespace::HardwareCategory = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHardwareCategory", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHardwareCategoryWithEditorOverride() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::HardwareCategory,
    > {
        let __cordl_ret: crate::GlobalNamespace::HardwareCategory = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHardwareCategoryWithEditorOverride", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HardwareCategories")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::HardwareCategories {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
