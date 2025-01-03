#[cfg(feature = "UnityEngine+Timeline+TimelinePlayable")]
#[repr(C)]
#[derive(Debug)]
pub struct TimelinePlayable {
    __cordl_parent: crate::UnityEngine::Playables::PlayableBehaviour,
    pub m_IntervalTree: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Timeline::IntervalTree_1<
            *mut crate::UnityEngine::Timeline::RuntimeElement,
        >,
    >,
    pub m_ActiveClips: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::Timeline::RuntimeElement,
        >,
    >,
    pub m_CurrentListOfActiveClips: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::Timeline::RuntimeElement,
        >,
    >,
    pub m_ActiveBit: i32,
    pub m_EvaluateCallbacks: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::Timeline::ITimelineEvaluateCallback,
        >,
    >,
    pub m_PlayableCache: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            *mut crate::UnityEngine::Timeline::TrackAsset,
            crate::UnityEngine::Playables::Playable,
        >,
    >,
}
#[cfg(feature = "UnityEngine+Timeline+TimelinePlayable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::TimelinePlayable =>
    "UnityEngine.Timeline"."TimelinePlayable"
);
#[cfg(feature = "UnityEngine+Timeline+TimelinePlayable")]
impl std::ops::Deref for crate::UnityEngine::Timeline::TimelinePlayable {
    type Target = crate::UnityEngine::Playables::PlayableBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelinePlayable")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::TimelinePlayable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelinePlayable")]
impl crate::UnityEngine::Timeline::TimelinePlayable {
    pub fn CacheTrack(
        &mut self,
        track: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
        playable: crate::UnityEngine::Playables::Playable,
        port: i32,
        parent: crate::UnityEngine::Playables::Playable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CacheTrack", (track, playable, port, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Compile(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        timelinePlayable: crate::UnityEngine::Playables::Playable,
        tracks: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::Timeline::TrackAsset,
            >,
        >,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        autoRebalance: bool,
        createOutputs: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Compile",
                (graph, timelinePlayable, tracks, go, autoRebalance, createOutputs),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileTrackList(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        timelinePlayable: crate::UnityEngine::Playables::Playable,
        tracks: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::Timeline::TrackAsset,
            >,
        >,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        createOutputs: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CompileTrackList",
                (graph, timelinePlayable, tracks, go, createOutputs),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Create(
        graph: crate::UnityEngine::Playables::PlayableGraph,
        tracks: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::Timeline::TrackAsset,
            >,
        >,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        autoRebalance: bool,
        createOutputs: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Playables::ScriptPlayable_1<
            *mut crate::UnityEngine::Timeline::TimelinePlayable,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::Playables::ScriptPlayable_1<
            *mut crate::UnityEngine::Timeline::TimelinePlayable,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (graph, tracks, go, autoRebalance, createOutputs))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateTrackOutput(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        track: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        playable: crate::UnityEngine::Playables::Playable,
        port: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateTrackOutput", (graph, track, go, playable, port))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateTrackPlayable(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        timelinePlayable: crate::UnityEngine::Playables::Playable,
        track: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        createOutputs: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::Playable = __cordl_object
            .invoke(
                "CreateTrackPlayable",
                (graph, timelinePlayable, track, go, createOutputs),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Evaluate(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
        frameData: crate::UnityEngine::Playables::FrameData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Evaluate", (playable, frameData))?;
        Ok(__cordl_ret.into())
    }
    pub fn EvaluateAnimationPreviewUpdateCallback(
        &mut self,
        track: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
        animOutput: crate::UnityEngine::Animations::AnimationPlayableOutput,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EvaluateAnimationPreviewUpdateCallback", (track, animOutput))?;
        Ok(__cordl_ret.into())
    }
    pub fn EvaluateWeightsForAnimationPlayableOutput(
        &mut self,
        track: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
        animOutput: crate::UnityEngine::Animations::AnimationPlayableOutput,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EvaluateWeightsForAnimationPlayableOutput", (track, animOutput))?;
        Ok(__cordl_ret.into())
    }
    pub fn ForAOTCompilationOnly() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ForAOTCompilationOnly", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PrepareFrame(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
        info: crate::UnityEngine::Playables::FrameData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PrepareFrame", (playable, info))?;
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
#[cfg(feature = "UnityEngine+Timeline+TimelinePlayable")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::TimelinePlayable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
