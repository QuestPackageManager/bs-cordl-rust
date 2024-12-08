#[cfg(feature = "UnityEngine+Android+AndroidAssetPacks")]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidAssetPacks {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPacks")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Android::AndroidAssetPacks =>
    "UnityEngine.Android"."AndroidAssetPacks"
);
#[cfg(feature = "UnityEngine+Android+AndroidAssetPacks")]
impl std::ops::Deref for crate::UnityEngine::Android::AndroidAssetPacks {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPacks")]
impl std::ops::DerefMut for crate::UnityEngine::Android::AndroidAssetPacks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPacks")]
impl crate::UnityEngine::Android::AndroidAssetPacks {
    #[cfg(
        feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerDownloadStatusCallback"
    )]
    pub type AssetPackManagerDownloadStatusCallback = crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerDownloadStatusCallback;
    #[cfg(
        feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerMobileDataConfirmationCallback"
    )]
    pub type AssetPackManagerMobileDataConfirmationCallback = crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerMobileDataConfirmationCallback;
    #[cfg(
        feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerStatusQueryCallback"
    )]
    pub type AssetPackManagerStatusQueryCallback = crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerStatusQueryCallback;
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPacks")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Android::AndroidAssetPacks {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerDownloadStatusCallback"
)]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidAssetPacks_AssetPackManagerDownloadStatusCallback {
    __cordl_parent: crate::UnityEngine::AndroidJavaProxy,
    pub m_Callback: *mut crate::System::Action_1<
        *mut crate::UnityEngine::Android::AndroidAssetPackInfo,
    >,
    pub m_AssetPacks: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
}
#[cfg(
    feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerDownloadStatusCallback"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerDownloadStatusCallback =>
    "UnityEngine.Android"."AndroidAssetPacks/AssetPackManagerDownloadStatusCallback"
);
#[cfg(
    feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerDownloadStatusCallback"
)]
impl std::ops::Deref
for crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerDownloadStatusCallback {
    type Target = crate::UnityEngine::AndroidJavaProxy;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerDownloadStatusCallback"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerDownloadStatusCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerDownloadStatusCallback"
)]
impl crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerDownloadStatusCallback {
    pub fn _ctor(
        &mut self,
        callback: *mut crate::System::Action_1<
            *mut crate::UnityEngine::Android::AndroidAssetPackInfo,
        >,
        assetPacks: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (callback, assetPacks))?;
        Ok(__cordl_ret)
    }
    pub fn onStatusUpdate(
        &mut self,
        assetPackName: *mut crate::System::String,
        assetPackStatus: i32,
        assetPackSize: i64,
        assetPackBytesDownloaded: i64,
        assetPackTransferProgress: i32,
        assetPackErrorCode: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "onStatusUpdate",
                (
                    assetPackName,
                    assetPackStatus,
                    assetPackSize,
                    assetPackBytesDownloaded,
                    assetPackTransferProgress,
                    assetPackErrorCode,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        callback: *mut crate::System::Action_1<
            *mut crate::UnityEngine::Android::AndroidAssetPackInfo,
        >,
        assetPacks: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (callback, assetPacks))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerDownloadStatusCallback"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerDownloadStatusCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerMobileDataConfirmationCallback"
)]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidAssetPacks_AssetPackManagerMobileDataConfirmationCallback {
    __cordl_parent: crate::UnityEngine::AndroidJavaProxy,
    pub m_Callback: *mut crate::System::Action_1<
        *mut crate::UnityEngine::Android::AndroidAssetPackUseMobileDataRequestResult,
    >,
}
#[cfg(
    feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerMobileDataConfirmationCallback"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerMobileDataConfirmationCallback
    => "UnityEngine.Android"
    ."AndroidAssetPacks/AssetPackManagerMobileDataConfirmationCallback"
);
#[cfg(
    feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerMobileDataConfirmationCallback"
)]
impl std::ops::Deref
for crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerMobileDataConfirmationCallback {
    type Target = crate::UnityEngine::AndroidJavaProxy;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerMobileDataConfirmationCallback"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerMobileDataConfirmationCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerMobileDataConfirmationCallback"
)]
impl crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerMobileDataConfirmationCallback {
    pub fn onMobileDataConfirmationResult(
        &mut self,
        allowed: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("onMobileDataConfirmationResult", (allowed))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        callback: *mut crate::System::Action_1<
            *mut crate::UnityEngine::Android::AndroidAssetPackUseMobileDataRequestResult,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (callback))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        callback: *mut crate::System::Action_1<
            *mut crate::UnityEngine::Android::AndroidAssetPackUseMobileDataRequestResult,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (callback))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerMobileDataConfirmationCallback"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerMobileDataConfirmationCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerStatusQueryCallback"
)]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidAssetPacks_AssetPackManagerStatusQueryCallback {
    __cordl_parent: crate::UnityEngine::AndroidJavaProxy,
    pub m_Callback: *mut crate::System::Action_2<
        u64,
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Android::AndroidAssetPackState,
        >,
    >,
    pub m_AssetPackNames: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
    pub m_States: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::Android::AndroidAssetPackState,
    >,
    pub m_Size: i64,
}
#[cfg(
    feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerStatusQueryCallback"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerStatusQueryCallback =>
    "UnityEngine.Android"."AndroidAssetPacks/AssetPackManagerStatusQueryCallback"
);
#[cfg(
    feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerStatusQueryCallback"
)]
impl std::ops::Deref
for crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerStatusQueryCallback {
    type Target = crate::UnityEngine::AndroidJavaProxy;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerStatusQueryCallback"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerStatusQueryCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerStatusQueryCallback"
)]
impl crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerStatusQueryCallback {
    pub fn onStatusResult(
        &mut self,
        totalBytes: i64,
        assetPackNames: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        >,
        assetPackStatuses: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        assetPackErrorCodes: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "onStatusResult",
                (totalBytes, assetPackNames, assetPackStatuses, assetPackErrorCodes),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        callback: *mut crate::System::Action_2<
            u64,
            *mut quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::Android::AndroidAssetPackState,
            >,
        >,
        assetPacks: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (callback, assetPacks))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        callback: *mut crate::System::Action_2<
            u64,
            *mut quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::Android::AndroidAssetPackState,
            >,
        >,
        assetPacks: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (callback, assetPacks))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerStatusQueryCallback"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerStatusQueryCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
