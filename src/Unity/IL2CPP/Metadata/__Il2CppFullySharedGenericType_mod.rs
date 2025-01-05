#[cfg(feature = "Unity+IL2CPP+Metadata+__Il2CppFullySharedGenericType")]
#[repr(C)]
#[derive(Debug)]
pub struct __Il2CppFullySharedGenericType {
    __cordl_parent: crate::System::ValueType,
}
#[cfg(feature = "Unity+IL2CPP+Metadata+__Il2CppFullySharedGenericType")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::IL2CPP::Metadata::__Il2CppFullySharedGenericType => "Unity.IL2CPP.Metadata"
    ."__Il2CppFullySharedGenericType"
);
#[cfg(feature = "Unity+IL2CPP+Metadata+__Il2CppFullySharedGenericType")]
impl std::ops::Deref for crate::Unity::IL2CPP::Metadata::__Il2CppFullySharedGenericType {
    type Target = crate::System::ValueType;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+IL2CPP+Metadata+__Il2CppFullySharedGenericType")]
impl std::ops::DerefMut
for crate::Unity::IL2CPP::Metadata::__Il2CppFullySharedGenericType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+IL2CPP+Metadata+__Il2CppFullySharedGenericType")]
impl crate::Unity::IL2CPP::Metadata::__Il2CppFullySharedGenericType {}
#[cfg(feature = "Unity+IL2CPP+Metadata+__Il2CppFullySharedGenericType")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::IL2CPP::Metadata::__Il2CppFullySharedGenericType {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
