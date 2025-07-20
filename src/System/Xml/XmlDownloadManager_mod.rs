#[cfg(feature = "System+Xml+XmlDownloadManager")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlDownloadManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub connections: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
}
#[cfg(feature = "System+Xml+XmlDownloadManager")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::XmlDownloadManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml";
    const CLASS_NAME: &'static str = "XmlDownloadManager";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Xml+XmlDownloadManager")]
impl std::ops::Deref for crate::System::Xml::XmlDownloadManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Uri>,
                            quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
                            quest_hook::libil2cpp::Gc<crate::System::Net::IWebProxy>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Net::Cache::RequestCachePolicy,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                        4usize,
                    >("GetNonFileStream")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetNonFileStream", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = unsafe {
            method.invoke_unchecked(self, (uri, credentials, proxy, cachePolicy))?
        };
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
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Uri>,
                            quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
                            quest_hook::libil2cpp::Gc<crate::System::Net::IWebProxy>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Net::Cache::RequestCachePolicy,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<
                                quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                            >,
                        >,
                        4usize,
                    >("GetNonFileStreamAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetNonFileStreamAsync", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
            >,
        > = unsafe {
            method.invoke_unchecked(self, (uri, credentials, proxy, cachePolicy))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Uri>,
                            quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
                            quest_hook::libil2cpp::Gc<crate::System::Net::IWebProxy>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Net::Cache::RequestCachePolicy,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                        4usize,
                    >("GetStream")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetStream", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = unsafe {
            method.invoke_unchecked(self, (uri, credentials, proxy, cachePolicy))?
        };
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
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Uri>,
                            quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
                            quest_hook::libil2cpp::Gc<crate::System::Net::IWebProxy>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Net::Cache::RequestCachePolicy,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<
                                quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                            >,
                        >,
                        4usize,
                    >("GetStreamAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetStreamAsync", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
            >,
        > = unsafe {
            method.invoke_unchecked(self, (uri, credentials, proxy, cachePolicy))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Remove")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Remove", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (host))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
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
