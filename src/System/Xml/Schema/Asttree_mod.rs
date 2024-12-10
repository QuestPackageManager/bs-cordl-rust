#[cfg(feature = "System+Xml+Schema+Asttree")]
#[repr(C)]
#[derive(Debug)]
pub struct Asttree {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _fAxisArray: *mut crate::System::Collections::ArrayList,
    pub _xpathexpr: *mut quest_hook::libil2cpp::Il2CppString,
    pub _isField: bool,
    pub _nsmgr: *mut crate::System::Xml::XmlNamespaceManager,
}
#[cfg(feature = "System+Xml+Schema+Asttree")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::Asttree =>
    "System.Xml.Schema"."Asttree"
);
#[cfg(feature = "System+Xml+Schema+Asttree")]
impl std::ops::Deref for crate::System::Xml::Schema::Asttree {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Asttree")]
impl std::ops::DerefMut for crate::System::Xml::Schema::Asttree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Asttree")]
impl crate::System::Xml::Schema::Asttree {
    pub fn CompileXPath(
        &mut self,
        xPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isField: bool,
        nsmgr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileXPath", (xPath, isField, nsmgr))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        xPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isField: bool,
        nsmgr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xPath, isField, nsmgr))?;
        Ok(__cordl_object.into())
    }
    pub fn SetURN(
        &mut self,
        axis: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::Axis>,
        nsmgr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetURN", (axis, nsmgr))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        xPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isField: bool,
        nsmgr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (xPath, isField, nsmgr))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SubtreeArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ArrayList,
        > = __cordl_object.invoke("get_SubtreeArray", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+Asttree")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::Asttree {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
