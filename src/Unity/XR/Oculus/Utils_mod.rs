#[cfg(feature = "Unity+XR+Oculus+Utils")]
#[repr(C)]
#[derive(Debug)]
pub struct Utils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+XR+Oculus+Utils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::XR::Oculus::Utils => "Unity.XR.Oculus"
    ."Utils"
);
#[cfg(feature = "Unity+XR+Oculus+Utils")]
impl std::ops::Deref for crate::Unity::XR::Oculus::Utils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Utils")]
impl std::ops::DerefMut for crate::Unity::XR::Oculus::Utils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Utils")]
impl crate::Unity::XR::Oculus::Utils {}
#[cfg(feature = "Unity+XR+Oculus+Utils")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::XR::Oculus::Utils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
