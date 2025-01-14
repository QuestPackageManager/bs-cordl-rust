#[cfg(feature = "Oculus+Platform+AssetFile")]
#[repr(C)]
#[derive(Debug)]
pub struct AssetFile {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+AssetFile")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Platform::AssetFile {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "AssetFile";
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
#[cfg(feature = "Oculus+Platform+AssetFile")]
impl std::ops::Deref for crate::Oculus::Platform::AssetFile {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+AssetFile")]
impl std::ops::DerefMut for crate::Oculus::Platform::AssetFile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+AssetFile")]
impl crate::Oculus::Platform::AssetFile {
    pub fn Delete(
        assetFileID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::AssetFileDeleteResult,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::AssetFileDeleteResult,
                        >,
                    >,
                >,
                1usize,
            >("Delete")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Delete", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::AssetFileDeleteResult,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), (assetFileID)) };
        Ok(__cordl_ret.into())
    }
    pub fn DeleteById(
        assetFileID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::AssetFileDeleteResult,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::AssetFileDeleteResult,
                        >,
                    >,
                >,
                1usize,
            >("DeleteById")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DeleteById", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::AssetFileDeleteResult,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), (assetFileID)) };
        Ok(__cordl_ret.into())
    }
    pub fn DeleteByName(
        assetFileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::AssetFileDeleteResult,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::AssetFileDeleteResult,
                        >,
                    >,
                >,
                1usize,
            >("DeleteByName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DeleteByName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::AssetFileDeleteResult,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), (assetFileName)) };
        Ok(__cordl_ret.into())
    }
    pub fn Download(
        assetFileID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::AssetFileDownloadResult,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::AssetFileDownloadResult,
                        >,
                    >,
                >,
                1usize,
            >("Download")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Download", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::AssetFileDownloadResult,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), (assetFileID)) };
        Ok(__cordl_ret.into())
    }
    pub fn DownloadById(
        assetFileID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::AssetFileDownloadResult,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::AssetFileDownloadResult,
                        >,
                    >,
                >,
                1usize,
            >("DownloadById")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DownloadById", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::AssetFileDownloadResult,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), (assetFileID)) };
        Ok(__cordl_ret.into())
    }
    pub fn DownloadByName(
        assetFileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::AssetFileDownloadResult,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::AssetFileDownloadResult,
                        >,
                    >,
                >,
                1usize,
            >("DownloadByName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DownloadByName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::AssetFileDownloadResult,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), (assetFileName)) };
        Ok(__cordl_ret.into())
    }
    pub fn DownloadCancel(
        assetFileID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::AssetFileDownloadCancelResult,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::AssetFileDownloadCancelResult,
                        >,
                    >,
                >,
                1usize,
            >("DownloadCancel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DownloadCancel", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::AssetFileDownloadCancelResult,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), (assetFileID)) };
        Ok(__cordl_ret.into())
    }
    pub fn DownloadCancelById(
        assetFileID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::AssetFileDownloadCancelResult,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::AssetFileDownloadCancelResult,
                        >,
                    >,
                >,
                1usize,
            >("DownloadCancelById")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DownloadCancelById", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::AssetFileDownloadCancelResult,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), (assetFileID)) };
        Ok(__cordl_ret.into())
    }
    pub fn DownloadCancelByName(
        assetFileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::AssetFileDownloadCancelResult,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::AssetFileDownloadCancelResult,
                        >,
                    >,
                >,
                1usize,
            >("DownloadCancelByName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DownloadCancelByName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::AssetFileDownloadCancelResult,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), (assetFileName)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetList() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::AssetDetailsList,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::AssetDetailsList,
                        >,
                    >,
                >,
                0usize,
            >("GetList")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetList", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::AssetDetailsList,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn SetDownloadUpdateNotificationCallback(
        callback: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1_Callback<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::AssetFileDownloadUpdate,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Message_1_Callback<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::AssetFileDownloadUpdate,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetDownloadUpdateNotificationCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetDownloadUpdateNotificationCallback", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (callback))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Status(
        assetFileID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::AssetDetails>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::AssetDetails,
                        >,
                    >,
                >,
                1usize,
            >("Status")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Status", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::AssetDetails>,
            >,
        > = unsafe { method.invoke_unchecked((), (assetFileID)) };
        Ok(__cordl_ret.into())
    }
    pub fn StatusById(
        assetFileID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::AssetDetails>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::AssetDetails,
                        >,
                    >,
                >,
                1usize,
            >("StatusById")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "StatusById", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::AssetDetails>,
            >,
        > = unsafe { method.invoke_unchecked((), (assetFileID)) };
        Ok(__cordl_ret.into())
    }
    pub fn StatusByName(
        assetFileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::AssetDetails>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::AssetDetails,
                        >,
                    >,
                >,
                1usize,
            >("StatusByName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "StatusByName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::AssetDetails>,
            >,
        > = unsafe { method.invoke_unchecked((), (assetFileName)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+AssetFile")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::AssetFile {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
