#[cfg(feature = "BeatmapLevelDataLoadRequest")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelDataLoadRequest {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _task: quest_hook::libil2cpp::Gc<
        crate::System::Threading::Tasks::Task_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatmapLevelData>,
        >,
    >,
    pub _assetBundlePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _levelDataAssetName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _internalCancellationSource: quest_hook::libil2cpp::Gc<
        crate::System::Threading::CancellationTokenSource,
    >,
    pub _externalCancellationTokens: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::System::Threading::CancellationToken,
        >,
    >,
    pub assetBundle: quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundle>,
}
#[cfg(feature = "BeatmapLevelDataLoadRequest")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapLevelDataLoadRequest {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapLevelDataLoadRequest";
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
#[cfg(feature = "BeatmapLevelDataLoadRequest")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapLevelDataLoadRequest {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadDataAsync(
        &mut self,
        externalCancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatmapLevelData>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatmapLevelData>,
            >,
        > = __cordl_object.invoke("LoadDataAsync", (externalCancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadDataAsyncInternal(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatmapLevelData>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatmapLevelData>,
            >,
        > = __cordl_object.invoke("LoadDataAsyncInternal", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        assetBundlePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        levelDataAssetName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (assetBundlePath, levelDataAssetName))?;
        Ok(__cordl_object.into())
    }
    pub fn ThrowIfExternalCancellationRequested(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowIfExternalCancellationRequested", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnloadBundle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnloadBundle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        assetBundlePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        levelDataAssetName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (assetBundlePath, levelDataAssetName))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasFailed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasFailed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasValidResult(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasValidResult", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "BeatmapLevelDataLoadRequest")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::BeatmapLevelDataLoadRequest {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapLevelDataLoadRequest")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::BeatmapLevelDataLoadRequest {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
