#[cfg(feature = "System+Xml+IXmlNamespaceResolver")]
#[repr(C)]
#[derive(Debug)]
pub struct IXmlNamespaceResolver {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Xml+IXmlNamespaceResolver")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::IXmlNamespaceResolver =>
    "System.Xml"."IXmlNamespaceResolver"
);
#[cfg(feature = "System+Xml+IXmlNamespaceResolver")]
impl std::ops::Deref for crate::System::Xml::IXmlNamespaceResolver {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+IXmlNamespaceResolver")]
impl std::ops::DerefMut for crate::System::Xml::IXmlNamespaceResolver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+IXmlNamespaceResolver")]
impl crate::System::Xml::IXmlNamespaceResolver {
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
        namespaceName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("LookupPrefix", (namespaceName))?;
        Ok(__cordl_ret)
    }
    pub fn GetNamespacesInScope(
        &mut self,
        scope: crate::System::Xml::XmlNamespaceScope,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IDictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IDictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        > = __cordl_object.invoke("GetNamespacesInScope", (scope))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Xml+IXmlNamespaceResolver")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::IXmlNamespaceResolver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
