#[cfg(feature = "cordl_class_System+Reflection+Emit+ModuleBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct ModuleBuilder {
    __cordl_parent: crate::System::Reflection::Module,
}
#[cfg(feature = "cordl_class_System+Reflection+Emit+ModuleBuilder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Reflection::Emit::ModuleBuilder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Reflection.Emit";
    const CLASS_NAME: &'static str = "ModuleBuilder";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Reflection+Emit+ModuleBuilder")]
impl std::ops::Deref for crate::System::Reflection::Emit::ModuleBuilder {
    type Target = crate::System::Reflection::Module;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+Emit+ModuleBuilder")]
impl std::ops::DerefMut for crate::System::Reflection::Emit::ModuleBuilder {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+Emit+ModuleBuilder")]
impl crate::System::Reflection::Emit::ModuleBuilder {}
#[cfg(feature = "cordl_class_System+Reflection+Emit+ModuleBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::Emit::ModuleBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
