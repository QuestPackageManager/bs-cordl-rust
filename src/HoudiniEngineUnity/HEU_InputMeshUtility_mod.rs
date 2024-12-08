#[cfg(feature = "HoudiniEngineUnity+HEU_InputMeshUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_InputMeshUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputMeshUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_InputMeshUtility =>
    "HoudiniEngineUnity"."HEU_InputMeshUtility"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_InputMeshUtility")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_InputMeshUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputMeshUtility")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_InputMeshUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputMeshUtility")]
impl crate::HoudiniEngineUnity::HEU_InputMeshUtility {}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputMeshUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_InputMeshUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
