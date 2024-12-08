#[cfg(feature = "System+Runtime+Serialization+OnSerializingAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct OnSerializingAttribute {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(feature = "System+Runtime+Serialization+OnSerializingAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::OnSerializingAttribute =>
    "System.Runtime.Serialization"."OnSerializingAttribute"
);
#[cfg(feature = "System+Runtime+Serialization+OnSerializingAttribute")]
impl std::ops::Deref for crate::System::Runtime::Serialization::OnSerializingAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+OnSerializingAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::OnSerializingAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+OnSerializingAttribute")]
impl crate::System::Runtime::Serialization::OnSerializingAttribute {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+Serialization+OnSerializingAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::OnSerializingAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
