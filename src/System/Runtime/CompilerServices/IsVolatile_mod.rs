#[cfg(feature = "System+Runtime+CompilerServices+IsVolatile")]
#[repr(C)]
#[derive(Debug)]
pub struct IsVolatile {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Runtime+CompilerServices+IsVolatile")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::CompilerServices::IsVolatile =>
    "System.Runtime.CompilerServices"."IsVolatile"
);
#[cfg(feature = "System+Runtime+CompilerServices+IsVolatile")]
impl std::ops::Deref for crate::System::Runtime::CompilerServices::IsVolatile {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+IsVolatile")]
impl std::ops::DerefMut for crate::System::Runtime::CompilerServices::IsVolatile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+IsVolatile")]
impl crate::System::Runtime::CompilerServices::IsVolatile {}
#[cfg(feature = "System+Runtime+CompilerServices+IsVolatile")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::IsVolatile {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
