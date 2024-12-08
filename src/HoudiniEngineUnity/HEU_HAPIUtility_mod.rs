#[cfg(feature = "HoudiniEngineUnity+HEU_HAPIUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_HAPIUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HAPIUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_HAPIUtility =>
    "HoudiniEngineUnity"."HEU_HAPIUtility"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_HAPIUtility")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_HAPIUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HAPIUtility")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_HAPIUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HAPIUtility")]
impl crate::HoudiniEngineUnity::HEU_HAPIUtility {}
#[cfg(feature = "HoudiniEngineUnity+HEU_HAPIUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_HAPIUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
