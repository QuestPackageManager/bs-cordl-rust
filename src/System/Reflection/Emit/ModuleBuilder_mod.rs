#[cfg(feature = "System+Reflection+Emit+ModuleBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct ModuleBuilder {
    __cordl_parent: crate::System::Reflection::Module,
}
#[cfg(feature = "System+Reflection+Emit+ModuleBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::Emit::ModuleBuilder =>
    "System.Reflection.Emit"."ModuleBuilder"
);
#[cfg(feature = "System+Reflection+Emit+ModuleBuilder")]
impl std::ops::Deref for crate::System::Reflection::Emit::ModuleBuilder {
    type Target = crate::System::Reflection::Module;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+Emit+ModuleBuilder")]
impl std::ops::DerefMut for crate::System::Reflection::Emit::ModuleBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+Emit+ModuleBuilder")]
impl crate::System::Reflection::Emit::ModuleBuilder {}
#[cfg(feature = "System+Reflection+Emit+ModuleBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::Emit::ModuleBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
