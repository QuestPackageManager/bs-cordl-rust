#[cfg(feature = "System+Xml+XmlParserContext")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlParserContext {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::XmlParserContext {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml";
    const CLASS_NAME: &'static str = "XmlParserContext";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Xml+XmlParserContext")]
impl std::ops::Deref for crate::System::Xml::XmlParserContext {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlParserContext")]
impl std::ops::DerefMut for crate::System::Xml::XmlParserContext {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlParserContext")]
impl crate::System::Xml::XmlParserContext {
    pub fn New_Il2CppString_Il2CppString_Il2CppString_Il2CppString_Il2CppString_XmlSpace1(
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
    pub fn New_Il2CppString_Il2CppString_Il2CppString_Il2CppString_Il2CppString_XmlSpace_Encoding2(
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
    pub fn _ctor_Il2CppString_Il2CppString_Il2CppString_Il2CppString_Il2CppString_XmlSpace1(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlNamespaceManager,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::System::Xml::XmlSpace,
                        ),
                        quest_hook::libil2cpp::Void,
                        9usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
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
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_Il2CppString_Il2CppString_Il2CppString_Il2CppString_XmlSpace_Encoding2(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlNamespaceManager,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::System::Xml::XmlSpace,
                            quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
                        ),
                        quest_hook::libil2cpp::Void,
                        10usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 10usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
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
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_XmlSpace0(
        &mut self,
        nt: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
        nsMgr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
        xmlLang: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        xmlSpace: crate::System::Xml::XmlSpace,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlNamespaceManager,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::System::Xml::XmlSpace,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (nt, nsMgr, xmlLang, xmlSpace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_BaseURI(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_BaseURI")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_BaseURI", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_DocTypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_DocTypeName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_DocTypeName", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Encoding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
                        0usize,
                    >("get_Encoding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_Encoding", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_HasDtdInfo(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), bool, 0usize>("get_HasDtdInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_HasDtdInfo", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_InternalSubset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_InternalSubset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_InternalSubset", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_NameTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
                        0usize,
                    >("get_NameTable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_NameTable", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_NamespaceManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Xml::XmlNamespaceManager,
                        >,
                        0usize,
                    >("get_NamespaceManager")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_NamespaceManager", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::XmlNamespaceManager,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_PublicId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_PublicId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_PublicId", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_SystemId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_SystemId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_SystemId", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_XmlLang(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_XmlLang")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_XmlLang", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_XmlSpace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlSpace> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::System::Xml::XmlSpace,
                        0usize,
                    >("get_XmlSpace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_XmlSpace", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Xml::XmlSpace = unsafe {
            method.invoke_unchecked(self, ())?
        };
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
