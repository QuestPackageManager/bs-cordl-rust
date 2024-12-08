#[cfg(feature = "System+Xml+Schema+SchemaNamespaceManager")]
#[repr(C)]
#[derive(Debug)]
pub struct SchemaNamespaceManager {
    __cordl_parent: crate::System::Xml::XmlNamespaceManager,
    pub node: *mut crate::System::Xml::Schema::XmlSchemaObject,
}
#[cfg(feature = "System+Xml+Schema+SchemaNamespaceManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::SchemaNamespaceManager =>
    "System.Xml.Schema"."SchemaNamespaceManager"
);
#[cfg(feature = "System+Xml+Schema+SchemaNamespaceManager")]
impl std::ops::Deref for crate::System::Xml::Schema::SchemaNamespaceManager {
    type Target = crate::System::Xml::XmlNamespaceManager;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+SchemaNamespaceManager")]
impl std::ops::DerefMut for crate::System::Xml::Schema::SchemaNamespaceManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+SchemaNamespaceManager")]
impl crate::System::Xml::Schema::SchemaNamespaceManager {
    pub fn LookupNamespace(
        &mut self,
        prefix: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("LookupNamespace", (prefix))?;
        Ok(__cordl_ret)
    }
    pub fn LookupPrefix(
        &mut self,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("LookupPrefix", (ns))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        node: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (node))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        node: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (node))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+SchemaNamespaceManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::SchemaNamespaceManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
