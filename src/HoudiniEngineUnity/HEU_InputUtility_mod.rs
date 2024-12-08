#[cfg(feature = "HoudiniEngineUnity+HEU_InputUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_InputUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_InputUtility =>
    "HoudiniEngineUnity"."HEU_InputUtility"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_InputUtility")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_InputUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputUtility")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_InputUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputUtility")]
impl crate::HoudiniEngineUnity::HEU_InputUtility {}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_InputUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
