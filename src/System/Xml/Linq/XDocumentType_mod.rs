#[cfg(feature = "System+Xml+Linq+XDocumentType")]
#[repr(C)]
#[derive(Debug)]
pub struct XDocumentType {
    __cordl_parent: crate::System::Xml::Linq::XNode,
    pub _name: *mut crate::System::String,
    pub _publicId: *mut crate::System::String,
    pub _systemId: *mut crate::System::String,
    pub _internalSubset: *mut crate::System::String,
}
#[cfg(feature = "System+Xml+Linq+XDocumentType")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Linq::XDocumentType =>
    "System.Xml.Linq"."XDocumentType"
);
#[cfg(feature = "System+Xml+Linq+XDocumentType")]
impl std::ops::Deref for crate::System::Xml::Linq::XDocumentType {
    type Target = crate::System::Xml::Linq::XNode;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+XDocumentType")]
impl std::ops::DerefMut for crate::System::Xml::Linq::XDocumentType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+XDocumentType")]
impl crate::System::Xml::Linq::XDocumentType {
    pub fn _ctor_String_String_String_String0(
        &mut self,
        name: *mut crate::System::String,
        publicId: *mut crate::System::String,
        systemId: *mut crate::System::String,
        internalSubset: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, publicId, systemId, internalSubset))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_XDocumentType1(
        &mut self,
        other: *mut crate::System::Xml::Linq::XDocumentType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (other))?;
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
    pub fn get_NodeType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlNodeType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlNodeType = __cordl_object
            .invoke("get_NodeType", ())?;
        Ok(__cordl_ret)
    }
    pub fn CloneNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Linq::XNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Linq::XNode = __cordl_object
            .invoke("CloneNode", ())?;
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
    pub fn WriteTo(
        &mut self,
        writer: *mut crate::System::Xml::XmlWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteTo", (writer))?;
        Ok(__cordl_ret)
    }
    pub fn New_String_String_String_String0(
        name: *mut crate::System::String,
        publicId: *mut crate::System::String,
        systemId: *mut crate::System::String,
        internalSubset: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, publicId, systemId, internalSubset))?;
        Ok(__cordl_object)
    }
    pub fn New_XDocumentType1(
        other: *mut crate::System::Xml::Linq::XDocumentType,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (other))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Linq+XDocumentType")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Linq::XDocumentType {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
