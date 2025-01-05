#[cfg(feature = "HoudiniEngineUnity+HEU_PlatformWin")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_PlatformWin {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PlatformWin")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_PlatformWin =>
    "HoudiniEngineUnity"."HEU_PlatformWin"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_PlatformWin")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_PlatformWin {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PlatformWin")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_PlatformWin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PlatformWin")]
impl crate::HoudiniEngineUnity::HEU_PlatformWin {}
#[cfg(feature = "HoudiniEngineUnity+HEU_PlatformWin")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_PlatformWin {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
