#[cfg(feature = "JetBrains+Annotations+BaseTypeRequiredAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseTypeRequiredAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _BaseType_k__BackingField: quest_hook::libil2cpp::Gc<crate::System::Type>,
}
#[cfg(feature = "JetBrains+Annotations+BaseTypeRequiredAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::JetBrains::Annotations::BaseTypeRequiredAttribute => "JetBrains.Annotations"
    ."BaseTypeRequiredAttribute"
);
#[cfg(feature = "JetBrains+Annotations+BaseTypeRequiredAttribute")]
impl std::ops::Deref for crate::JetBrains::Annotations::BaseTypeRequiredAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+BaseTypeRequiredAttribute")]
impl std::ops::DerefMut for crate::JetBrains::Annotations::BaseTypeRequiredAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+BaseTypeRequiredAttribute")]
impl crate::JetBrains::Annotations::BaseTypeRequiredAttribute {
    pub fn New(
        baseType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (baseType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        baseType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (baseType))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BaseType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_BaseType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_BaseType(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_BaseType", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "JetBrains+Annotations+BaseTypeRequiredAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::JetBrains::Annotations::BaseTypeRequiredAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
