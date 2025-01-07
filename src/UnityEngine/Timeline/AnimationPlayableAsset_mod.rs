#[cfg(feature = "UnityEngine+Timeline+AnimationPlayableAsset")]
#[repr(C)]
#[derive(Debug)]
pub struct AnimationPlayableAsset {
    __cordl_parent: crate::UnityEngine::Playables::PlayableAsset,
    pub m_Clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
    pub m_Position: crate::UnityEngine::Vector3,
    pub m_EulerAngles: crate::UnityEngine::Vector3,
    pub m_UseTrackMatchFields: bool,
    pub m_MatchTargetFields: crate::UnityEngine::Timeline::MatchTargetFields,
    pub m_RemoveStartOffset: bool,
    pub m_ApplyFootIK: bool,
    pub m_Loop: crate::UnityEngine::Timeline::AnimationPlayableAsset_LoopMode,
    pub _appliedOffsetMode_k__BackingField: crate::UnityEngine::Timeline::AppliedOffsetMode,
    pub m_Version: i32,
    pub m_Rotation: crate::UnityEngine::Quaternion,
}
#[cfg(feature = "UnityEngine+Timeline+AnimationPlayableAsset")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Timeline::AnimationPlayableAsset {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "AnimationPlayableAsset";
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
#[cfg(feature = "UnityEngine+Timeline+AnimationPlayableAsset")]
impl std::ops::Deref for crate::UnityEngine::Timeline::AnimationPlayableAsset {
    type Target = crate::UnityEngine::Playables::PlayableAsset;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationPlayableAsset")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::AnimationPlayableAsset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationPlayableAsset")]
impl crate::UnityEngine::Timeline::AnimationPlayableAsset {
    #[cfg(
        feature = "UnityEngine+Timeline+AnimationPlayableAsset+AnimationPlayableAssetUpgrade"
    )]
    pub type AnimationPlayableAssetUpgrade = crate::UnityEngine::Timeline::AnimationPlayableAsset_AnimationPlayableAssetUpgrade;
    #[cfg(feature = "UnityEngine+Timeline+AnimationPlayableAsset+LoopMode")]
    pub type LoopMode = crate::UnityEngine::Timeline::AnimationPlayableAsset_LoopMode;
    #[cfg(feature = "UnityEngine+Timeline+AnimationPlayableAsset+Versions")]
    pub type Versions = crate::UnityEngine::Timeline::AnimationPlayableAsset_Versions;
    pub fn CreatePlayable_AnimationClip_Vector3_Vector3__cordl_bool_AppliedOffsetMode__cordl_bool_AnimationPlayableAsset_LoopMode1(
        graph: crate::UnityEngine::Playables::PlayableGraph,
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
        positionOffset: crate::UnityEngine::Vector3,
        eulerOffset: crate::UnityEngine::Vector3,
        removeStartOffset: bool,
        mode: crate::UnityEngine::Timeline::AppliedOffsetMode,
        applyFootIK: bool,
        _cordl_loop: crate::UnityEngine::Timeline::AnimationPlayableAsset_LoopMode,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_ret: crate::UnityEngine::Playables::Playable = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreatePlayable",
                (
                    graph,
                    clip,
                    positionOffset,
                    eulerOffset,
                    removeStartOffset,
                    mode,
                    applyFootIK,
                    _cordl_loop,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreatePlayable_GameObject0(
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
    pub fn HasRootTransforms(
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasRootTransforms", (clip))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn ShouldApplyOffset(
        mode: crate::UnityEngine::Timeline::AppliedOffsetMode,
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldApplyOffset", (mode, clip))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldApplyScaleRemove(
        mode: crate::UnityEngine::Timeline::AppliedOffsetMode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldApplyScaleRemove", (mode))?;
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
    pub fn get_appliedOffsetMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Timeline::AppliedOffsetMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Timeline::AppliedOffsetMode = __cordl_object
            .invoke("get_appliedOffsetMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_applyFootIK(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_applyFootIK", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_clip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip> = __cordl_object
            .invoke("get_clip", ())?;
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
    pub fn get_hasRootTransforms(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasRootTransforms", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_loop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Timeline::AnimationPlayableAsset_LoopMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Timeline::AnimationPlayableAsset_LoopMode = __cordl_object
            .invoke("get_loop", ())?;
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
    pub fn get_removeStartOffset(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_removeStartOffset", ())?;
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
    pub fn get_useTrackMatchFields(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_useTrackMatchFields", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_appliedOffsetMode(
        &mut self,
        value: crate::UnityEngine::Timeline::AppliedOffsetMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_appliedOffsetMode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_applyFootIK(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_applyFootIK", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_clip(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_clip", (value))?;
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
    pub fn set_loop(
        &mut self,
        value: crate::UnityEngine::Timeline::AnimationPlayableAsset_LoopMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_loop", (value))?;
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
    pub fn set_removeStartOffset(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_removeStartOffset", (value))?;
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
    pub fn set_useTrackMatchFields(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_useTrackMatchFields", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationPlayableAsset")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::AnimationPlayableAsset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationPlayableAsset")]
impl AsRef<crate::UnityEngine::ISerializationCallbackReceiver>
for crate::UnityEngine::Timeline::AnimationPlayableAsset {
    fn as_ref(&self) -> &crate::UnityEngine::ISerializationCallbackReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationPlayableAsset")]
impl AsMut<crate::UnityEngine::ISerializationCallbackReceiver>
for crate::UnityEngine::Timeline::AnimationPlayableAsset {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::ISerializationCallbackReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationPlayableAsset")]
impl AsRef<crate::UnityEngine::Timeline::IPropertyPreview>
for crate::UnityEngine::Timeline::AnimationPlayableAsset {
    fn as_ref(&self) -> &crate::UnityEngine::Timeline::IPropertyPreview {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationPlayableAsset")]
impl AsMut<crate::UnityEngine::Timeline::IPropertyPreview>
for crate::UnityEngine::Timeline::AnimationPlayableAsset {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Timeline::IPropertyPreview {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationPlayableAsset")]
impl AsRef<crate::UnityEngine::Timeline::ITimelineClipAsset>
for crate::UnityEngine::Timeline::AnimationPlayableAsset {
    fn as_ref(&self) -> &crate::UnityEngine::Timeline::ITimelineClipAsset {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationPlayableAsset")]
impl AsMut<crate::UnityEngine::Timeline::ITimelineClipAsset>
for crate::UnityEngine::Timeline::AnimationPlayableAsset {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Timeline::ITimelineClipAsset {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+Timeline+AnimationPlayableAsset+AnimationPlayableAssetUpgrade"
)]
#[repr(C)]
#[derive(Debug)]
pub struct AnimationPlayableAsset_AnimationPlayableAssetUpgrade {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "UnityEngine+Timeline+AnimationPlayableAsset+AnimationPlayableAssetUpgrade"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Timeline::AnimationPlayableAsset_AnimationPlayableAssetUpgrade {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "AnimationPlayableAssetUpgrade";
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
#[cfg(
    feature = "UnityEngine+Timeline+AnimationPlayableAsset+AnimationPlayableAssetUpgrade"
)]
impl std::ops::Deref
for crate::UnityEngine::Timeline::AnimationPlayableAsset_AnimationPlayableAssetUpgrade {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Timeline+AnimationPlayableAsset+AnimationPlayableAssetUpgrade"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Timeline::AnimationPlayableAsset_AnimationPlayableAssetUpgrade {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Timeline+AnimationPlayableAsset+AnimationPlayableAssetUpgrade"
)]
impl crate::UnityEngine::Timeline::AnimationPlayableAsset_AnimationPlayableAssetUpgrade {
    pub fn ConvertRotationToEuler(
        asset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::AnimationPlayableAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertRotationToEuler", (asset))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+Timeline+AnimationPlayableAsset+AnimationPlayableAssetUpgrade"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::AnimationPlayableAsset_AnimationPlayableAssetUpgrade {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationPlayableAsset+LoopMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimationPlayableAsset_LoopMode {
    #[default]
    Off = 2i32,
    On = 1i32,
    UseSourceAsset = 0i32,
}
#[cfg(feature = "UnityEngine+Timeline+AnimationPlayableAsset+LoopMode")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Timeline::AnimationPlayableAsset_LoopMode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "LoopMode";
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
#[cfg(feature = "UnityEngine+Timeline+AnimationPlayableAsset+LoopMode")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Timeline::AnimationPlayableAsset_LoopMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationPlayableAsset+LoopMode")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Timeline::AnimationPlayableAsset_LoopMode {
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
#[cfg(feature = "UnityEngine+Timeline+AnimationPlayableAsset+LoopMode")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Timeline::AnimationPlayableAsset_LoopMode {
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
#[cfg(feature = "UnityEngine+Timeline+AnimationPlayableAsset+LoopMode")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Timeline::AnimationPlayableAsset_LoopMode {
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
#[cfg(feature = "UnityEngine+Timeline+AnimationPlayableAsset+Versions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimationPlayableAsset_Versions {
    #[default]
    Initial = 0i32,
    RotationAsEuler = 1i32,
}
#[cfg(feature = "UnityEngine+Timeline+AnimationPlayableAsset+Versions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Timeline::AnimationPlayableAsset_Versions {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "Versions";
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
#[cfg(feature = "UnityEngine+Timeline+AnimationPlayableAsset+Versions")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Timeline::AnimationPlayableAsset_Versions {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationPlayableAsset+Versions")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Timeline::AnimationPlayableAsset_Versions {
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
#[cfg(feature = "UnityEngine+Timeline+AnimationPlayableAsset+Versions")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Timeline::AnimationPlayableAsset_Versions {
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
#[cfg(feature = "UnityEngine+Timeline+AnimationPlayableAsset+Versions")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Timeline::AnimationPlayableAsset_Versions {
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
