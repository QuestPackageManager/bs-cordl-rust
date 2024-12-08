#[cfg(feature = "InteropErrorExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct InteropErrorExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "InteropErrorExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::InteropErrorExtensions => ""
    ."InteropErrorExtensions"
);
#[cfg(feature = "InteropErrorExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::InteropErrorExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "InteropErrorExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::InteropErrorExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "InteropErrorExtensions")]
impl crate::GlobalNamespace::InteropErrorExtensions {}
#[cfg(feature = "InteropErrorExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::InteropErrorExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
