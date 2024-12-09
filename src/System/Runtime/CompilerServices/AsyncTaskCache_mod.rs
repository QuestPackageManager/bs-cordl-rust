#[cfg(feature = "System+Runtime+CompilerServices+AsyncTaskCache")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncTaskCache {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+CompilerServices+AsyncTaskCache")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::AsyncTaskCache =>
    "System.Runtime.CompilerServices"."AsyncTaskCache"
);
#[cfg(feature = "System+Runtime+CompilerServices+AsyncTaskCache")]
impl std::ops::Deref for crate::System::Runtime::CompilerServices::AsyncTaskCache {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+AsyncTaskCache")]
impl std::ops::DerefMut for crate::System::Runtime::CompilerServices::AsyncTaskCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+AsyncTaskCache")]
impl crate::System::Runtime::CompilerServices::AsyncTaskCache {}
#[cfg(feature = "System+Runtime+CompilerServices+AsyncTaskCache")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::AsyncTaskCache {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
