#[cfg(feature = "System+Reflection+Emit+AssemblyBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct AssemblyBuilder {
    __cordl_parent: crate::System::Reflection::Assembly,
}
#[cfg(feature = "System+Reflection+Emit+AssemblyBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::Emit::AssemblyBuilder =>
    "System.Reflection.Emit"."AssemblyBuilder"
);
#[cfg(feature = "System+Reflection+Emit+AssemblyBuilder")]
impl std::ops::Deref for crate::System::Reflection::Emit::AssemblyBuilder {
    type Target = crate::System::Reflection::Assembly;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+Emit+AssemblyBuilder")]
impl std::ops::DerefMut for crate::System::Reflection::Emit::AssemblyBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+Emit+AssemblyBuilder")]
impl crate::System::Reflection::Emit::AssemblyBuilder {}
#[cfg(feature = "System+Reflection+Emit+AssemblyBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::Emit::AssemblyBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
