#[cfg(feature = "System+Data+XmlIgnoreNamespaceReader")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlIgnoreNamespaceReader {
    __cordl_parent: crate::System::Xml::XmlNodeReader,
    pub _namespacesToIgnore: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
}
#[cfg(feature = "System+Data+XmlIgnoreNamespaceReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::XmlIgnoreNamespaceReader =>
    "System.Data"."XmlIgnoreNamespaceReader"
);
#[cfg(feature = "System+Data+XmlIgnoreNamespaceReader")]
impl std::ops::Deref for crate::System::Data::XmlIgnoreNamespaceReader {
    type Target = crate::System::Xml::XmlNodeReader;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+XmlIgnoreNamespaceReader")]
impl std::ops::DerefMut for crate::System::Data::XmlIgnoreNamespaceReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+XmlIgnoreNamespaceReader")]
impl crate::System::Data::XmlIgnoreNamespaceReader {
    pub fn MoveToFirstAttribute(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveToFirstAttribute", ())?;
        Ok(__cordl_ret)
    }
    pub fn MoveToNextAttribute(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveToNextAttribute", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        xdoc: *mut crate::System::Xml::XmlDocument,
        namespacesToIgnore: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xdoc, namespacesToIgnore))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        xdoc: *mut crate::System::Xml::XmlDocument,
        namespacesToIgnore: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (xdoc, namespacesToIgnore))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Data+XmlIgnoreNamespaceReader")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Data::XmlIgnoreNamespaceReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
