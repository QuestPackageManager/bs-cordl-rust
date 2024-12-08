#[cfg(feature = "System+Runtime+Serialization+OptionalFieldAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct OptionalFieldAttribute {
    __cordl_parent: crate::System::Attribute,
    pub versionAdded: i32,
}
#[cfg(feature = "System+Runtime+Serialization+OptionalFieldAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::OptionalFieldAttribute =>
    "System.Runtime.Serialization"."OptionalFieldAttribute"
);
#[cfg(feature = "System+Runtime+Serialization+OptionalFieldAttribute")]
impl std::ops::Deref for crate::System::Runtime::Serialization::OptionalFieldAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+OptionalFieldAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::OptionalFieldAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+OptionalFieldAttribute")]
impl crate::System::Runtime::Serialization::OptionalFieldAttribute {
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
    pub fn set_VersionAdded(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_VersionAdded", (value))?;
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
#[cfg(feature = "System+Runtime+Serialization+OptionalFieldAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::OptionalFieldAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
