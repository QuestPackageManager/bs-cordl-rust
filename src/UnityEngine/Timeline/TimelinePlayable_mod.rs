#[cfg(feature = "cordl_class_UnityEngine+Timeline+TimelinePlayable")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct TimelinePlayable {
    __cordl_parent: crate::UnityEngine::Playables::PlayableBehaviour,
    pub m_IntervalTree: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Timeline::IntervalTree_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::RuntimeElement>,
        >,
    >,
    pub m_ActiveClips: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::RuntimeElement>,
        >,
    >,
    pub m_CurrentListOfActiveClips: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::RuntimeElement>,
        >,
    >,
    pub m_ActiveBit: i32,
    pub m_EvaluateCallbacks: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::ITimelineEvaluateCallback>,
        >,
    >,
    pub m_PlayableCache: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
            crate::UnityEngine::Playables::Playable,
        >,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Timeline+TimelinePlayable")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Timeline::TimelinePlayable {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "TimelinePlayable";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+Timeline+TimelinePlayable")]
impl std::ops::Deref for crate::UnityEngine::Timeline::TimelinePlayable {
    type Target = crate::UnityEngine::Playables::PlayableBehaviour;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelinePlayable")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::TimelinePlayable {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
                        crate::UnityEngine::Playables::Playable,
                        i32,
                        crate::UnityEngine::Playables::Playable,
                    ), quest_hook::libil2cpp::Void, 4usize>("CacheTrack")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CacheTrack",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (track, playable, port, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Compile(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        timelinePlayable: crate::UnityEngine::Playables::Playable,
        tracks: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
            >,
        >,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        autoRebalance: bool,
        createOutputs: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Playables::PlayableGraph,
                        crate::UnityEngine::Playables::Playable,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                        bool,
                        bool,
                    ), quest_hook::libil2cpp::Void, 6usize>("Compile")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Compile",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    graph,
                    timelinePlayable,
                    tracks,
                    go,
                    autoRebalance,
                    createOutputs,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileTrackList(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        timelinePlayable: crate::UnityEngine::Playables::Playable,
        tracks: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
            >,
        >,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        createOutputs: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Playables::PlayableGraph,
                        crate::UnityEngine::Playables::Playable,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                        bool,
                    ), quest_hook::libil2cpp::Void, 5usize>("CompileTrackList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CompileTrackList",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (graph, timelinePlayable, tracks, go, createOutputs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Create(
        graph: crate::UnityEngine::Playables::PlayableGraph,
        tracks: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
            >,
        >,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        autoRebalance: bool,
        createOutputs: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Playables::ScriptPlayable_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelinePlayable>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Playables::PlayableGraph,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                        bool,
                        bool,
                    ), crate::UnityEngine::Playables::ScriptPlayable_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelinePlayable>,
                    >, 5usize>("Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Create",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Playables::ScriptPlayable_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelinePlayable>,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (graph, tracks, go, autoRebalance, createOutputs))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Playables::PlayableGraph,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                        crate::UnityEngine::Playables::Playable,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("CreateTrackOutput")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateTrackOutput",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (graph, track, go, playable, port))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Playables::PlayableGraph,
                        crate::UnityEngine::Playables::Playable,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                        bool,
                    ), crate::UnityEngine::Playables::Playable, 5usize>(
                        "CreateTrackPlayable"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateTrackPlayable",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Playables::Playable = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (graph, timelinePlayable, track, go, createOutputs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Evaluate(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
        frameData: crate::UnityEngine::Playables::FrameData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Playables::Playable,
                        crate::UnityEngine::Playables::FrameData,
                    ), quest_hook::libil2cpp::Void, 2usize>("Evaluate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Evaluate",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (playable, frameData))? };
        Ok(__cordl_ret.into())
    }
    pub fn EvaluateAnimationPreviewUpdateCallback(
        &mut self,
        track: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
        animOutput: crate::UnityEngine::Animations::AnimationPlayableOutput,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
                        crate::UnityEngine::Animations::AnimationPlayableOutput,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "EvaluateAnimationPreviewUpdateCallback",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EvaluateAnimationPreviewUpdateCallback",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (track, animOutput))? };
        Ok(__cordl_ret.into())
    }
    pub fn EvaluateWeightsForAnimationPlayableOutput(
        &mut self,
        track: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
        animOutput: crate::UnityEngine::Animations::AnimationPlayableOutput,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
                        crate::UnityEngine::Animations::AnimationPlayableOutput,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "EvaluateWeightsForAnimationPlayableOutput",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EvaluateWeightsForAnimationPlayableOutput",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (track, animOutput))? };
        Ok(__cordl_ret.into())
    }
    pub fn ForAOTCompilationOnly() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "ForAOTCompilationOnly",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ForAOTCompilationOnly",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PrepareFrame(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
        info: crate::UnityEngine::Playables::FrameData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Playables::Playable,
                        crate::UnityEngine::Playables::FrameData,
                    ), quest_hook::libil2cpp::Void, 2usize>("PrepareFrame")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PrepareFrame",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (playable, info))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Timeline+TimelinePlayable")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Timeline::TimelinePlayable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
