#[cfg(feature = "BeatmapLevelDataLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelDataLoader {
    __cordl_parent: crate::System::Object,
    pub _loadRequests: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut BeatmapLevelDataLoadRequest,
    >,
}
#[cfg(feature = "BeatmapLevelDataLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapLevelDataLoader => ""."BeatmapLevelDataLoader"
);
#[cfg(feature = "BeatmapLevelDataLoader")]
impl std::ops::Deref for BeatmapLevelDataLoader {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelDataLoader")]
impl std::ops::DerefMut for BeatmapLevelDataLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelDataLoader")]
impl BeatmapLevelDataLoader {
    #[cfg(
        feature = "BeatmapLevelDataLoader+_CheckBeatmapLevelDataFromAssetBundleExistsAsync_d__4"
    )]
    pub type _CheckBeatmapLevelDataFromAssetBundleExistsAsync_d__4 = crate::GlobalNamespace::BeatmapLevelDataLoader__CheckBeatmapLevelDataFromAssetBundleExistsAsync_d__4;
    pub fn TryUnload(
        &mut self,
        beatmapLevelId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryUnload", (beatmapLevelId))?;
        Ok(__cordl_ret)
    }
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
    pub fn CheckBeatmapLevelDataFromAssetBundleExistsAsync(
        &mut self,
        levelId: *mut crate::System::String,
        assetBundlePath: *mut crate::System::String,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<bool>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<bool> = __cordl_object
            .invoke(
                "CheckBeatmapLevelDataFromAssetBundleExistsAsync",
                (levelId, assetBundlePath, cancellationToken),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_loadRequestCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_loadRequestCount", ())?;
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
    pub fn LoadBeatmapLevelDataFromAssetBundleAsync(
        &mut self,
        levelId: *mut crate::System::String,
        assetBundlePath: *mut crate::System::String,
        levelDataAssetName: *mut crate::System::String,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut IBeatmapLevelData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut IBeatmapLevelData,
        > = __cordl_object
            .invoke(
                "LoadBeatmapLevelDataFromAssetBundleAsync",
                (levelId, assetBundlePath, levelDataAssetName, cancellationToken),
            )?;
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
#[cfg(feature = "BeatmapLevelDataLoader")]
impl quest_hook::libil2cpp::ObjectType for BeatmapLevelDataLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
