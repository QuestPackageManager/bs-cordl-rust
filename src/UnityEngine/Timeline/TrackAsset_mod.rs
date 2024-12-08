#[cfg(feature = "UnityEngine+Timeline+TrackAsset")]
#[repr(C)]
#[derive(Debug)]
pub struct TrackAsset {
    __cordl_parent: crate::UnityEngine::Playables::PlayableAsset,
    pub m_Version: i32,
    pub m_AnimClip: *mut crate::UnityEngine::AnimationClip,
    pub m_Locked: bool,
    pub m_Muted: bool,
    pub m_CustomPlayableFullTypename: *mut crate::System::String,
    pub m_Curves: *mut crate::UnityEngine::AnimationClip,
    pub m_Parent: *mut crate::UnityEngine::Playables::PlayableAsset,
    pub m_Children: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::ScriptableObject,
    >,
    pub m_ItemsHash: i32,
    pub m_ClipsCache: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::Timeline::TimelineClip,
    >,
    pub m_Start: crate::UnityEngine::Timeline::DiscreteTime,
    pub m_End: crate::UnityEngine::Timeline::DiscreteTime,
    pub m_CacheSorted: bool,
    pub m_SupportsNotifications: crate::System::Nullable_1<bool>,
    pub m_ChildTrackCache: *mut crate::System::Collections::Generic::IEnumerable_1<
        *mut crate::UnityEngine::Timeline::TrackAsset,
    >,
    pub m_Clips: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::Timeline::TimelineClip,
    >,
    pub m_Markers: crate::UnityEngine::Timeline::MarkerList,
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::TrackAsset =>
    "UnityEngine.Timeline"."TrackAsset"
);
#[cfg(feature = "UnityEngine+Timeline+TrackAsset")]
impl std::ops::Deref for crate::UnityEngine::Timeline::TrackAsset {
    type Target = crate::UnityEngine::Playables::PlayableAsset;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::TrackAsset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset")]
impl crate::UnityEngine::Timeline::TrackAsset {
    pub const kDefaultCurvesName: &'static str = "Track Parameters";
    pub const k_LatestVersion: i32 = 3i32;
    #[cfg(feature = "UnityEngine+Timeline+TrackAsset+__c")]
    pub type __c = crate::UnityEngine::Timeline::TrackAsset___c;
    #[cfg(feature = "UnityEngine+Timeline+TrackAsset+TransientBuildData")]
    pub type TransientBuildData = crate::UnityEngine::Timeline::TrackAsset_TransientBuildData;
    #[cfg(feature = "UnityEngine+Timeline+TrackAsset+_get_outputs_d__65")]
    pub type _get_outputs_d__65 = crate::UnityEngine::Timeline::TrackAsset__get_outputs_d__65;
    #[cfg(feature = "UnityEngine+Timeline+TrackAsset+Versions")]
    pub type Versions = crate::UnityEngine::Timeline::TrackAsset_Versions;
    #[cfg(feature = "UnityEngine+Timeline+TrackAsset+TrackAssetUpgrade")]
    pub type TrackAssetUpgrade = crate::UnityEngine::Timeline::TrackAsset_TrackAssetUpgrade;
    pub fn AddChild(
        &mut self,
        child: *mut crate::UnityEngine::Timeline::TrackAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddChild", (child))?;
        Ok(__cordl_ret)
    }
    pub fn AddClip(
        &mut self,
        newClip: *mut crate::UnityEngine::Timeline::TimelineClip,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddClip", (newClip))?;
        Ok(__cordl_ret)
    }
    pub fn AddMarker(
        &mut self,
        e: *mut crate::UnityEngine::ScriptableObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddMarker", (e))?;
        Ok(__cordl_ret)
    }
    pub fn CalculateItemsHash(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CalculateItemsHash", ())?;
        Ok(__cordl_ret)
    }
    pub fn CanCompileClips(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanCompileClips", ())?;
        Ok(__cordl_ret)
    }
    pub fn CanCompileNotifications(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanCompileNotifications", ())?;
        Ok(__cordl_ret)
    }
    pub fn CanCreateMixerRecursive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanCreateMixerRecursive", ())?;
        Ok(__cordl_ret)
    }
    pub fn CanCreateTrackMixer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanCreateTrackMixer", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearClipsInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearClipsInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearMarkers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearMarkers", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearSubTracksInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearSubTracksInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn CompileClips(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        go: *mut crate::UnityEngine::GameObject,
        timelineClips: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::Timeline::TimelineClip,
        >,
        tree: *mut crate::UnityEngine::Timeline::IntervalTree_1<
            *mut crate::UnityEngine::Timeline::RuntimeElement,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::Playable = __cordl_object
            .invoke("CompileClips", (graph, go, timelineClips, tree))?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureTrackAnimation(
        &mut self,
        tree: *mut crate::UnityEngine::Timeline::IntervalTree_1<
            *mut crate::UnityEngine::Timeline::RuntimeElement,
        >,
        go: *mut crate::UnityEngine::GameObject,
        blend: crate::UnityEngine::Playables::Playable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureTrackAnimation", (tree, go, blend))?;
        Ok(__cordl_ret)
    }
    pub fn CreateAndAddNewClipOfType(
        &mut self,
        requestedType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Timeline::TimelineClip> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Timeline::TimelineClip = __cordl_object
            .invoke("CreateAndAddNewClipOfType", (requestedType))?;
        Ok(__cordl_ret)
    }
    pub fn CreateClipFromAsset(
        &mut self,
        playableAsset: *mut crate::UnityEngine::ScriptableObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Timeline::TimelineClip> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Timeline::TimelineClip = __cordl_object
            .invoke("CreateClipFromAsset", (playableAsset))?;
        Ok(__cordl_ret)
    }
    pub fn CreateClipFromPlayableAsset(
        &mut self,
        asset: *mut crate::UnityEngine::Playables::IPlayableAsset,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Timeline::TimelineClip> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Timeline::TimelineClip = __cordl_object
            .invoke("CreateClipFromPlayableAsset", (asset))?;
        Ok(__cordl_ret)
    }
    pub fn CreateClipOfType(
        &mut self,
        requestedType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Timeline::TimelineClip> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Timeline::TimelineClip = __cordl_object
            .invoke("CreateClipOfType", (requestedType))?;
        Ok(__cordl_ret)
    }
    pub fn CreateClip_0<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Timeline::TimelineClip>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Timeline::TimelineClip = __cordl_object
            .invoke("CreateClip", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateClip_Type1(
        &mut self,
        requestedType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Timeline::TimelineClip> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Timeline::TimelineClip = __cordl_object
            .invoke("CreateClip", (requestedType))?;
        Ok(__cordl_ret)
    }
    pub fn CreateCurves(
        &mut self,
        curvesClipName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateCurves", (curvesClipName))?;
        Ok(__cordl_ret)
    }
    pub fn CreateDefaultClip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Timeline::TimelineClip> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Timeline::TimelineClip = __cordl_object
            .invoke("CreateDefaultClip", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateMarker_Type_f64_0(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        _cordl_time: f64,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Timeline::IMarker> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Timeline::IMarker = __cordl_object
            .invoke("CreateMarker", (_cordl_type, _cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn CreateMarker_f64_1<T>(
        &mut self,
        _cordl_time: f64,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("CreateMarker", (_cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn CreateMixerPlayableGraph(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        go: *mut crate::UnityEngine::GameObject,
        tree: *mut crate::UnityEngine::Timeline::IntervalTree_1<
            *mut crate::UnityEngine::Timeline::RuntimeElement,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::Playable = __cordl_object
            .invoke("CreateMixerPlayableGraph", (graph, go, tree))?;
        Ok(__cordl_ret)
    }
    pub fn CreateNewClipContainerInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Timeline::TimelineClip> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Timeline::TimelineClip = __cordl_object
            .invoke("CreateNewClipContainerInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateNotificationsPlayable(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        mixerPlayable: crate::UnityEngine::Playables::Playable,
        go: *mut crate::UnityEngine::GameObject,
        timelinePlayable: crate::UnityEngine::Playables::Playable,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::Playable = __cordl_object
            .invoke(
                "CreateNotificationsPlayable",
                (graph, mixerPlayable, go, timelinePlayable),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CreatePlayableGraph(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        go: *mut crate::UnityEngine::GameObject,
        tree: *mut crate::UnityEngine::Timeline::IntervalTree_1<
            *mut crate::UnityEngine::Timeline::RuntimeElement,
        >,
        timelinePlayable: crate::UnityEngine::Playables::Playable,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::Playable = __cordl_object
            .invoke("CreatePlayableGraph", (graph, go, tree, timelinePlayable))?;
        Ok(__cordl_ret)
    }
    pub fn CreatePlayable_PlayableGraph_GameObject0(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        go: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::Playable = __cordl_object
            .invoke("CreatePlayable", (graph, go))?;
        Ok(__cordl_ret)
    }
    pub fn CreatePlayable_TimelineClip1(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        gameObject: *mut crate::UnityEngine::GameObject,
        clip: *mut crate::UnityEngine::Timeline::TimelineClip,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::Playable = __cordl_object
            .invoke("CreatePlayable", (graph, gameObject, clip))?;
        Ok(__cordl_ret)
    }
    pub fn CreateTrackMixer(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        go: *mut crate::UnityEngine::GameObject,
        inputCount: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::Playable = __cordl_object
            .invoke("CreateTrackMixer", (graph, go, inputCount))?;
        Ok(__cordl_ret)
    }
    pub fn DeleteClip(
        &mut self,
        clip: *mut crate::UnityEngine::Timeline::TimelineClip,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("DeleteClip", (clip))?;
        Ok(__cordl_ret)
    }
    pub fn DeleteMarker(
        &mut self,
        marker: *mut crate::UnityEngine::Timeline::IMarker,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("DeleteMarker", (marker))?;
        Ok(__cordl_ret)
    }
    pub fn DeleteMarkerRaw(
        &mut self,
        marker: *mut crate::UnityEngine::ScriptableObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("DeleteMarkerRaw", (marker))?;
        Ok(__cordl_ret)
    }
    pub fn GatherCompilableTracks(
        &mut self,
        tracks: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::Timeline::TrackAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GatherCompilableTracks", (tracks))?;
        Ok(__cordl_ret)
    }
    pub fn GatherNotifications(
        &mut self,
        markers: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::Timeline::IMarker,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GatherNotifications", (markers))?;
        Ok(__cordl_ret)
    }
    pub fn GatherProperties(
        &mut self,
        director: *mut crate::UnityEngine::Playables::PlayableDirector,
        driver: *mut crate::UnityEngine::Timeline::IPropertyCollector,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GatherProperties", (director, driver))?;
        Ok(__cordl_ret)
    }
    pub fn GetChildTracks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::Timeline::TrackAsset,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::Timeline::TrackAsset,
        > = __cordl_object.invoke("GetChildTracks", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetClips(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::Timeline::TimelineClip,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::Timeline::TimelineClip,
        > = __cordl_object.invoke("GetClips", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetClipsHash(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetClipsHash", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetEvaluationTime(
        &mut self,
        outStart: quest_hook::libil2cpp::ByRefMut<f64>,
        outDuration: quest_hook::libil2cpp::ByRefMut<f64>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetEvaluationTime", (outStart, outDuration))?;
        Ok(__cordl_ret)
    }
    pub fn GetGameObjectBinding(
        &mut self,
        director: *mut crate::UnityEngine::Playables::PlayableDirector,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("GetGameObjectBinding", (director))?;
        Ok(__cordl_ret)
    }
    pub fn GetMarker(
        &mut self,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Timeline::IMarker> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Timeline::IMarker = __cordl_object
            .invoke("GetMarker", (idx))?;
        Ok(__cordl_ret)
    }
    pub fn GetMarkerCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetMarkerCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetMarkers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::Timeline::IMarker,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::Timeline::IMarker,
        > = __cordl_object.invoke("GetMarkers", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetMarkersRaw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::ScriptableObject,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::ScriptableObject,
        > = __cordl_object.invoke("GetMarkersRaw", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetNotificationDuration(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("GetNotificationDuration", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSequenceTime(
        &mut self,
        outStart: quest_hook::libil2cpp::ByRefMut<f64>,
        outDuration: quest_hook::libil2cpp::ByRefMut<f64>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetSequenceTime", (outStart, outDuration))?;
        Ok(__cordl_ret)
    }
    pub fn GetTimeRangeHash(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetTimeRangeHash", ())?;
        Ok(__cordl_ret)
    }
    pub fn HasNotifications(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasNotifications", ())?;
        Ok(__cordl_ret)
    }
    pub fn Hash(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Hash", ())?;
        Ok(__cordl_ret)
    }
    pub fn Invalidate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invalidate", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsCompilable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsCompilable", ())?;
        Ok(__cordl_ret)
    }
    pub fn MoveLastTrackBefore(
        &mut self,
        asset: *mut crate::UnityEngine::Timeline::TrackAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveLastTrackBefore", (asset))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnAfterTrackDeserialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnAfterTrackDeserialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnBeforeTrackSerialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnBeforeTrackSerialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnClipMove(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnClipMove", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnCreateClip(
        &mut self,
        clip: *mut crate::UnityEngine::Timeline::TimelineClip,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCreateClip", (clip))?;
        Ok(__cordl_ret)
    }
    pub fn OnUpgradeFromVersion(
        &mut self,
        oldVersion: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnUpgradeFromVersion", (oldVersion))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveClip(
        &mut self,
        clip: *mut crate::UnityEngine::Timeline::TimelineClip,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveClip", (clip))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveSubTrack(
        &mut self,
        child: *mut crate::UnityEngine::Timeline::TrackAsset,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("RemoveSubTrack", (child))?;
        Ok(__cordl_ret)
    }
    pub fn SortClips(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SortClips", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_ISerializationCallbackReceiver_OnBeforeSerialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.ISerializationCallbackReceiver.OnBeforeSerialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_Timeline_ICurvesOwner_get_asset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Object = __cordl_object
            .invoke("UnityEngine.Timeline.ICurvesOwner.get_asset", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_Timeline_ICurvesOwner_get_assetOwner(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Object = __cordl_object
            .invoke("UnityEngine.Timeline.ICurvesOwner.get_assetOwner", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_Timeline_ICurvesOwner_get_defaultCurvesName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("UnityEngine.Timeline.ICurvesOwner.get_defaultCurvesName", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_Timeline_ICurvesOwner_get_targetTrack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Timeline::TrackAsset> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Timeline::TrackAsset = __cordl_object
            .invoke("UnityEngine.Timeline.ICurvesOwner.get_targetTrack", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateChildTrackCache(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateChildTrackCache", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateDuration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateDuration", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpgradeToLatestVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpgradeToLatestVersion", ())?;
        Ok(__cordl_ret)
    }
    pub fn ValidateClipType(
        &mut self,
        clipType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ValidateClipType", (clipType))?;
        Ok(__cordl_ret)
    }
    pub fn __internalAwake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("__internalAwake", ())?;
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
    pub fn get_clips(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Timeline::TimelineClip,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Timeline::TimelineClip,
        > = __cordl_object.invoke("get_clips", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_curves(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AnimationClip> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AnimationClip = __cordl_object
            .invoke("get_curves", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_customPlayableTypename(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_customPlayableTypename", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_duration(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_duration", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_end(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_end", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasClips(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasClips", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasCurves(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasCurves", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isEmpty", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isSubTrack(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isSubTrack", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_locked(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_locked", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lockedInHierarchy(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_lockedInHierarchy", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_muted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_muted", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_mutedInHierarchy(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_mutedInHierarchy", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_outputs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::UnityEngine::Playables::PlayableBinding,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::UnityEngine::Playables::PlayableBinding,
        > = __cordl_object.invoke("get_outputs", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_parent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::Playables::PlayableAsset,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Playables::PlayableAsset = __cordl_object
            .invoke("get_parent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_start(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_start", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_subTracksObjects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::ScriptableObject,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::ScriptableObject,
        > = __cordl_object.invoke("get_subTracksObjects", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_supportsNotifications(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_supportsNotifications", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_timelineAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::Timeline::TimelineAsset,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Timeline::TimelineAsset = __cordl_object
            .invoke("get_timelineAsset", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_curves(
        &mut self,
        value: *mut crate::UnityEngine::AnimationClip,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_curves", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_customPlayableTypename(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_customPlayableTypename", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_locked(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_locked", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_muted(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_muted", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_parent(
        &mut self,
        value: *mut crate::UnityEngine::Playables::PlayableAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_parent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Timeline::TrackAsset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset+TrackAssetUpgrade")]
#[repr(C)]
#[derive(Debug)]
pub struct TrackAsset_TrackAssetUpgrade {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset+TrackAssetUpgrade")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Timeline::TrackAsset_TrackAssetUpgrade => "UnityEngine.Timeline"
    ."TrackAsset/TrackAssetUpgrade"
);
#[cfg(feature = "UnityEngine+Timeline+TrackAsset+TrackAssetUpgrade")]
impl std::ops::Deref for crate::UnityEngine::Timeline::TrackAsset_TrackAssetUpgrade {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset+TrackAssetUpgrade")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::TrackAsset_TrackAssetUpgrade {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset+TrackAssetUpgrade")]
impl crate::UnityEngine::Timeline::TrackAsset_TrackAssetUpgrade {}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset+TrackAssetUpgrade")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::TrackAsset_TrackAssetUpgrade {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset+TransientBuildData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TrackAsset_TransientBuildData {
    pub trackList: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::Timeline::TrackAsset,
    >,
    pub clipList: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::Timeline::TimelineClip,
    >,
    pub markerList: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::Timeline::IMarker,
    >,
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset+TransientBuildData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Timeline::TrackAsset_TransientBuildData => "UnityEngine.Timeline"
    ."TrackAsset/TransientBuildData"
);
#[cfg(feature = "UnityEngine+Timeline+TrackAsset+TransientBuildData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Timeline::TrackAsset_TransientBuildData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset+TransientBuildData")]
impl crate::UnityEngine::Timeline::TrackAsset_TransientBuildData {
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Clear",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset+Versions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackAsset_Versions {
    AnimatedTrackProperties = 3i32,
    Initial = 0i32,
    RootMotionUpgrade = 2i32,
    RotationAsEuler = 1i32,
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset+Versions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::TrackAsset_Versions =>
    "UnityEngine.Timeline"."TrackAsset/Versions"
);