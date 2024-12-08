#[cfg(feature = "System+Xml+Serialization+XmlAttributeAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlAttributeAttribute {
    __cordl_parent: crate::System::Attribute,
    pub attributeName: *mut crate::System::String,
    pub dataType: *mut crate::System::String,
    pub _cordl_type: *mut crate::System::Type,
    pub form: crate::System::Xml::Schema::XmlSchemaForm,
    pub ns: *mut crate::System::String,
}
#[cfg(feature = "System+Xml+Serialization+XmlAttributeAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlAttributeAttribute => "System.Xml.Serialization"
    ."XmlAttributeAttribute"
);
#[cfg(feature = "System+Xml+Serialization+XmlAttributeAttribute")]
impl std::ops::Deref for crate::System::Xml::Serialization::XmlAttributeAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlAttributeAttribute")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::XmlAttributeAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlAttributeAttribute")]
impl crate::System::Xml::Serialization::XmlAttributeAttribute {
    pub fn get_Form(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::XmlSchemaForm> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::XmlSchemaForm = __cordl_object
            .invoke("get_Form", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddKeyHash(
        &mut self,
        sb: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddKeyHash", (sb))?;
        Ok(__cordl_ret)
    }
    pub fn get_AttributeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_AttributeName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Namespace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Namespace", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_DataType(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DataType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        attributeName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (attributeName))?;
        Ok(__cordl_ret)
    }
    pub fn get_DataType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_DataType", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        attributeName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (attributeName))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlAttributeAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlAttributeAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
