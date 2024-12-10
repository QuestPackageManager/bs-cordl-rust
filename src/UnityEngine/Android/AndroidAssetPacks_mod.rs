#[cfg(feature = "UnityEngine+Android+AndroidAssetPacks")]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidAssetPacks {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPacks")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Android::AndroidAssetPacks =>
    "UnityEngine.Android"."AndroidAssetPacks"
);
#[cfg(feature = "UnityEngine+Android+AndroidAssetPacks")]
impl std::ops::Deref for crate::UnityEngine::Android::AndroidAssetPacks {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        *mut quest_hook::libil2cpp::Il2CppString,
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
    pub fn New(
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::UnityEngine::Android::AndroidAssetPackInfo,
            >,
        >,
        assetPacks: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (callback, assetPacks))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::UnityEngine::Android::AndroidAssetPackInfo,
            >,
        >,
        assetPacks: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (callback, assetPacks))?;
        Ok(__cordl_ret.into())
    }
    pub fn onStatusUpdate(
        &mut self,
        assetPackName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
        Ok(__cordl_ret.into())
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
    pub fn New(
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::UnityEngine::Android::AndroidAssetPackUseMobileDataRequestResult,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (callback))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::UnityEngine::Android::AndroidAssetPackUseMobileDataRequestResult,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn onMobileDataConfirmationResult(
        &mut self,
        allowed: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("onMobileDataConfirmationResult", (allowed))?;
        Ok(__cordl_ret.into())
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
        *mut quest_hook::libil2cpp::Il2CppString,
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
    pub fn New(
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                u64,
                *mut quest_hook::libil2cpp::Il2CppArray<
                    *mut crate::UnityEngine::Android::AndroidAssetPackState,
                >,
            >,
        >,
        assetPacks: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (callback, assetPacks))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                u64,
                *mut quest_hook::libil2cpp::Il2CppArray<
                    *mut crate::UnityEngine::Android::AndroidAssetPackState,
                >,
            >,
        >,
        assetPacks: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (callback, assetPacks))?;
        Ok(__cordl_ret.into())
    }
    pub fn onStatusResult(
        &mut self,
        totalBytes: i64,
        assetPackNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        assetPackStatuses: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        >,
        assetPackErrorCodes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "onStatusResult",
                (totalBytes, assetPackNames, assetPackStatuses, assetPackErrorCodes),
            )?;
        Ok(__cordl_ret.into())
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
