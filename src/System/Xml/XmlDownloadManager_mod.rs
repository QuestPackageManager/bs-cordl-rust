#[cfg(feature = "System+Xml+XmlDownloadManager")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlDownloadManager {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub connections: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
}
#[cfg(feature = "System+Xml+XmlDownloadManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlDownloadManager => "System.Xml"
    ."XmlDownloadManager"
);
#[cfg(feature = "System+Xml+XmlDownloadManager")]
impl std::ops::Deref for crate::System::Xml::XmlDownloadManager {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlDownloadManager")]
impl std::ops::DerefMut for crate::System::Xml::XmlDownloadManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlDownloadManager")]
impl crate::System::Xml::XmlDownloadManager {
    pub fn GetNonFileStream(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        credentials: quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
        proxy: quest_hook::libil2cpp::Gc<crate::System::Net::IWebProxy>,
        cachePolicy: quest_hook::libil2cpp::Gc<
            crate::System::Net::Cache::RequestCachePolicy,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = __cordl_object
            .invoke("GetNonFileStream", (uri, credentials, proxy, cachePolicy))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNonFileStreamAsync(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        credentials: quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
        proxy: quest_hook::libil2cpp::Gc<crate::System::Net::IWebProxy>,
        cachePolicy: quest_hook::libil2cpp::Gc<
            crate::System::Net::Cache::RequestCachePolicy,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Gc<crate::System::IO::Stream>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        > = __cordl_object
            .invoke("GetNonFileStreamAsync", (uri, credentials, proxy, cachePolicy))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStream(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        credentials: quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
        proxy: quest_hook::libil2cpp::Gc<crate::System::Net::IWebProxy>,
        cachePolicy: quest_hook::libil2cpp::Gc<
            crate::System::Net::Cache::RequestCachePolicy,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = __cordl_object
            .invoke("GetStream", (uri, credentials, proxy, cachePolicy))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStreamAsync(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        credentials: quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
        proxy: quest_hook::libil2cpp::Gc<crate::System::Net::IWebProxy>,
        cachePolicy: quest_hook::libil2cpp::Gc<
            crate::System::Net::Cache::RequestCachePolicy,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Gc<crate::System::IO::Stream>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        > = __cordl_object
            .invoke("GetStreamAsync", (uri, credentials, proxy, cachePolicy))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Remove(
        &mut self,
        host: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (host))?;
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
}
#[cfg(feature = "System+Xml+XmlDownloadManager")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::XmlDownloadManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
