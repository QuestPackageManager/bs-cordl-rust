#[cfg(feature = "XRSubsystemHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct XRSubsystemHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "XRSubsystemHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::XRSubsystemHelper => ""
    ."XRSubsystemHelper"
);
#[cfg(feature = "XRSubsystemHelper")]
impl std::ops::Deref for crate::GlobalNamespace::XRSubsystemHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "XRSubsystemHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::XRSubsystemHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "XRSubsystemHelper")]
impl crate::GlobalNamespace::XRSubsystemHelper {}
#[cfg(feature = "XRSubsystemHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::XRSubsystemHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
