#[cfg(feature = "OVRTask")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRTask {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRTask")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OVRTask => ""."OVRTask"
);
#[cfg(feature = "OVRTask")]
impl std::ops::Deref for OVRTask {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTask")]
impl std::ops::DerefMut for OVRTask {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTask")]
impl OVRTask {}
#[cfg(feature = "OVRTask")]
impl quest_hook::libil2cpp::ObjectType for OVRTask {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
