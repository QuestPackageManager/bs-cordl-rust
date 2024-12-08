#[cfg(feature = "ConstructorStringExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct ConstructorStringExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "ConstructorStringExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ConstructorStringExtensions => ""
    ."ConstructorStringExtensions"
);
#[cfg(feature = "ConstructorStringExtensions")]
impl std::ops::Deref for ConstructorStringExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ConstructorStringExtensions")]
impl std::ops::DerefMut for ConstructorStringExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ConstructorStringExtensions")]
impl ConstructorStringExtensions {}
#[cfg(feature = "ConstructorStringExtensions")]
impl quest_hook::libil2cpp::ObjectType for ConstructorStringExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
