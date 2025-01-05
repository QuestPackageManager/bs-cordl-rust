#[cfg(feature = "System+Xml+XmlParserContext")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlParserContext {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _nt: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
    pub _nsMgr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
    pub _docTypeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _pubId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _sysId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _internalSubset: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _xmlLang: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _xmlSpace: crate::System::Xml::XmlSpace,
    pub _baseURI: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
}
#[cfg(feature = "System+Xml+XmlParserContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlParserContext => "System.Xml"
    ."XmlParserContext"
);
#[cfg(feature = "System+Xml+XmlParserContext")]
impl std::ops::Deref for crate::System::Xml::XmlParserContext {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
    pub fn New_Gc_Gc_Gc_Gc_Gc_XmlSpace1(
        nt: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
        nsMgr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
        docTypeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pubId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sysId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        internalSubset: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        baseURI: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        xmlLang: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        xmlSpace: crate::System::Xml::XmlSpace,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
    }
    pub fn New_Gc_Gc_Gc_Gc_Gc_XmlSpace_Gc2(
        nt: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
        nsMgr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
        docTypeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pubId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sysId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        internalSubset: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        baseURI: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        xmlLang: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        xmlSpace: crate::System::Xml::XmlSpace,
        enc: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
    }
    pub fn New_XmlSpace0(
        nt: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
        nsMgr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
        xmlLang: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        xmlSpace: crate::System::Xml::XmlSpace,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nt, nsMgr, xmlLang, xmlSpace))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Gc_Gc_Gc_Gc_Gc_XmlSpace1(
        &mut self,
        nt: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
        nsMgr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
        docTypeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pubId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sysId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        internalSubset: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        baseURI: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        xmlLang: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Gc_Gc_Gc_Gc_Gc_XmlSpace_Gc2(
        &mut self,
        nt: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
        nsMgr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
        docTypeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pubId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sysId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        internalSubset: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        baseURI: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        xmlLang: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        xmlSpace: crate::System::Xml::XmlSpace,
        enc: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_XmlSpace0(
        &mut self,
        nt: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
        nsMgr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
        xmlLang: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        xmlSpace: crate::System::Xml::XmlSpace,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nt, nsMgr, xmlLang, xmlSpace))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BaseURI(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_BaseURI", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DocTypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_DocTypeName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Encoding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding> = __cordl_object
            .invoke("get_Encoding", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasDtdInfo(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasDtdInfo", ())?;
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
    pub fn get_NameTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable> = __cordl_object
            .invoke("get_NameTable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NamespaceManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::XmlNamespaceManager,
        > = __cordl_object.invoke("get_NamespaceManager", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PublicId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_PublicId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SystemId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_SystemId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_XmlLang(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_XmlLang", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_XmlSpace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlSpace> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlSpace = __cordl_object
            .invoke("get_XmlSpace", ())?;
        Ok(__cordl_ret.into())
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
