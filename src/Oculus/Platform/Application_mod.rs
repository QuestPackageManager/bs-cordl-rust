#[cfg(feature = "Oculus+Platform+Application")]
#[repr(C)]
#[derive(Debug)]
pub struct Application {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+Application")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Application =>
    "Oculus.Platform"."Application"
);
#[cfg(feature = "Oculus+Platform+Application")]
impl std::ops::Deref for crate::Oculus::Platform::Application {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Application")]
impl std::ops::DerefMut for crate::Oculus::Platform::Application {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Application")]
impl crate::Oculus::Platform::Application {
    pub fn CancelAppDownload() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::AppDownloadResult,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::AppDownloadResult,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CancelAppDownload", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckAppDownloadProgress() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::AppDownloadProgressResult,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::AppDownloadProgressResult,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckAppDownloadProgress", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVersion() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::ApplicationVersion,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::ApplicationVersion,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InstallAppUpdateAndRelaunch(
        deeplink_options: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::ApplicationOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::AppDownloadResult,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::AppDownloadResult,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InstallAppUpdateAndRelaunch", (deeplink_options))?;
        Ok(__cordl_ret.into())
    }
    pub fn LaunchOtherApp(
        appID: u64,
        deeplink_options: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::ApplicationOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<*mut quest_hook::libil2cpp::Il2CppString>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LaunchOtherApp", (appID, deeplink_options))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartAppDownload() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::AppDownloadResult,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::AppDownloadResult,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StartAppDownload", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+Application")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Application {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
