#[cfg(feature = "System+Linq+Expressions+SymbolDocumentInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct SymbolDocumentInfo {
    __cordl_parent: crate::System::Object,
    pub _FileName_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "System+Linq+Expressions+SymbolDocumentInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::SymbolDocumentInfo =>
    "System.Linq.Expressions"."SymbolDocumentInfo"
);
#[cfg(feature = "System+Linq+Expressions+SymbolDocumentInfo")]
impl std::ops::Deref for crate::System::Linq::Expressions::SymbolDocumentInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+SymbolDocumentInfo")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::SymbolDocumentInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+SymbolDocumentInfo")]
impl crate::System::Linq::Expressions::SymbolDocumentInfo {
    pub fn get_FileName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_FileName", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Linq+Expressions+SymbolDocumentInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::SymbolDocumentInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}