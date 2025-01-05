#[cfg(feature = "System+MonoType")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoType {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
}
#[cfg(feature = "System+MonoType")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::MonoType => "System"."MonoType"
);
#[cfg(feature = "System+MonoType")]
impl std::ops::Deref for crate::System::MonoType {
    type Target = quest_hook::libil2cpp::Gc<crate::System::RuntimeType>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+MonoType")]
impl std::ops::DerefMut for crate::System::MonoType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+MonoType")]
impl crate::System::MonoType {}
#[cfg(feature = "System+MonoType")]
impl quest_hook::libil2cpp::ObjectType for crate::System::MonoType {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
