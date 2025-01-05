#[cfg(feature = "System+Runtime+Serialization+IgnoreDataMemberAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct IgnoreDataMemberAttribute {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::Attribute>,
}
#[cfg(feature = "System+Runtime+Serialization+IgnoreDataMemberAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::IgnoreDataMemberAttribute =>
    "System.Runtime.Serialization"."IgnoreDataMemberAttribute"
);
#[cfg(feature = "System+Runtime+Serialization+IgnoreDataMemberAttribute")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::IgnoreDataMemberAttribute {
    type Target = quest_hook::libil2cpp::Gc<crate::System::Attribute>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+IgnoreDataMemberAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::IgnoreDataMemberAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+IgnoreDataMemberAttribute")]
impl crate::System::Runtime::Serialization::IgnoreDataMemberAttribute {}
#[cfg(feature = "System+Runtime+Serialization+IgnoreDataMemberAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::IgnoreDataMemberAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
