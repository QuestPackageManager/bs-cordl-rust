#[cfg(feature = "System+Xml+Schema+Asttree")]
#[repr(C)]
#[derive(Debug)]
pub struct Asttree {
    __cordl_parent: crate::System::Object,
    pub _fAxisArray: *mut crate::System::Collections::ArrayList,
    pub _xpathexpr: *mut crate::System::String,
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
    type Target = crate::System::Object;
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
    pub fn _ctor(
        &mut self,
        xPath: *mut crate::System::String,
        isField: bool,
        nsmgr: *mut crate::System::Xml::XmlNamespaceManager,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (xPath, isField, nsmgr))?;
        Ok(__cordl_ret)
    }
    pub fn SetURN(
        &mut self,
        axis: *mut crate::MS::Internal::Xml::XPath::Axis,
        nsmgr: *mut crate::System::Xml::XmlNamespaceManager,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetURN", (axis, nsmgr))?;
        Ok(__cordl_ret)
    }
    pub fn get_SubtreeArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ArrayList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ArrayList = __cordl_object
            .invoke("get_SubtreeArray", ())?;
        Ok(__cordl_ret)
    }
    pub fn CompileXPath(
        &mut self,
        xPath: *mut crate::System::String,
        isField: bool,
        nsmgr: *mut crate::System::Xml::XmlNamespaceManager,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileXPath", (xPath, isField, nsmgr))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        xPath: *mut crate::System::String,
        isField: bool,
        nsmgr: *mut crate::System::Xml::XmlNamespaceManager,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xPath, isField, nsmgr))?;
        Ok(__cordl_object)
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
