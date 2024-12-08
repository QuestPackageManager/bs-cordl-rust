#[cfg(feature = "Vector3Extensions")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector3Extensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Vector3Extensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::Vector3Extensions => ""
    ."Vector3Extensions"
);
#[cfg(feature = "Vector3Extensions")]
impl std::ops::Deref for crate::GlobalNamespace::Vector3Extensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Vector3Extensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::Vector3Extensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Vector3Extensions")]
impl crate::GlobalNamespace::Vector3Extensions {}
#[cfg(feature = "Vector3Extensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::Vector3Extensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
