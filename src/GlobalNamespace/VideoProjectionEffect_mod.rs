#[cfg(feature = "VideoProjectionEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct VideoProjectionEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _videoProjectionDataModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::VideoProjectionDataModelSO,
    >,
    pub _videoPlayer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SongTimeSyncedVideoPlayer,
    >,
    pub _videoEventType: crate::GlobalNamespace::BasicBeatmapEventType,
    pub _initData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::VideoProjectionEffect_InitData,
    >,
    pub _environmentContext: crate::GlobalNamespace::EnvironmentContext,
    pub _beatmapData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IReadonlyBeatmapData,
    >,
    pub _beatmapCallbacksController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCallbacksController,
    >,
    pub _behavior: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::VideoProjectionEffect_VideoProjectionBehavior,
    >,
}
#[cfg(feature = "VideoProjectionEffect")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::VideoProjectionEffect {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "VideoProjectionEffect";
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
#[cfg(feature = "VideoProjectionEffect")]
impl std::ops::Deref for crate::GlobalNamespace::VideoProjectionEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VideoProjectionEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::VideoProjectionEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VideoProjectionEffect")]
impl crate::GlobalNamespace::VideoProjectionEffect {
    #[cfg(feature = "VideoProjectionEffect+BeatmapEditorVideoProjectionBehavior")]
    pub type BeatmapEditorVideoProjectionBehavior = crate::GlobalNamespace::VideoProjectionEffect_BeatmapEditorVideoProjectionBehavior;
    #[cfg(feature = "VideoProjectionEffect+GameplayVideoProjectionBehavior")]
    pub type GameplayVideoProjectionBehavior = crate::GlobalNamespace::VideoProjectionEffect_GameplayVideoProjectionBehavior;
    #[cfg(feature = "VideoProjectionEffect+InitData")]
    pub type InitData = crate::GlobalNamespace::VideoProjectionEffect_InitData;
    #[cfg(feature = "VideoProjectionEffect+VideoProjectionBehavior")]
    pub type VideoProjectionBehavior = crate::GlobalNamespace::VideoProjectionEffect_VideoProjectionBehavior;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "VideoProjectionEffect")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::VideoProjectionEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "VideoProjectionEffect+BeatmapEditorVideoProjectionBehavior")]
#[repr(C)]
#[derive(Debug)]
pub struct VideoProjectionEffect_BeatmapEditorVideoProjectionBehavior {
    __cordl_parent: crate::GlobalNamespace::VideoProjectionEffect_VideoProjectionBehavior,
    pub _beatmapCallbacksController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCallbacksController,
    >,
    pub _callbackWrapper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapDataCallbackWrapper,
    >,
}
#[cfg(feature = "VideoProjectionEffect+BeatmapEditorVideoProjectionBehavior")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::VideoProjectionEffect_BeatmapEditorVideoProjectionBehavior {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapEditorVideoProjectionBehavior";
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
#[cfg(feature = "VideoProjectionEffect+BeatmapEditorVideoProjectionBehavior")]
impl std::ops::Deref
for crate::GlobalNamespace::VideoProjectionEffect_BeatmapEditorVideoProjectionBehavior {
    type Target = crate::GlobalNamespace::VideoProjectionEffect_VideoProjectionBehavior;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VideoProjectionEffect+BeatmapEditorVideoProjectionBehavior")]
impl std::ops::DerefMut
for crate::GlobalNamespace::VideoProjectionEffect_BeatmapEditorVideoProjectionBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VideoProjectionEffect+BeatmapEditorVideoProjectionBehavior")]
impl crate::GlobalNamespace::VideoProjectionEffect_BeatmapEditorVideoProjectionBehavior {
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
    pub fn HandleBeatmapEvent(
        &mut self,
        data: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BasicBeatmapEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBeatmapEvent", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        beatmapData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        >,
        dataModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VideoProjectionDataModelSO,
        >,
        videoPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SongTimeSyncedVideoPlayer,
        >,
        videoEventType: crate::GlobalNamespace::BasicBeatmapEventType,
        beatmapCallbacksController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCallbacksController,
        >,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    beatmapData,
                    dataModel,
                    videoPlayer,
                    videoEventType,
                    beatmapCallbacksController,
                    beatmapLevel,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        beatmapData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        >,
        dataModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VideoProjectionDataModelSO,
        >,
        videoPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SongTimeSyncedVideoPlayer,
        >,
        videoEventType: crate::GlobalNamespace::BasicBeatmapEventType,
        beatmapCallbacksController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCallbacksController,
        >,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    beatmapData,
                    dataModel,
                    videoPlayer,
                    videoEventType,
                    beatmapCallbacksController,
                    beatmapLevel,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "VideoProjectionEffect+BeatmapEditorVideoProjectionBehavior")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::VideoProjectionEffect_BeatmapEditorVideoProjectionBehavior {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "VideoProjectionEffect+GameplayVideoProjectionBehavior")]
#[repr(C)]
#[derive(Debug)]
pub struct VideoProjectionEffect_GameplayVideoProjectionBehavior {
    __cordl_parent: crate::GlobalNamespace::VideoProjectionEffect_VideoProjectionBehavior,
}
#[cfg(feature = "VideoProjectionEffect+GameplayVideoProjectionBehavior")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::VideoProjectionEffect_GameplayVideoProjectionBehavior {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GameplayVideoProjectionBehavior";
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
#[cfg(feature = "VideoProjectionEffect+GameplayVideoProjectionBehavior")]
impl std::ops::Deref
for crate::GlobalNamespace::VideoProjectionEffect_GameplayVideoProjectionBehavior {
    type Target = crate::GlobalNamespace::VideoProjectionEffect_VideoProjectionBehavior;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VideoProjectionEffect+GameplayVideoProjectionBehavior")]
impl std::ops::DerefMut
for crate::GlobalNamespace::VideoProjectionEffect_GameplayVideoProjectionBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VideoProjectionEffect+GameplayVideoProjectionBehavior")]
impl crate::GlobalNamespace::VideoProjectionEffect_GameplayVideoProjectionBehavior {
    pub fn New(
        beatmapData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        >,
        dataModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VideoProjectionDataModelSO,
        >,
        videoPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SongTimeSyncedVideoPlayer,
        >,
        videoEventType: crate::GlobalNamespace::BasicBeatmapEventType,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (beatmapData, dataModel, videoPlayer, videoEventType, beatmapLevel),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        beatmapData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        >,
        dataModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VideoProjectionDataModelSO,
        >,
        videoPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SongTimeSyncedVideoPlayer,
        >,
        videoEventType: crate::GlobalNamespace::BasicBeatmapEventType,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (beatmapData, dataModel, videoPlayer, videoEventType, beatmapLevel),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "VideoProjectionEffect+GameplayVideoProjectionBehavior")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::VideoProjectionEffect_GameplayVideoProjectionBehavior {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "VideoProjectionEffect+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct VideoProjectionEffect_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
}
#[cfg(feature = "VideoProjectionEffect+InitData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::VideoProjectionEffect_InitData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "InitData";
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
#[cfg(feature = "VideoProjectionEffect+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::VideoProjectionEffect_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VideoProjectionEffect+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::VideoProjectionEffect_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VideoProjectionEffect+InitData")]
impl crate::GlobalNamespace::VideoProjectionEffect_InitData {
    pub fn New(
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beatmapLevel))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beatmapLevel))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "VideoProjectionEffect+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::VideoProjectionEffect_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "VideoProjectionEffect+VideoProjectionBehavior")]
#[repr(C)]
#[derive(Debug)]
pub struct VideoProjectionEffect_VideoProjectionBehavior {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _coverLoaded: bool,
    pub _eventValue: i32,
    pub _videoClipHandle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Video::VideoClip>,
    >,
    pub _model: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::VideoProjectionDataModelSO,
    >,
    pub _videoPlayer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SongTimeSyncedVideoPlayer,
    >,
    pub _beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
}
#[cfg(feature = "VideoProjectionEffect+VideoProjectionBehavior")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::VideoProjectionEffect_VideoProjectionBehavior {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "VideoProjectionBehavior";
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
#[cfg(feature = "VideoProjectionEffect+VideoProjectionBehavior")]
impl std::ops::Deref
for crate::GlobalNamespace::VideoProjectionEffect_VideoProjectionBehavior {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VideoProjectionEffect+VideoProjectionBehavior")]
impl std::ops::DerefMut
for crate::GlobalNamespace::VideoProjectionEffect_VideoProjectionBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VideoProjectionEffect+VideoProjectionBehavior")]
impl crate::GlobalNamespace::VideoProjectionEffect_VideoProjectionBehavior {
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
    pub fn LoadPreviewCoverAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadPreviewCoverAsset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadVideoFromModel(
        &mut self,
        eventValue: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadVideoFromModel", (eventValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        dataModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VideoProjectionDataModelSO,
        >,
        videoPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SongTimeSyncedVideoPlayer,
        >,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dataModel, videoPlayer, beatmapLevel))?;
        Ok(__cordl_object.into())
    }
    pub fn UnloadCoverAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnloadCoverAsset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        dataModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VideoProjectionDataModelSO,
        >,
        videoPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SongTimeSyncedVideoPlayer,
        >,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dataModel, videoPlayer, beatmapLevel))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "VideoProjectionEffect+VideoProjectionBehavior")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::VideoProjectionEffect_VideoProjectionBehavior {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
