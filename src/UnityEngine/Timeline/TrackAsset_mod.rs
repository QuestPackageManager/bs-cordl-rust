#[cfg(feature = "UnityEngine+Timeline+TrackAsset")]
#[repr(C)]
#[derive(Debug)]
pub struct TrackAsset {
    __cordl_parent: crate::UnityEngine::Playables::PlayableAsset,
    pub m_Version: i32,
    pub m_AnimClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
    pub m_Locked: bool,
    pub m_Muted: bool,
    pub m_CustomPlayableFullTypename: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub m_Curves: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
    pub m_Parent: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Playables::PlayableAsset,
    >,
    pub m_Children: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
        >,
    >,
    pub m_ItemsHash: i32,
    pub m_ClipsCache: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
        >,
    >,
    pub m_Start: crate::UnityEngine::Timeline::DiscreteTime,
    pub m_End: crate::UnityEngine::Timeline::DiscreteTime,
    pub m_CacheSorted: bool,
    pub m_SupportsNotifications: crate::System::Nullable_1<bool>,
    pub m_ChildTrackCache: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IEnumerable_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
        >,
    >,
    pub m_Clips: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
        >,
    >,
    pub m_Markers: crate::UnityEngine::Timeline::MarkerList,
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Timeline::TrackAsset {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "TrackAsset";
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
    #[cfg(feature = "UnityEngine+Timeline+TrackAsset+TrackAssetUpgrade")]
    pub type TrackAssetUpgrade = crate::UnityEngine::Timeline::TrackAsset_TrackAssetUpgrade;
    #[cfg(feature = "UnityEngine+Timeline+TrackAsset+TransientBuildData")]
    pub type TransientBuildData = crate::UnityEngine::Timeline::TrackAsset_TransientBuildData;
    #[cfg(feature = "UnityEngine+Timeline+TrackAsset+Versions")]
    pub type Versions = crate::UnityEngine::Timeline::TrackAsset_Versions;
    pub fn AddChild(
        &mut self,
        child: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddChild", (child))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddClip(
        &mut self,
        newClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddClip", (newClip))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddMarker(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddMarker", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateItemsHash(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CalculateItemsHash", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CanCompileClips(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanCompileClips", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CanCompileNotifications(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanCompileNotifications", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CanCreateMixerRecursive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanCreateMixerRecursive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CanCreateTrackMixer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanCreateTrackMixer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearClipsInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearClipsInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearMarkers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearMarkers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearSubTracksInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearSubTracksInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileClips(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        timelineClips: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
            >,
        >,
        tree: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::IntervalTree_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::RuntimeElement>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::Playable = __cordl_object
            .invoke("CompileClips", (graph, go, timelineClips, tree))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConfigureTrackAnimation(
        &mut self,
        tree: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::IntervalTree_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::RuntimeElement>,
            >,
        >,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        blend: crate::UnityEngine::Playables::Playable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureTrackAnimation", (tree, go, blend))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateAndAddNewClipOfType(
        &mut self,
        requestedType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TimelineClip,
        > = __cordl_object.invoke("CreateAndAddNewClipOfType", (requestedType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateClipFromAsset(
        &mut self,
        playableAsset: quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TimelineClip,
        > = __cordl_object.invoke("CreateClipFromAsset", (playableAsset))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateClipFromPlayableAsset(
        &mut self,
        asset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Playables::IPlayableAsset>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TimelineClip,
        > = __cordl_object.invoke("CreateClipFromPlayableAsset", (asset))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateClipOfType(
        &mut self,
        requestedType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TimelineClip,
        > = __cordl_object.invoke("CreateClipOfType", (requestedType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateClip_0<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TimelineClip,
        > = __cordl_object.invoke("CreateClip", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateClip_Type1(
        &mut self,
        requestedType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TimelineClip,
        > = __cordl_object.invoke("CreateClip", (requestedType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCurves(
        &mut self,
        curvesClipName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateCurves", (curvesClipName))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDefaultClip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TimelineClip,
        > = __cordl_object.invoke("CreateDefaultClip", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateMarker_Type_f64_0(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        _cordl_time: f64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::IMarker>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::IMarker,
        > = __cordl_object.invoke("CreateMarker", (_cordl_type, _cordl_time))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn CreateMixerPlayableGraph(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        tree: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::IntervalTree_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::RuntimeElement>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::Playable = __cordl_object
            .invoke("CreateMixerPlayableGraph", (graph, go, tree))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateNewClipContainerInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TimelineClip,
        > = __cordl_object.invoke("CreateNewClipContainerInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateNotificationsPlayable(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        mixerPlayable: crate::UnityEngine::Playables::Playable,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
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
        Ok(__cordl_ret.into())
    }
    pub fn CreatePlayableGraph(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        tree: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::IntervalTree_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::RuntimeElement>,
            >,
        >,
        timelinePlayable: crate::UnityEngine::Playables::Playable,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::Playable = __cordl_object
            .invoke("CreatePlayableGraph", (graph, go, tree, timelinePlayable))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreatePlayable_PlayableGraph_GameObject0(
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
    pub fn CreatePlayable_TimelineClip1(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::Playable = __cordl_object
            .invoke("CreatePlayable", (graph, gameObject, clip))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateTrackMixer(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        inputCount: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::Playable = __cordl_object
            .invoke("CreateTrackMixer", (graph, go, inputCount))?;
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
    pub fn DeleteMarker(
        &mut self,
        marker: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::IMarker>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("DeleteMarker", (marker))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteMarkerRaw(
        &mut self,
        marker: quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("DeleteMarkerRaw", (marker))?;
        Ok(__cordl_ret.into())
    }
    pub fn GatherCompilableTracks(
        &mut self,
        tracks: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GatherCompilableTracks", (tracks))?;
        Ok(__cordl_ret.into())
    }
    pub fn GatherNotifications(
        &mut self,
        markers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::IMarker>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GatherNotifications", (markers))?;
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
    pub fn GetAnimationClipHash(
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAnimationClipHash", (clip))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetChildTracks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
            >,
        > = __cordl_object.invoke("GetChildTracks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetClips(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
            >,
        > = __cordl_object.invoke("GetClips", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetClipsHash(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetClipsHash", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn GetGameObjectBinding(
        &mut self,
        director: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Playables::PlayableDirector,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke("GetGameObjectBinding", (director))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMarker(
        &mut self,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::IMarker>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::IMarker,
        > = __cordl_object.invoke("GetMarker", (idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMarkerCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetMarkerCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMarkers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::IMarker>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::IMarker>,
            >,
        > = __cordl_object.invoke("GetMarkers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMarkersRaw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
            >,
        > = __cordl_object.invoke("GetMarkersRaw", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNotificationDuration(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("GetNotificationDuration", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn GetTimeRangeHash(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetTimeRangeHash", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasNotifications(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasNotifications", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Hash(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Hash", ())?;
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
    pub fn IsCompilable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsCompilable", ())?;
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
    pub fn OnAfterTrackDeserialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnAfterTrackDeserialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnBeforeTrackSerialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnBeforeTrackSerialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnClipMove(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnClipMove", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnCreateClip(
        &mut self,
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCreateClip", (clip))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn RemoveClip(
        &mut self,
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveClip", (clip))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveSubTrack(
        &mut self,
        child: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("RemoveSubTrack", (child))?;
        Ok(__cordl_ret.into())
    }
    pub fn SortClips(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SortClips", ())?;
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
    pub fn UnityEngine_Timeline_ICurvesOwner_get_asset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = __cordl_object
            .invoke("UnityEngine.Timeline.ICurvesOwner.get_asset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_Timeline_ICurvesOwner_get_assetOwner(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = __cordl_object
            .invoke("UnityEngine.Timeline.ICurvesOwner.get_assetOwner", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_Timeline_ICurvesOwner_get_defaultCurvesName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object
            .invoke("UnityEngine.Timeline.ICurvesOwner.get_defaultCurvesName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_Timeline_ICurvesOwner_get_targetTrack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TrackAsset,
        > = __cordl_object
            .invoke("UnityEngine.Timeline.ICurvesOwner.get_targetTrack", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateChildTrackCache(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateChildTrackCache", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateDuration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateDuration", ())?;
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
    pub fn ValidateClipType(
        &mut self,
        clipType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ValidateClipType", (clipType))?;
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
    pub fn add_OnClipPlayableCreate(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                crate::UnityEngine::Playables::Playable,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_OnClipPlayableCreate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_OnTrackAnimationPlayableCreate(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                crate::UnityEngine::Playables::Playable,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_OnTrackAnimationPlayableCreate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_clips(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
            >,
        > = __cordl_object.invoke("get_clips", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_curves(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip> = __cordl_object
            .invoke("get_curves", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_customPlayableTypename(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_customPlayableTypename", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_duration(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_duration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_end(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_end", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasClips(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasClips", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasCurves(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasCurves", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isEmpty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isSubTrack(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isSubTrack", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_locked(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_locked", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lockedInHierarchy(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_lockedInHierarchy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_muted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_muted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mutedInHierarchy(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_mutedInHierarchy", ())?;
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
    pub fn get_parent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Playables::PlayableAsset>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Playables::PlayableAsset,
        > = __cordl_object.invoke("get_parent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_start(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_subTracksObjects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
            >,
        > = __cordl_object.invoke("get_subTracksObjects", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_supportsNotifications(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_supportsNotifications", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_timelineAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineAsset>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TimelineAsset,
        > = __cordl_object.invoke("get_timelineAsset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_OnClipPlayableCreate(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                crate::UnityEngine::Playables::Playable,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_OnClipPlayableCreate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_OnTrackAnimationPlayableCreate(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                crate::UnityEngine::Playables::Playable,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_OnTrackAnimationPlayableCreate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_curves(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_curves", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_customPlayableTypename(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_customPlayableTypename", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_parent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Playables::PlayableAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_parent", (value))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "UnityEngine+Timeline+TrackAsset")]
impl AsRef<crate::UnityEngine::ISerializationCallbackReceiver>
for crate::UnityEngine::Timeline::TrackAsset {
    fn as_ref(&self) -> &crate::UnityEngine::ISerializationCallbackReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset")]
impl AsMut<crate::UnityEngine::ISerializationCallbackReceiver>
for crate::UnityEngine::Timeline::TrackAsset {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::ISerializationCallbackReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset")]
impl AsRef<crate::UnityEngine::Timeline::ICurvesOwner>
for crate::UnityEngine::Timeline::TrackAsset {
    fn as_ref(&self) -> &crate::UnityEngine::Timeline::ICurvesOwner {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset")]
impl AsMut<crate::UnityEngine::Timeline::ICurvesOwner>
for crate::UnityEngine::Timeline::TrackAsset {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Timeline::ICurvesOwner {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset")]
impl AsRef<crate::UnityEngine::Timeline::IPropertyPreview>
for crate::UnityEngine::Timeline::TrackAsset {
    fn as_ref(&self) -> &crate::UnityEngine::Timeline::IPropertyPreview {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset")]
impl AsMut<crate::UnityEngine::Timeline::IPropertyPreview>
for crate::UnityEngine::Timeline::TrackAsset {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Timeline::IPropertyPreview {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset+TrackAssetUpgrade")]
#[repr(C)]
#[derive(Debug)]
pub struct TrackAsset_TrackAssetUpgrade {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset+TrackAssetUpgrade")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Timeline::TrackAsset_TrackAssetUpgrade {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "TrackAsset/TrackAssetUpgrade";
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
#[cfg(feature = "UnityEngine+Timeline+TrackAsset+TrackAssetUpgrade")]
impl std::ops::Deref for crate::UnityEngine::Timeline::TrackAsset_TrackAssetUpgrade {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrackAsset_TransientBuildData {
    pub trackList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
        >,
    >,
    pub clipList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
        >,
    >,
    pub markerList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::IMarker>,
        >,
    >,
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset+TransientBuildData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Timeline::TrackAsset_TransientBuildData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "TrackAsset/TransientBuildData";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset+TransientBuildData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Timeline::TrackAsset_TransientBuildData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset+TransientBuildData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Timeline::TrackAsset_TransientBuildData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset+TransientBuildData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Timeline::TrackAsset_TransientBuildData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset+TransientBuildData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Timeline::TrackAsset_TransientBuildData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
        Ok(__cordl_ret.into())
    }
    pub fn Create() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Timeline::TrackAsset_TransientBuildData,
    > {
        let __cordl_ret: crate::UnityEngine::Timeline::TrackAsset_TransientBuildData = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset+Versions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TrackAsset_Versions {
    #[default]
    AnimatedTrackProperties = 3i32,
    Initial = 0i32,
    RootMotionUpgrade = 2i32,
    RotationAsEuler = 1i32,
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset+Versions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Timeline::TrackAsset_Versions {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "TrackAsset/Versions";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset+Versions")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Timeline::TrackAsset_Versions {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset+Versions")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Timeline::TrackAsset_Versions {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset+Versions")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Timeline::TrackAsset_Versions {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAsset+Versions")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Timeline::TrackAsset_Versions {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
