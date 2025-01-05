#[cfg(feature = "Oculus+Platform+AssetFile")]
#[repr(C)]
#[derive(Debug)]
pub struct AssetFile {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Oculus+Platform+AssetFile")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::AssetFile => "Oculus.Platform"
    ."AssetFile"
);
#[cfg(feature = "Oculus+Platform+AssetFile")]
impl std::ops::Deref for crate::Oculus::Platform::AssetFile {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::AssetFileDeleteResult,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::AssetFileDeleteResult,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Delete", (assetFileID))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteById(
        assetFileID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::AssetFileDeleteResult,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::AssetFileDeleteResult,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeleteById", (assetFileID))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteByName(
        assetFileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::AssetFileDeleteResult,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::AssetFileDeleteResult,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeleteByName", (assetFileName))?;
        Ok(__cordl_ret.into())
    }
    pub fn Download(
        assetFileID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::AssetFileDownloadResult,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::AssetFileDownloadResult,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Download", (assetFileID))?;
        Ok(__cordl_ret.into())
    }
    pub fn DownloadById(
        assetFileID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::AssetFileDownloadResult,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::AssetFileDownloadResult,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DownloadById", (assetFileID))?;
        Ok(__cordl_ret.into())
    }
    pub fn DownloadByName(
        assetFileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::AssetFileDownloadResult,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::AssetFileDownloadResult,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DownloadByName", (assetFileName))?;
        Ok(__cordl_ret.into())
    }
    pub fn DownloadCancel(
        assetFileID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::AssetFileDownloadCancelResult,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::AssetFileDownloadCancelResult,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DownloadCancel", (assetFileID))?;
        Ok(__cordl_ret.into())
    }
    pub fn DownloadCancelById(
        assetFileID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::AssetFileDownloadCancelResult,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::AssetFileDownloadCancelResult,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DownloadCancelById", (assetFileID))?;
        Ok(__cordl_ret.into())
    }
    pub fn DownloadCancelByName(
        assetFileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::AssetFileDownloadCancelResult,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::AssetFileDownloadCancelResult,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DownloadCancelByName", (assetFileName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetList() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::AssetDetailsList>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::AssetDetailsList>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDownloadUpdateNotificationCallback(
        callback: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::AssetFileDownloadUpdate,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetDownloadUpdateNotificationCallback", (callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn Status(
        assetFileID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::AssetDetails>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::AssetDetails>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Status", (assetFileID))?;
        Ok(__cordl_ret.into())
    }
    pub fn StatusById(
        assetFileID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::AssetDetails>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::AssetDetails>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StatusById", (assetFileID))?;
        Ok(__cordl_ret.into())
    }
    pub fn StatusByName(
        assetFileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::AssetDetails>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::AssetDetails>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StatusByName", (assetFileName))?;
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
