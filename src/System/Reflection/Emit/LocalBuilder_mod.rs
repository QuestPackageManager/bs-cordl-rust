#[cfg(feature = "cordl_class_System+Reflection+Emit+LocalBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalBuilder {
    __cordl_parent: crate::System::Reflection::LocalVariableInfo,
    pub name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub ilgen: quest_hook::libil2cpp::Gc<crate::System::Reflection::Emit::ILGenerator>,
    pub startOffset: i32,
    pub endOffset: i32,
}
#[cfg(feature = "cordl_class_System+Reflection+Emit+LocalBuilder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Reflection::Emit::LocalBuilder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Reflection.Emit";
    const CLASS_NAME: &'static str = "LocalBuilder";
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
#[cfg(feature = "cordl_class_System+Reflection+Emit+LocalBuilder")]
impl std::ops::Deref for crate::System::Reflection::Emit::LocalBuilder {
    type Target = crate::System::Reflection::LocalVariableInfo;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_System+Reflection+Emit+LocalBuilder")]
impl std::ops::DerefMut for crate::System::Reflection::Emit::LocalBuilder {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+Emit+LocalBuilder")]
impl crate::System::Reflection::Emit::LocalBuilder {}
#[cfg(feature = "cordl_class_System+Reflection+Emit+LocalBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::Emit::LocalBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
