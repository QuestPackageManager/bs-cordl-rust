#[cfg(feature = "Newtonsoft+Json+Converters+XDocumentTypeWrapper")]
#[repr(C)]
#[derive(Debug)]
pub struct XDocumentTypeWrapper {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Converters::XObjectWrapper,
    >,
    pub _documentType: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Linq::XDocumentType,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Converters+XDocumentTypeWrapper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Converters::XDocumentTypeWrapper => "Newtonsoft.Json.Converters"
    ."XDocumentTypeWrapper"
);
#[cfg(feature = "Newtonsoft+Json+Converters+XDocumentTypeWrapper")]
impl std::ops::Deref for crate::Newtonsoft::Json::Converters::XDocumentTypeWrapper {
    type Target = quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Converters::XObjectWrapper,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XDocumentTypeWrapper")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Converters::XDocumentTypeWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XDocumentTypeWrapper")]
impl crate::Newtonsoft::Json::Converters::XDocumentTypeWrapper {
    pub fn New(
        documentType: quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XDocumentType>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (documentType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        documentType: quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XDocumentType>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (documentType))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InternalSubset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_InternalSubset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LocalName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_LocalName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Public(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Public", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_System(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_System", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XDocumentTypeWrapper")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Converters::XDocumentTypeWrapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XDocumentTypeWrapper")]
impl AsRef<
    quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlDocumentType>,
> for crate::Newtonsoft::Json::Converters::XDocumentTypeWrapper {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Converters::IXmlDocumentType,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XDocumentTypeWrapper")]
impl AsMut<
    quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlDocumentType>,
> for crate::Newtonsoft::Json::Converters::XDocumentTypeWrapper {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Converters::IXmlDocumentType,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XDocumentTypeWrapper")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode>>
for crate::Newtonsoft::Json::Converters::XDocumentTypeWrapper {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XDocumentTypeWrapper")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode>>
for crate::Newtonsoft::Json::Converters::XDocumentTypeWrapper {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode> {
        unsafe { std::mem::transmute(self) }
    }
}
