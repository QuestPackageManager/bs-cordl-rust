#[cfg(feature = "System+Xml+XmlParserContext")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlParserContext {
    __cordl_parent: crate::System::Object,
    pub _nt: *mut crate::System::Xml::XmlNameTable,
    pub _nsMgr: *mut crate::System::Xml::XmlNamespaceManager,
    pub _docTypeName: *mut crate::System::String,
    pub _pubId: *mut crate::System::String,
    pub _sysId: *mut crate::System::String,
    pub _internalSubset: *mut crate::System::String,
    pub _xmlLang: *mut crate::System::String,
    pub _xmlSpace: crate::System::Xml::XmlSpace,
    pub _baseURI: *mut crate::System::String,
    pub _encoding: *mut crate::System::Text::Encoding,
}
#[cfg(feature = "System+Xml+XmlParserContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlParserContext => "System.Xml"
    ."XmlParserContext"
);
#[cfg(feature = "System+Xml+XmlParserContext")]
impl std::ops::Deref for crate::System::Xml::XmlParserContext {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlParserContext")]
impl std::ops::DerefMut for crate::System::Xml::XmlParserContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlParserContext")]
impl crate::System::Xml::XmlParserContext {
    pub fn New_String_String_String_String_String_XmlSpace1(
        nt: *mut crate::System::Xml::XmlNameTable,
        nsMgr: *mut crate::System::Xml::XmlNamespaceManager,
        docTypeName: *mut crate::System::String,
        pubId: *mut crate::System::String,
        sysId: *mut crate::System::String,
        internalSubset: *mut crate::System::String,
        baseURI: *mut crate::System::String,
        xmlLang: *mut crate::System::String,
        xmlSpace: crate::System::Xml::XmlSpace,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    nt,
                    nsMgr,
                    docTypeName,
                    pubId,
                    sysId,
                    internalSubset,
                    baseURI,
                    xmlLang,
                    xmlSpace,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_String_String_String_String_String_XmlSpace_Encoding2(
        nt: *mut crate::System::Xml::XmlNameTable,
        nsMgr: *mut crate::System::Xml::XmlNamespaceManager,
        docTypeName: *mut crate::System::String,
        pubId: *mut crate::System::String,
        sysId: *mut crate::System::String,
        internalSubset: *mut crate::System::String,
        baseURI: *mut crate::System::String,
        xmlLang: *mut crate::System::String,
        xmlSpace: crate::System::Xml::XmlSpace,
        enc: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    nt,
                    nsMgr,
                    docTypeName,
                    pubId,
                    sysId,
                    internalSubset,
                    baseURI,
                    xmlLang,
                    xmlSpace,
                    enc,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_XmlSpace0(
        nt: *mut crate::System::Xml::XmlNameTable,
        nsMgr: *mut crate::System::Xml::XmlNamespaceManager,
        xmlLang: *mut crate::System::String,
        xmlSpace: crate::System::Xml::XmlSpace,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nt, nsMgr, xmlLang, xmlSpace))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_String_String_String_String_String_XmlSpace1(
        &mut self,
        nt: *mut crate::System::Xml::XmlNameTable,
        nsMgr: *mut crate::System::Xml::XmlNamespaceManager,
        docTypeName: *mut crate::System::String,
        pubId: *mut crate::System::String,
        sysId: *mut crate::System::String,
        internalSubset: *mut crate::System::String,
        baseURI: *mut crate::System::String,
        xmlLang: *mut crate::System::String,
        xmlSpace: crate::System::Xml::XmlSpace,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    nt,
                    nsMgr,
                    docTypeName,
                    pubId,
                    sysId,
                    internalSubset,
                    baseURI,
                    xmlLang,
                    xmlSpace,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_String_String_String_String_XmlSpace_Encoding2(
        &mut self,
        nt: *mut crate::System::Xml::XmlNameTable,
        nsMgr: *mut crate::System::Xml::XmlNamespaceManager,
        docTypeName: *mut crate::System::String,
        pubId: *mut crate::System::String,
        sysId: *mut crate::System::String,
        internalSubset: *mut crate::System::String,
        baseURI: *mut crate::System::String,
        xmlLang: *mut crate::System::String,
        xmlSpace: crate::System::Xml::XmlSpace,
        enc: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    nt,
                    nsMgr,
                    docTypeName,
                    pubId,
                    sysId,
                    internalSubset,
                    baseURI,
                    xmlLang,
                    xmlSpace,
                    enc,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_XmlSpace0(
        &mut self,
        nt: *mut crate::System::Xml::XmlNameTable,
        nsMgr: *mut crate::System::Xml::XmlNamespaceManager,
        xmlLang: *mut crate::System::String,
        xmlSpace: crate::System::Xml::XmlSpace,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nt, nsMgr, xmlLang, xmlSpace))?;
        Ok(__cordl_ret)
    }
    pub fn get_BaseURI(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_BaseURI", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DocTypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_DocTypeName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Encoding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::Encoding> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::Encoding = __cordl_object
            .invoke("get_Encoding", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasDtdInfo(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasDtdInfo", ())?;
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
    pub fn get_NameTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNameTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNameTable = __cordl_object
            .invoke("get_NameTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NamespaceManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNamespaceManager> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNamespaceManager = __cordl_object
            .invoke("get_NamespaceManager", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PublicId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_PublicId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SystemId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_SystemId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_XmlLang(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_XmlLang", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_XmlSpace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlSpace> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlSpace = __cordl_object
            .invoke("get_XmlSpace", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+XmlParserContext")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::XmlParserContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
