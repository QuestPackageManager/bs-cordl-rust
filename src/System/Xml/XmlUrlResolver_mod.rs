#[cfg(feature = "System+Xml+XmlUrlResolver")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlUrlResolver {
    __cordl_parent: crate::System::Xml::XmlResolver,
    pub _credentials: quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
    pub _proxy: quest_hook::libil2cpp::Gc<crate::System::Net::IWebProxy>,
    pub _cachePolicy: quest_hook::libil2cpp::Gc<
        crate::System::Net::Cache::RequestCachePolicy,
    >,
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
    pub fn GetEntity(
        &mut self,
        absoluteUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        role: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ofObjectToReturn: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetEntity", (absoluteUri, role, ofObjectToReturn))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEntityAsync(
        &mut self,
        absoluteUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        role: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ofObjectToReturn: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = __cordl_object
            .invoke("GetEntityAsync", (absoluteUri, role, ofObjectToReturn))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ResolveUri(
        &mut self,
        baseUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        relativeUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Uri>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Uri> = __cordl_object
            .invoke("ResolveUri", (baseUri, relativeUri))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DownloadManager() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDownloadManager>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::XmlDownloadManager,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_DownloadManager", ())?;
        Ok(__cordl_ret.into())
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
