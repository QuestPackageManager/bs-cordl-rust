#[cfg(feature = "System+Xml+IDtdParser")]
#[repr(C)]
#[derive(Debug)]
pub struct IDtdParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Xml+IDtdParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::IDtdParser => "System.Xml"
    ."IDtdParser"
);
#[cfg(feature = "System+Xml+IDtdParser")]
impl std::ops::Deref for crate::System::Xml::IDtdParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+IDtdParser")]
impl std::ops::DerefMut for crate::System::Xml::IDtdParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+IDtdParser")]
impl crate::System::Xml::IDtdParser {
    pub fn ParseFreeFloatingDtd(
        &mut self,
        baseUri: *mut quest_hook::libil2cpp::Il2CppString,
        docTypeName: *mut quest_hook::libil2cpp::Il2CppString,
        publicId: *mut quest_hook::libil2cpp::Il2CppString,
        systemId: *mut quest_hook::libil2cpp::Il2CppString,
        internalSubset: *mut quest_hook::libil2cpp::Il2CppString,
        adapter: *mut crate::System::Xml::IDtdParserAdapter,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::IDtdInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::IDtdInfo = __cordl_object
            .invoke(
                "ParseFreeFloatingDtd",
                (baseUri, docTypeName, publicId, systemId, internalSubset, adapter),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ParseInternalDtd(
        &mut self,
        adapter: *mut crate::System::Xml::IDtdParserAdapter,
        saveInternalSubset: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::IDtdInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::IDtdInfo = __cordl_object
            .invoke("ParseInternalDtd", (adapter, saveInternalSubset))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Xml+IDtdParser")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::IDtdParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
