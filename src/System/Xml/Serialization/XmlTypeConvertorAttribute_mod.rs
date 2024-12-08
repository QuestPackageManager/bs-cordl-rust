#[cfg(feature = "System+Xml+Serialization+XmlTypeConvertorAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlTypeConvertorAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _Method_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "System+Xml+Serialization+XmlTypeConvertorAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlTypeConvertorAttribute => "System.Xml.Serialization"
    ."XmlTypeConvertorAttribute"
);
#[cfg(feature = "System+Xml+Serialization+XmlTypeConvertorAttribute")]
impl std::ops::Deref for crate::System::Xml::Serialization::XmlTypeConvertorAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlTypeConvertorAttribute")]
impl std::ops::DerefMut
for crate::System::Xml::Serialization::XmlTypeConvertorAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlTypeConvertorAttribute")]
impl crate::System::Xml::Serialization::XmlTypeConvertorAttribute {
    pub fn get_Method(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Method", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        method: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (method))?;
        Ok(__cordl_ret)
    }
    pub fn set_Method(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Method", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        method: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (method))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlTypeConvertorAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlTypeConvertorAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
