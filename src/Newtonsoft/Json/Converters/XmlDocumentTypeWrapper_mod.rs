#[cfg(feature = "Newtonsoft+Json+Converters+XmlDocumentTypeWrapper")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlDocumentTypeWrapper {
    __cordl_parent: crate::Newtonsoft::Json::Converters::XmlNodeWrapper,
    pub _documentType: *mut crate::System::Xml::XmlDocumentType,
}
#[cfg(feature = "Newtonsoft+Json+Converters+XmlDocumentTypeWrapper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Converters::XmlDocumentTypeWrapper =>
    "Newtonsoft.Json.Converters"."XmlDocumentTypeWrapper"
);
#[cfg(feature = "Newtonsoft+Json+Converters+XmlDocumentTypeWrapper")]
impl std::ops::Deref for crate::Newtonsoft::Json::Converters::XmlDocumentTypeWrapper {
    type Target = crate::Newtonsoft::Json::Converters::XmlNodeWrapper;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XmlDocumentTypeWrapper")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Converters::XmlDocumentTypeWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XmlDocumentTypeWrapper")]
impl crate::Newtonsoft::Json::Converters::XmlDocumentTypeWrapper {
    pub fn get_Public(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Public", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LocalName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_LocalName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_System(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_System", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_InternalSubset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_InternalSubset", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        documentType: *mut crate::System::Xml::XmlDocumentType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (documentType))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        documentType: *mut crate::System::Xml::XmlDocumentType,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (documentType))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XmlDocumentTypeWrapper")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Converters::XmlDocumentTypeWrapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
