#[cfg(feature = "OVRCustomFaceExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRCustomFaceExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRCustomFaceExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRCustomFaceExtensions => ""
    ."OVRCustomFaceExtensions"
);
#[cfg(feature = "OVRCustomFaceExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::OVRCustomFaceExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRCustomFaceExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRCustomFaceExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRCustomFaceExtensions")]
impl crate::GlobalNamespace::OVRCustomFaceExtensions {}
#[cfg(feature = "OVRCustomFaceExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRCustomFaceExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
