#[cfg(feature = "UnityEngine+Timeline+TimelineAsset")]
#[repr(C)]
#[derive(Debug)]
pub struct TimelineAsset {
    __cordl_parent: crate::UnityEngine::Playables::PlayableAsset,
    pub m_Version: i32,
    pub m_Tracks: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::ScriptableObject,
        >,
    >,
    pub m_FixedDuration: f64,
    pub m_CacheOutputTracks: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Timeline::TrackAsset>,
    >,
    pub m_CacheRootTracks: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::Timeline::TrackAsset,
        >,
    >,
    pub m_CacheFlattenedTracks: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Timeline::TrackAsset>,
    >,
    pub m_EditorSettings: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Timeline::TimelineAsset_EditorSettings,
    >,
    pub m_DurationMode: crate::UnityEngine::Timeline::TimelineAsset_DurationMode,
    pub m_MarkerTrack: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Timeline::MarkerTrack,
    >,
}
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::TimelineAsset =>
    "UnityEngine.Timeline"."TimelineAsset"
);
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset")]
impl std::ops::Deref for crate::UnityEngine::Timeline::TimelineAsset {
    type Target = crate::UnityEngine::Playables::PlayableAsset;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::TimelineAsset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset")]
impl crate::UnityEngine::Timeline::TimelineAsset {
    pub const k_LatestVersion: i32 = 0i32;
    #[cfg(feature = "UnityEngine+Timeline+TimelineAsset+DurationMode")]
    pub type DurationMode = crate::UnityEngine::Timeline::TimelineAsset_DurationMode;
    #[cfg(feature = "UnityEngine+Timeline+TimelineAsset+EditorSettings")]
    pub type EditorSettings = crate::UnityEngine::Timeline::TimelineAsset_EditorSettings;
    #[cfg(feature = "UnityEngine+Timeline+TimelineAsset+MediaType")]
    pub type MediaType = crate::UnityEngine::Timeline::TimelineAsset_MediaType;
    #[cfg(feature = "UnityEngine+Timeline+TimelineAsset+TimelineAssetUpgrade")]
    pub type TimelineAssetUpgrade = crate::UnityEngine::Timeline::TimelineAsset_TimelineAssetUpgrade;
    #[cfg(feature = "UnityEngine+Timeline+TimelineAsset+Versions")]
    pub type Versions = crate::UnityEngine::Timeline::TimelineAsset_Versions;
    pub fn AddSubTracksRecursive(
        track: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
        allTracks: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::Timeline::TrackAsset,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddSubTracksRecursive", (track, allTracks))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddTrackInternal(
        &mut self,
        track: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddTrackInternal", (track))?;
        Ok(__cordl_ret.into())
    }
    pub fn AllocateTrack(
        &mut self,
        trackAssetParent: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TrackAsset,
        >,
        trackName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        trackType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TrackAsset,
        > = __cordl_object
            .invoke("AllocateTrack", (trackAssetParent, trackName, trackType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateItemsDuration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Timeline::DiscreteTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Timeline::DiscreteTime = __cordl_object
            .invoke("CalculateItemsDuration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateMarkerTrack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateMarkerTrack", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreatePlayable(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::Playable = __cordl_object
            .invoke("CreatePlayable", (graph, go))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateTrack_3<T>(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("CreateTrack", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateTrack_Il2CppString2<T>(
        &mut self,
        trackName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("CreateTrack", (trackName))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateTrack_TrackAsset_Il2CppString1<T>(
        &mut self,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
        trackName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("CreateTrack", (parent, trackName))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateTrack_Type_TrackAsset_Il2CppString0(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TrackAsset,
        > = __cordl_object.invoke("CreateTrack", (_cordl_type, parent, name))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteClip(
        &mut self,
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("DeleteClip", (clip))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteRecordedAnimation_TimelineClip1(
        &mut self,
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeleteRecordedAnimation", (clip))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteRecordedAnimation_TrackAsset0(
        &mut self,
        track: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeleteRecordedAnimation", (track))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteTrack(
        &mut self,
        track: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("DeleteTrack", (track))?;
        Ok(__cordl_ret.into())
    }
    pub fn GatherProperties(
        &mut self,
        director: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Playables::PlayableDirector,
        >,
        driver: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::IPropertyCollector,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GatherProperties", (director, driver))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOutputTrack(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TrackAsset,
        > = __cordl_object.invoke("GetOutputTrack", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOutputTracks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::Timeline::TrackAsset,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::Timeline::TrackAsset,
            >,
        > = __cordl_object.invoke("GetOutputTracks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRootTrack(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TrackAsset,
        > = __cordl_object.invoke("GetRootTrack", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRootTracks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::Timeline::TrackAsset,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::Timeline::TrackAsset,
            >,
        > = __cordl_object.invoke("GetRootTracks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValidFrameRate(frameRate: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetValidFrameRate", (frameRate))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invalidate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invalidate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveLastTrackBefore(
        &mut self,
        asset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveLastTrackBefore", (asset))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnValidate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnValidate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveTrack(
        &mut self,
        track: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveTrack", (track))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ISerializationCallbackReceiver_OnAfterDeserialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.ISerializationCallbackReceiver.OnAfterDeserialize",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ISerializationCallbackReceiver_OnBeforeSerialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.ISerializationCallbackReceiver.OnBeforeSerialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateFixedDurationWithItemsDuration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateFixedDurationWithItemsDuration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateOutputTrackCache(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateOutputTrackCache", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateRootTrackCache(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateRootTrackCache", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpgradeToLatestVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpgradeToLatestVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn __internalAwake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("__internalAwake", ())?;
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
    pub fn get_clipCaps(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Timeline::ClipCaps> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Timeline::ClipCaps = __cordl_object
            .invoke("get_clipCaps", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_duration(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_duration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_durationMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Timeline::TimelineAsset_DurationMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Timeline::TimelineAsset_DurationMode = __cordl_object
            .invoke("get_durationMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_editorSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TimelineAsset_EditorSettings,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TimelineAsset_EditorSettings,
        > = __cordl_object.invoke("get_editorSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fixedDuration(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_fixedDuration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_flattenedTracks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::Timeline::TrackAsset,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::Timeline::TrackAsset,
            >,
        > = __cordl_object.invoke("get_flattenedTracks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_markerTrack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::MarkerTrack>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::MarkerTrack,
        > = __cordl_object.invoke("get_markerTrack", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_outputTrackCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_outputTrackCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_outputs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::Playables::PlayableBinding,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::Playables::PlayableBinding,
            >,
        > = __cordl_object.invoke("get_outputs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rootTrackCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_rootTrackCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_trackObjects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::ScriptableObject,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::ScriptableObject,
            >,
        > = __cordl_object.invoke("get_trackObjects", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_durationMode(
        &mut self,
        value: crate::UnityEngine::Timeline::TimelineAsset_DurationMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_durationMode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_fixedDuration(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fixedDuration", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Timeline::TimelineAsset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset")]
impl AsRef<crate::UnityEngine::ISerializationCallbackReceiver>
for crate::UnityEngine::Timeline::TimelineAsset {
    fn as_ref(&self) -> &crate::UnityEngine::ISerializationCallbackReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset")]
impl AsMut<crate::UnityEngine::ISerializationCallbackReceiver>
for crate::UnityEngine::Timeline::TimelineAsset {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::ISerializationCallbackReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset")]
impl AsRef<crate::UnityEngine::Timeline::IPropertyPreview>
for crate::UnityEngine::Timeline::TimelineAsset {
    fn as_ref(&self) -> &crate::UnityEngine::Timeline::IPropertyPreview {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset")]
impl AsMut<crate::UnityEngine::Timeline::IPropertyPreview>
for crate::UnityEngine::Timeline::TimelineAsset {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Timeline::IPropertyPreview {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset")]
impl AsRef<crate::UnityEngine::Timeline::ITimelineClipAsset>
for crate::UnityEngine::Timeline::TimelineAsset {
    fn as_ref(&self) -> &crate::UnityEngine::Timeline::ITimelineClipAsset {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset")]
impl AsMut<crate::UnityEngine::Timeline::ITimelineClipAsset>
for crate::UnityEngine::Timeline::TimelineAsset {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Timeline::ITimelineClipAsset {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset+DurationMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimelineAsset_DurationMode {
    BasedOnClips = 0i32,
    FixedLength = 1i32,
}
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset+DurationMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Timeline::TimelineAsset_DurationMode => "UnityEngine.Timeline"
    ."TimelineAsset/DurationMode"
);
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset+EditorSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct TimelineAsset_EditorSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Framerate: f64,
    pub m_ScenePreview: bool,
}
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset+EditorSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Timeline::TimelineAsset_EditorSettings => "UnityEngine.Timeline"
    ."TimelineAsset/EditorSettings"
);
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset+EditorSettings")]
impl std::ops::Deref for crate::UnityEngine::Timeline::TimelineAsset_EditorSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset+EditorSettings")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::TimelineAsset_EditorSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset+EditorSettings")]
impl crate::UnityEngine::Timeline::TimelineAsset_EditorSettings {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetStandardFrameRate(
        &mut self,
        enumValue: crate::UnityEngine::Timeline::StandardFrameRates,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStandardFrameRate", (enumValue))?;
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
    pub fn get_fps(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_fps", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_frameRate(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_frameRate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scenePreview(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_scenePreview", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_fps(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fps", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_frameRate(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_frameRate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_scenePreview(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_scenePreview", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset+EditorSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::TimelineAsset_EditorSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset+MediaType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimelineAsset_MediaType {
    Animation = 0i32,
    Audio = 1i32,
    Group = 5i32,
    Hybrid = 4i32,
    Script = 3i32,
    Texture = 2i32,
}
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset+MediaType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::TimelineAsset_MediaType
    => "UnityEngine.Timeline"."TimelineAsset/MediaType"
);
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset+TimelineAssetUpgrade")]
#[repr(C)]
#[derive(Debug)]
pub struct TimelineAsset_TimelineAssetUpgrade {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset+TimelineAssetUpgrade")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Timeline::TimelineAsset_TimelineAssetUpgrade => "UnityEngine.Timeline"
    ."TimelineAsset/TimelineAssetUpgrade"
);
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset+TimelineAssetUpgrade")]
impl std::ops::Deref
for crate::UnityEngine::Timeline::TimelineAsset_TimelineAssetUpgrade {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset+TimelineAssetUpgrade")]
impl std::ops::DerefMut
for crate::UnityEngine::Timeline::TimelineAsset_TimelineAssetUpgrade {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset+TimelineAssetUpgrade")]
impl crate::UnityEngine::Timeline::TimelineAsset_TimelineAssetUpgrade {}
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset+TimelineAssetUpgrade")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::TimelineAsset_TimelineAssetUpgrade {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset+Versions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimelineAsset_Versions {
    Initial = 0i32,
}
#[cfg(feature = "UnityEngine+Timeline+TimelineAsset+Versions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::TimelineAsset_Versions =>
    "UnityEngine.Timeline"."TimelineAsset/Versions"
);
