#[cfg(feature = "BeatmapLevelDataLoadRequest")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelDataLoadRequest {
    __cordl_parent: crate::System::Object,
    pub _task: *mut crate::System::Threading::Tasks::Task_1<
        *mut crate::GlobalNamespace::IBeatmapLevelData,
    >,
    pub _assetBundlePath: *mut crate::System::String,
    pub _levelDataAssetName: *mut crate::System::String,
    pub _internalCancellationSource: *mut crate::System::Threading::CancellationTokenSource,
    pub _externalCancellationTokens: *mut crate::System::Collections::Generic::List_1<
        crate::System::Threading::CancellationToken,
    >,
    pub assetBundle: *mut crate::UnityEngine::AssetBundle,
}
#[cfg(feature = "BeatmapLevelDataLoadRequest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapLevelDataLoadRequest =>
    ""."BeatmapLevelDataLoadRequest"
);
#[cfg(feature = "BeatmapLevelDataLoadRequest")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapLevelDataLoadRequest {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelDataLoadRequest")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapLevelDataLoadRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelDataLoadRequest")]
impl crate::GlobalNamespace::BeatmapLevelDataLoadRequest {
    #[cfg(feature = "BeatmapLevelDataLoadRequest+_LoadDataAsyncInternal_d__8")]
    pub type _LoadDataAsyncInternal_d__8 = crate::GlobalNamespace::BeatmapLevelDataLoadRequest__LoadDataAsyncInternal_d__8;
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadDataAsync(
        &mut self,
        externalCancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::GlobalNamespace::IBeatmapLevelData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::GlobalNamespace::IBeatmapLevelData,
        > = __cordl_object.invoke("LoadDataAsync", (externalCancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn LoadDataAsyncInternal(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::GlobalNamespace::IBeatmapLevelData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::GlobalNamespace::IBeatmapLevelData,
        > = __cordl_object.invoke("LoadDataAsyncInternal", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        assetBundlePath: *mut crate::System::String,
        levelDataAssetName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (assetBundlePath, levelDataAssetName))?;
        Ok(__cordl_object)
    }
    pub fn ThrowIfExternalCancellationRequested(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowIfExternalCancellationRequested", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnloadBundle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnloadBundle", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        assetBundlePath: *mut crate::System::String,
        levelDataAssetName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (assetBundlePath, levelDataAssetName))?;
        Ok(__cordl_ret)
    }
    pub fn get_HasFailed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasFailed", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasValidResult(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasValidResult", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapLevelDataLoadRequest")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapLevelDataLoadRequest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
