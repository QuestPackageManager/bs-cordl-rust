#[cfg(feature = "System+Reflection+Emit+LocalBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalBuilder {
    __cordl_parent: crate::System::Reflection::LocalVariableInfo,
    pub name: *mut crate::System::String,
    pub ilgen: *mut crate::System::Reflection::Emit::ILGenerator,
    pub startOffset: i32,
    pub endOffset: i32,
}
#[cfg(feature = "System+Reflection+Emit+LocalBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::Emit::LocalBuilder =>
    "System.Reflection.Emit"."LocalBuilder"
);
#[cfg(feature = "System+Reflection+Emit+LocalBuilder")]
impl std::ops::Deref for crate::System::Reflection::Emit::LocalBuilder {
    type Target = crate::System::Reflection::LocalVariableInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+Emit+LocalBuilder")]
impl std::ops::DerefMut for crate::System::Reflection::Emit::LocalBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+Emit+LocalBuilder")]
impl crate::System::Reflection::Emit::LocalBuilder {}
#[cfg(feature = "System+Reflection+Emit+LocalBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::Emit::LocalBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
