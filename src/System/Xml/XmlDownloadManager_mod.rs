#[cfg(feature = "System+Xml+XmlDownloadManager")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlDownloadManager {
    __cordl_parent: crate::System::Object,
    pub connections: *mut crate::System::Collections::Hashtable,
}
#[cfg(feature = "System+Xml+XmlDownloadManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlDownloadManager => "System.Xml"
    ."XmlDownloadManager"
);
#[cfg(feature = "System+Xml+XmlDownloadManager")]
impl std::ops::Deref for crate::System::Xml::XmlDownloadManager {
    type Target = crate::System::Object;
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
    #[cfg(feature = "System+Xml+XmlDownloadManager+_GetNonFileStreamAsync_d__5")]
    pub type _GetNonFileStreamAsync_d__5 = crate::System::Xml::XmlDownloadManager__GetNonFileStreamAsync_d__5;
    #[cfg(feature = "System+Xml+XmlDownloadManager+__c__DisplayClass4_0")]
    pub type __c__DisplayClass4_0 = crate::System::Xml::XmlDownloadManager___c__DisplayClass4_0;
    pub fn GetStreamAsync(
        &mut self,
        uri: *mut crate::System::Uri,
        credentials: *mut crate::System::Net::ICredentials,
        proxy: *mut crate::System::Net::IWebProxy,
        cachePolicy: *mut crate::System::Net::Cache::RequestCachePolicy,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::System::IO::Stream>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::System::IO::Stream,
        > = __cordl_object
            .invoke("GetStreamAsync", (uri, credentials, proxy, cachePolicy))?;
        Ok(__cordl_ret)
    }
    pub fn GetNonFileStreamAsync(
        &mut self,
        uri: *mut crate::System::Uri,
        credentials: *mut crate::System::Net::ICredentials,
        proxy: *mut crate::System::Net::IWebProxy,
        cachePolicy: *mut crate::System::Net::Cache::RequestCachePolicy,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::System::IO::Stream>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::System::IO::Stream,
        > = __cordl_object
            .invoke("GetNonFileStreamAsync", (uri, credentials, proxy, cachePolicy))?;
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
    pub fn GetNonFileStream(
        &mut self,
        uri: *mut crate::System::Uri,
        credentials: *mut crate::System::Net::ICredentials,
        proxy: *mut crate::System::Net::IWebProxy,
        cachePolicy: *mut crate::System::Net::Cache::RequestCachePolicy,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("GetNonFileStream", (uri, credentials, proxy, cachePolicy))?;
        Ok(__cordl_ret)
    }
    pub fn Remove(
        &mut self,
        host: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (host))?;
        Ok(__cordl_ret)
    }
    pub fn GetStream(
        &mut self,
        uri: *mut crate::System::Uri,
        credentials: *mut crate::System::Net::ICredentials,
        proxy: *mut crate::System::Net::IWebProxy,
        cachePolicy: *mut crate::System::Net::Cache::RequestCachePolicy,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("GetStream", (uri, credentials, proxy, cachePolicy))?;
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
#[cfg(feature = "System+Xml+XmlDownloadManager")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::XmlDownloadManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
