#[cfg(feature = "HoudiniEngineUnity+HEU_SessionPDG")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_SessionPDG {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionPDG")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_SessionPDG =>
    "HoudiniEngineUnity"."HEU_SessionPDG"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionPDG")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_SessionPDG {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionPDG")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_SessionPDG {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionPDG")]
impl crate::HoudiniEngineUnity::HEU_SessionPDG {}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionPDG")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_SessionPDG {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
