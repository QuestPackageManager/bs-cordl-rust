#[cfg(feature = "cordl_class_Unity+IL2CPP+Metadata+__Il2CppFullySharedGenericType")]
#[repr(C)]
#[derive(Debug)]
pub struct __Il2CppFullySharedGenericType {
    __cordl_parent: crate::System::ValueType,
}
#[cfg(feature = "cordl_class_Unity+IL2CPP+Metadata+__Il2CppFullySharedGenericType")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Unity::IL2CPP::Metadata::__Il2CppFullySharedGenericType
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.IL2CPP.Metadata";
    const CLASS_NAME: &'static str = "__Il2CppFullySharedGenericType";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Unity+IL2CPP+Metadata+__Il2CppFullySharedGenericType")]
impl std::ops::Deref for crate::Unity::IL2CPP::Metadata::__Il2CppFullySharedGenericType {
    type Target = crate::System::ValueType;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+IL2CPP+Metadata+__Il2CppFullySharedGenericType")]
impl std::ops::DerefMut for crate::Unity::IL2CPP::Metadata::__Il2CppFullySharedGenericType {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+IL2CPP+Metadata+__Il2CppFullySharedGenericType")]
impl crate::Unity::IL2CPP::Metadata::__Il2CppFullySharedGenericType {}
#[cfg(feature = "cordl_class_Unity+IL2CPP+Metadata+__Il2CppFullySharedGenericType")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Unity::IL2CPP::Metadata::__Il2CppFullySharedGenericType
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
