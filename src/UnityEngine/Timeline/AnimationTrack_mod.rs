#[cfg(feature = "UnityEngine+Timeline+AnimationTrack")]
#[repr(C)]
#[derive(Debug)]
pub struct AnimationTrack {
    __cordl_parent: crate::UnityEngine::Timeline::TrackAsset,
    pub m_InfiniteClipPreExtrapolation: crate::UnityEngine::Timeline::TimelineClip_ClipExtrapolation,
    pub m_InfiniteClipPostExtrapolation: crate::UnityEngine::Timeline::TimelineClip_ClipExtrapolation,
    pub m_InfiniteClipOffsetPosition: crate::UnityEngine::Vector3,
    pub m_InfiniteClipOffsetEulerAngles: crate::UnityEngine::Vector3,
    pub m_InfiniteClipTimeOffset: f64,
    pub m_InfiniteClipRemoveOffset: bool,
    pub m_InfiniteClipApplyFootIK: bool,
    pub mInfiniteClipLoop: crate::UnityEngine::Timeline::AnimationPlayableAsset_LoopMode,
    pub m_MatchTargetFields: crate::UnityEngine::Timeline::MatchTargetFields,
    pub m_Position: crate::UnityEngine::Vector3,
    pub m_EulerAngles: crate::UnityEngine::Vector3,
    pub m_AvatarMask: *mut crate::UnityEngine::AvatarMask,
    pub m_ApplyAvatarMask: bool,
    pub m_TrackOffset: crate::UnityEngine::Timeline::TrackOffset,
    pub m_InfiniteClip: *mut crate::UnityEngine::AnimationClip,
    pub m_OpenClipOffsetRotation: crate::UnityEngine::Quaternion,
    pub m_Rotation: crate::UnityEngine::Quaternion,
    pub m_ApplyOffsets: bool,
}
#[cfg(feature = "UnityEngine+Timeline+AnimationTrack")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::AnimationTrack =>
    "UnityEngine.Timeline"."AnimationTrack"
);
#[cfg(feature = "UnityEngine+Timeline+AnimationTrack")]
impl std::ops::Deref for crate::UnityEngine::Timeline::AnimationTrack {
    type Target = crate::UnityEngine::Timeline::TrackAsset;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationTrack")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::AnimationTrack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationTrack")]
impl crate::UnityEngine::Timeline::AnimationTrack {
    pub const k_DefaultInfiniteClipName: &'static str = "Recorded";
    pub const k_DefaultRecordableClipName: &'static str = "Recorded";
    #[cfg(feature = "UnityEngine+Timeline+AnimationTrack+AnimationTrackUpgrade")]
    pub type AnimationTrackUpgrade = crate::UnityEngine::Timeline::AnimationTrack_AnimationTrackUpgrade;
    pub fn AnimatesRootTransform(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AnimatesRootTransform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyTrackOffset(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        root: crate::UnityEngine::Playables::Playable,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        mode: crate::UnityEngine::Timeline::AppliedOffsetMode,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::Playable = __cordl_object
            .invoke("ApplyTrackOffset", (graph, root, go, mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn AssignAnimationClip(
        &mut self,
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
        animClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AssignAnimationClip", (clip, animClip))?;
        Ok(__cordl_ret.into())
    }
    pub fn AttachDefaultBlend(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        mixer: crate::UnityEngine::Animations::AnimationLayerMixerPlayable,
        requireOffset: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AttachDefaultBlend", (graph, mixer, requireOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn AttachOffsetPlayable(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        playable: crate::UnityEngine::Playables::Playable,
        pos: crate::UnityEngine::Vector3,
        rot: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::Playable = __cordl_object
            .invoke("AttachOffsetPlayable", (graph, playable, pos, rot))?;
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
    pub fn CompileTrackPlayable(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        track: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::AnimationTrack>,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        tree: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::IntervalTree_1<
                *mut crate::UnityEngine::Timeline::RuntimeElement,
            >,
        >,
        mode: crate::UnityEngine::Timeline::AppliedOffsetMode,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::Playable = __cordl_object
            .invoke("CompileTrackPlayable", (graph, track, go, tree, mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateClip(
        &mut self,
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TimelineClip,
        > = __cordl_object.invoke("CreateClip", (clip))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateGroupMixer(
        graph: crate::UnityEngine::Playables::PlayableGraph,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        inputCount: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Animations::AnimationLayerMixerPlayable,
    > {
        let __cordl_ret: crate::UnityEngine::Animations::AnimationLayerMixerPlayable = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateGroupMixer", (graph, go, inputCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInfiniteClip(
        &mut self,
        infiniteClipName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateInfiniteClip", (infiniteClipName))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInfiniteTrackPlayable(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        tree: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::IntervalTree_1<
                *mut crate::UnityEngine::Timeline::RuntimeElement,
            >,
        >,
        mode: crate::UnityEngine::Timeline::AppliedOffsetMode,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::Playable = __cordl_object
            .invoke("CreateInfiniteTrackPlayable", (graph, go, tree, mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateMixerPlayableGraph(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        tree: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::IntervalTree_1<
                *mut crate::UnityEngine::Timeline::RuntimeElement,
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
    pub fn CreateRecordableClip(
        &mut self,
        animClipName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TimelineClip,
        > = __cordl_object.invoke("CreateRecordableClip", (animClipName))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindInHierarchyBreadthFirst(
        t: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindInHierarchyBreadthFirst", (t, name))?;
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
    pub fn GetAnimationClips(
        &mut self,
        animClips: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::AnimationClip,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetAnimationClips", (animClips))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBinding(
        &mut self,
        director: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Playables::PlayableDirector,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator> = __cordl_object
            .invoke("GetBinding", (director))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultBlendCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetDefaultBlendCount", ())?;
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
    pub fn GetGenericRootNode(
        &mut self,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform> = __cordl_object
            .invoke("GetGenericRootNode", (gameObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOffsetMode(
        &mut self,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        animatesRootTransform: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Timeline::AppliedOffsetMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Timeline::AppliedOffsetMode = __cordl_object
            .invoke("GetOffsetMode", (go, animatesRootTransform))?;
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
    pub fn HasController(
        &mut self,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasController", (gameObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsRootTransformDisabledByMask(
        &mut self,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        genericRootNode: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsRootTransformDisabledByMask", (gameObject, genericRootNode))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn RequiresMotionXPlayable(
        &mut self,
        mode: crate::UnityEngine::Timeline::AppliedOffsetMode,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("RequiresMotionXPlayable", (mode, gameObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetOffsets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetOffsets", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_Timeline_ILayerable_CreateLayerMixer(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        inputCount: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::Playable = __cordl_object
            .invoke(
                "UnityEngine.Timeline.ILayerable.CreateLayerMixer",
                (graph, go, inputCount),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateClipOffsets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateClipOffsets", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UsesAbsoluteMotion(
        mode: crate::UnityEngine::Timeline::AppliedOffsetMode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UsesAbsoluteMotion", (mode))?;
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
    pub fn get_applyAvatarMask(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_applyAvatarMask", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_applyOffsets(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_applyOffsets", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_avatarMask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AvatarMask>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AvatarMask> = __cordl_object
            .invoke("get_avatarMask", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_eulerAngles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_eulerAngles", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_inClipMode(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_inClipMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_infiniteClip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip> = __cordl_object
            .invoke("get_infiniteClip", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_infiniteClipApplyFootIK(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_infiniteClipApplyFootIK", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_infiniteClipLoop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Timeline::AnimationPlayableAsset_LoopMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Timeline::AnimationPlayableAsset_LoopMode = __cordl_object
            .invoke("get_infiniteClipLoop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_infiniteClipOffsetEulerAngles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_infiniteClipOffsetEulerAngles", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_infiniteClipOffsetPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_infiniteClipOffsetPosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_infiniteClipOffsetRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("get_infiniteClipOffsetRotation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_infiniteClipPostExtrapolation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Timeline::TimelineClip_ClipExtrapolation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Timeline::TimelineClip_ClipExtrapolation = __cordl_object
            .invoke("get_infiniteClipPostExtrapolation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_infiniteClipPreExtrapolation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Timeline::TimelineClip_ClipExtrapolation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Timeline::TimelineClip_ClipExtrapolation = __cordl_object
            .invoke("get_infiniteClipPreExtrapolation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_infiniteClipRemoveOffset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_infiniteClipRemoveOffset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_infiniteClipTimeOffset(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_infiniteClipTimeOffset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_matchTargetFields(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Timeline::MatchTargetFields> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Timeline::MatchTargetFields = __cordl_object
            .invoke("get_matchTargetFields", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_openClipOffsetEulerAngles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_openClipOffsetEulerAngles", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_openClipOffsetPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_openClipOffsetPosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_openClipOffsetRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("get_openClipOffsetRotation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_openClipPostExtrapolation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Timeline::TimelineClip_ClipExtrapolation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Timeline::TimelineClip_ClipExtrapolation = __cordl_object
            .invoke("get_openClipPostExtrapolation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_openClipPreExtrapolation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Timeline::TimelineClip_ClipExtrapolation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Timeline::TimelineClip_ClipExtrapolation = __cordl_object
            .invoke("get_openClipPreExtrapolation", ())?;
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
    pub fn get_position(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_position", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("get_rotation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_trackOffset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Timeline::TrackOffset> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Timeline::TrackOffset = __cordl_object
            .invoke("get_trackOffset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_applyAvatarMask(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_applyAvatarMask", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_applyOffsets(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_applyOffsets", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_avatarMask(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::AvatarMask>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_avatarMask", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_eulerAngles(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_eulerAngles", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_infiniteClip(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_infiniteClip", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_infiniteClipApplyFootIK(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_infiniteClipApplyFootIK", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_infiniteClipLoop(
        &mut self,
        value: crate::UnityEngine::Timeline::AnimationPlayableAsset_LoopMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_infiniteClipLoop", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_infiniteClipOffsetEulerAngles(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_infiniteClipOffsetEulerAngles", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_infiniteClipOffsetPosition(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_infiniteClipOffsetPosition", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_infiniteClipOffsetRotation(
        &mut self,
        value: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_infiniteClipOffsetRotation", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_infiniteClipPostExtrapolation(
        &mut self,
        value: crate::UnityEngine::Timeline::TimelineClip_ClipExtrapolation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_infiniteClipPostExtrapolation", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_infiniteClipPreExtrapolation(
        &mut self,
        value: crate::UnityEngine::Timeline::TimelineClip_ClipExtrapolation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_infiniteClipPreExtrapolation", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_infiniteClipRemoveOffset(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_infiniteClipRemoveOffset", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_infiniteClipTimeOffset(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_infiniteClipTimeOffset", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_matchTargetFields(
        &mut self,
        value: crate::UnityEngine::Timeline::MatchTargetFields,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_matchTargetFields", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_openClipOffsetEulerAngles(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_openClipOffsetEulerAngles", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_openClipOffsetPosition(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_openClipOffsetPosition", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_openClipOffsetRotation(
        &mut self,
        value: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_openClipOffsetRotation", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_openClipPostExtrapolation(
        &mut self,
        value: crate::UnityEngine::Timeline::TimelineClip_ClipExtrapolation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_openClipPostExtrapolation", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_openClipPreExtrapolation(
        &mut self,
        value: crate::UnityEngine::Timeline::TimelineClip_ClipExtrapolation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_openClipPreExtrapolation", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_position(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_position", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_rotation(
        &mut self,
        value: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rotation", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_trackOffset(
        &mut self,
        value: crate::UnityEngine::Timeline::TrackOffset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_trackOffset", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationTrack")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Timeline::AnimationTrack {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationTrack")]
impl AsRef<crate::UnityEngine::Timeline::ILayerable>
for crate::UnityEngine::Timeline::AnimationTrack {
    fn as_ref(&self) -> &crate::UnityEngine::Timeline::ILayerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationTrack")]
impl AsMut<crate::UnityEngine::Timeline::ILayerable>
for crate::UnityEngine::Timeline::AnimationTrack {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Timeline::ILayerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationTrack+AnimationTrackUpgrade")]
#[repr(C)]
#[derive(Debug)]
pub struct AnimationTrack_AnimationTrackUpgrade {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Timeline+AnimationTrack+AnimationTrackUpgrade")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Timeline::AnimationTrack_AnimationTrackUpgrade =>
    "UnityEngine.Timeline"."AnimationTrack/AnimationTrackUpgrade"
);
#[cfg(feature = "UnityEngine+Timeline+AnimationTrack+AnimationTrackUpgrade")]
impl std::ops::Deref
for crate::UnityEngine::Timeline::AnimationTrack_AnimationTrackUpgrade {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationTrack+AnimationTrackUpgrade")]
impl std::ops::DerefMut
for crate::UnityEngine::Timeline::AnimationTrack_AnimationTrackUpgrade {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationTrack+AnimationTrackUpgrade")]
impl crate::UnityEngine::Timeline::AnimationTrack_AnimationTrackUpgrade {
    pub fn ConvertInfiniteTrack(
        track: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::AnimationTrack>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertInfiniteTrack", (track))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertRootMotion(
        track: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::AnimationTrack>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertRootMotion", (track))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertRotationsToEuler(
        track: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::AnimationTrack>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertRotationsToEuler", (track))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationTrack+AnimationTrackUpgrade")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::AnimationTrack_AnimationTrackUpgrade {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
