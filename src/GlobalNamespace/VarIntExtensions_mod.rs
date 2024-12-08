#[cfg(feature = "VarIntExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct VarIntExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "VarIntExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for VarIntExtensions => ""."VarIntExtensions"
);
#[cfg(feature = "VarIntExtensions")]
impl std::ops::Deref for VarIntExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VarIntExtensions")]
impl std::ops::DerefMut for VarIntExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VarIntExtensions")]
impl VarIntExtensions {}
#[cfg(feature = "VarIntExtensions")]
impl quest_hook::libil2cpp::ObjectType for VarIntExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}