#[cfg(feature = "System+Xml+XmlUrlResolver")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlUrlResolver {
    __cordl_parent: crate::System::Xml::XmlResolver,
    pub _credentials: *mut crate::System::Net::ICredentials,
    pub _proxy: *mut crate::System::Net::IWebProxy,
    pub _cachePolicy: *mut crate::System::Net::Cache::RequestCachePolicy,
}
#[cfg(feature = "System+Xml+XmlUrlResolver")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlUrlResolver => "System.Xml"
    ."XmlUrlResolver"
);
#[cfg(feature = "System+Xml+XmlUrlResolver")]
impl std::ops::Deref for crate::System::Xml::XmlUrlResolver {
    type Target = crate::System::Xml::XmlResolver;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlUrlResolver")]
impl std::ops::DerefMut for crate::System::Xml::XmlUrlResolver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlUrlResolver")]
impl crate::System::Xml::XmlUrlResolver {
    #[cfg(feature = "System+Xml+XmlUrlResolver+_GetEntityAsync_d__15")]
    pub type _GetEntityAsync_d__15 = crate::System::Xml::XmlUrlResolver__GetEntityAsync_d__15;
    pub fn GetEntity(
        &mut self,
        absoluteUri: *mut crate::System::Uri,
        role: *mut crate::System::String,
        ofObjectToReturn: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetEntity", (absoluteUri, role, ofObjectToReturn))?;
        Ok(__cordl_ret)
    }
    pub fn GetEntityAsync(
        &mut self,
        absoluteUri: *mut crate::System::Uri,
        role: *mut crate::System::String,
        ofObjectToReturn: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::System::Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::System::Object,
        > = __cordl_object
            .invoke("GetEntityAsync", (absoluteUri, role, ofObjectToReturn))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveUri(
        &mut self,
        baseUri: *mut crate::System::Uri,
        relativeUri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Uri> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Uri = __cordl_object
            .invoke("ResolveUri", (baseUri, relativeUri))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+XmlUrlResolver")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::XmlUrlResolver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
