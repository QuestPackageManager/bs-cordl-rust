#[cfg(feature = "System+Xml+Serialization+XmlSchemaProviderAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSchemaProviderAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _methodName: *mut crate::System::String,
    pub _isAny: bool,
}
#[cfg(feature = "System+Xml+Serialization+XmlSchemaProviderAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlSchemaProviderAttribute =>
    "System.Xml.Serialization"."XmlSchemaProviderAttribute"
);
#[cfg(feature = "System+Xml+Serialization+XmlSchemaProviderAttribute")]
impl std::ops::Deref for crate::System::Xml::Serialization::XmlSchemaProviderAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSchemaProviderAttribute")]
impl std::ops::DerefMut
for crate::System::Xml::Serialization::XmlSchemaProviderAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSchemaProviderAttribute")]
impl crate::System::Xml::Serialization::XmlSchemaProviderAttribute {
    pub fn New(
        methodName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (methodName))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        methodName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (methodName))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsAny(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsAny", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MethodName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_MethodName", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_IsAny(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsAny", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSchemaProviderAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlSchemaProviderAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
