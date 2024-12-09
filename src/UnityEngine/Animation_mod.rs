#[cfg(feature = "UnityEngine+Animation")]
#[repr(C)]
#[derive(Debug)]
pub struct Animation {
    __cordl_parent: crate::UnityEngine::Behaviour,
}
#[cfg(feature = "UnityEngine+Animation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Animation => "UnityEngine"
    ."Animation"
);
#[cfg(feature = "UnityEngine+Animation")]
impl std::ops::Deref for crate::UnityEngine::Animation {
    type Target = crate::UnityEngine::Behaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Animation")]
impl std::ops::DerefMut for crate::UnityEngine::Animation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Animation")]
impl crate::UnityEngine::Animation {
    #[cfg(feature = "UnityEngine+Animation+Enumerator")]
    pub type Enumerator = crate::UnityEngine::Animation_Enumerator;
    pub fn AddClip_AnimationClip_Il2CppString0(
        &mut self,
        clip: *mut crate::UnityEngine::AnimationClip,
        newName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddClip", (clip, newName))?;
        Ok(__cordl_ret)
    }
    pub fn AddClip_i32_i32_1(
        &mut self,
        clip: *mut crate::UnityEngine::AnimationClip,
        newName: *mut quest_hook::libil2cpp::Il2CppString,
        firstFrame: i32,
        lastFrame: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddClip", (clip, newName, firstFrame, lastFrame))?;
        Ok(__cordl_ret)
    }
    pub fn AddClip_i32_i32__cordl_bool2(
        &mut self,
        clip: *mut crate::UnityEngine::AnimationClip,
        newName: *mut quest_hook::libil2cpp::Il2CppString,
        firstFrame: i32,
        lastFrame: i32,
        addLoopFrame: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddClip", (clip, newName, firstFrame, lastFrame, addLoopFrame))?;
        Ok(__cordl_ret)
    }
    pub fn Blend_Il2CppString0(
        &mut self,
        animation: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Blend", (animation))?;
        Ok(__cordl_ret)
    }
    pub fn Blend_f32_1(
        &mut self,
        animation: *mut quest_hook::libil2cpp::Il2CppString,
        targetWeight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Blend", (animation, targetWeight))?;
        Ok(__cordl_ret)
    }
    pub fn Blend_f32_f32_2(
        &mut self,
        animation: *mut quest_hook::libil2cpp::Il2CppString,
        targetWeight: f32,
        fadeLength: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Blend", (animation, targetWeight, fadeLength))?;
        Ok(__cordl_ret)
    }
    pub fn CrossFadeQueued_Il2CppString0(
        &mut self,
        animation: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AnimationState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AnimationState = __cordl_object
            .invoke("CrossFadeQueued", (animation))?;
        Ok(__cordl_ret)
    }
    pub fn CrossFadeQueued_f32_1(
        &mut self,
        animation: *mut quest_hook::libil2cpp::Il2CppString,
        fadeLength: f32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AnimationState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AnimationState = __cordl_object
            .invoke("CrossFadeQueued", (animation, fadeLength))?;
        Ok(__cordl_ret)
    }
    pub fn CrossFadeQueued_f32_QueueMode2(
        &mut self,
        animation: *mut quest_hook::libil2cpp::Il2CppString,
        fadeLength: f32,
        queue: crate::UnityEngine::QueueMode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AnimationState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AnimationState = __cordl_object
            .invoke("CrossFadeQueued", (animation, fadeLength, queue))?;
        Ok(__cordl_ret)
    }
    pub fn CrossFadeQueued_f32_QueueMode_PlayMode3(
        &mut self,
        animation: *mut quest_hook::libil2cpp::Il2CppString,
        fadeLength: f32,
        queue: crate::UnityEngine::QueueMode,
        mode: crate::UnityEngine::PlayMode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AnimationState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AnimationState = __cordl_object
            .invoke("CrossFadeQueued", (animation, fadeLength, queue, mode))?;
        Ok(__cordl_ret)
    }
    pub fn CrossFade_Il2CppString0(
        &mut self,
        animation: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CrossFade", (animation))?;
        Ok(__cordl_ret)
    }
    pub fn CrossFade_f32_1(
        &mut self,
        animation: *mut quest_hook::libil2cpp::Il2CppString,
        fadeLength: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CrossFade", (animation, fadeLength))?;
        Ok(__cordl_ret)
    }
    pub fn CrossFade_f32_PlayMode2(
        &mut self,
        animation: *mut quest_hook::libil2cpp::Il2CppString,
        fadeLength: f32,
        mode: crate::UnityEngine::PlayMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CrossFade", (animation, fadeLength, mode))?;
        Ok(__cordl_ret)
    }
    pub fn GetClip(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AnimationClip> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AnimationClip = __cordl_object
            .invoke("GetClip", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetClipCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetClipCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetState(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AnimationState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AnimationState = __cordl_object
            .invoke("GetState", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetStateAtIndex(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AnimationState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AnimationState = __cordl_object
            .invoke("GetStateAtIndex", (index))?;
        Ok(__cordl_ret)
    }
    pub fn GetStateCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetStateCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsPlaying(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsPlaying", (name))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn PlayDefaultAnimation(
        &mut self,
        mode: crate::UnityEngine::PlayMode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("PlayDefaultAnimation", (mode))?;
        Ok(__cordl_ret)
    }
    pub fn PlayQueued_Il2CppString0(
        &mut self,
        animation: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AnimationState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AnimationState = __cordl_object
            .invoke("PlayQueued", (animation))?;
        Ok(__cordl_ret)
    }
    pub fn PlayQueued_QueueMode1(
        &mut self,
        animation: *mut quest_hook::libil2cpp::Il2CppString,
        queue: crate::UnityEngine::QueueMode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AnimationState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AnimationState = __cordl_object
            .invoke("PlayQueued", (animation, queue))?;
        Ok(__cordl_ret)
    }
    pub fn PlayQueued_QueueMode_PlayMode2(
        &mut self,
        animation: *mut quest_hook::libil2cpp::Il2CppString,
        queue: crate::UnityEngine::QueueMode,
        mode: crate::UnityEngine::PlayMode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AnimationState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AnimationState = __cordl_object
            .invoke("PlayQueued", (animation, queue, mode))?;
        Ok(__cordl_ret)
    }
    pub fn Play_0(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Play", ())?;
        Ok(__cordl_ret)
    }
    pub fn Play_AnimationPlayMode4(
        &mut self,
        mode: crate::UnityEngine::AnimationPlayMode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Play", (mode))?;
        Ok(__cordl_ret)
    }
    pub fn Play_Il2CppString2(
        &mut self,
        animation: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Play", (animation))?;
        Ok(__cordl_ret)
    }
    pub fn Play_Il2CppString_AnimationPlayMode5(
        &mut self,
        animation: *mut quest_hook::libil2cpp::Il2CppString,
        mode: crate::UnityEngine::AnimationPlayMode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Play", (animation, mode))?;
        Ok(__cordl_ret)
    }
    pub fn Play_Il2CppString_PlayMode3(
        &mut self,
        animation: *mut quest_hook::libil2cpp::Il2CppString,
        mode: crate::UnityEngine::PlayMode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Play", (animation, mode))?;
        Ok(__cordl_ret)
    }
    pub fn Play_PlayMode1(
        &mut self,
        mode: crate::UnityEngine::PlayMode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Play", (mode))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveClipNamed(
        &mut self,
        clipName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveClipNamed", (clipName))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveClip_AnimationClip0(
        &mut self,
        clip: *mut crate::UnityEngine::AnimationClip,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveClip", (clip))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveClip_Il2CppString1(
        &mut self,
        clipName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveClip", (clipName))?;
        Ok(__cordl_ret)
    }
    pub fn RewindNamed(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RewindNamed", (name))?;
        Ok(__cordl_ret)
    }
    pub fn Rewind_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Rewind", ())?;
        Ok(__cordl_ret)
    }
    pub fn Rewind_Il2CppString1(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Rewind", (name))?;
        Ok(__cordl_ret)
    }
    pub fn Sample(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Sample", ())?;
        Ok(__cordl_ret)
    }
    pub fn StopNamed(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopNamed", (name))?;
        Ok(__cordl_ret)
    }
    pub fn Stop_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Stop", ())?;
        Ok(__cordl_ret)
    }
    pub fn Stop_Il2CppString1(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Stop", (name))?;
        Ok(__cordl_ret)
    }
    pub fn SyncLayer(
        &mut self,
        layer: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SyncLayer", (layer))?;
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
    pub fn get_Item(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AnimationState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AnimationState = __cordl_object
            .invoke("get_Item", (name))?;
        Ok(__cordl_ret)
    }
    pub fn get_animateOnlyIfVisible(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_animateOnlyIfVisible", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_animatePhysics(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_animatePhysics", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_clip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AnimationClip> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AnimationClip = __cordl_object
            .invoke("get_clip", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_cullingType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::AnimationCullingType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::AnimationCullingType = __cordl_object
            .invoke("get_cullingType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isPlaying(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isPlaying", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_localBounds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Bounds> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Bounds = __cordl_object
            .invoke("get_localBounds", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_localBounds_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_localBounds_Injected", (ret))?;
        Ok(__cordl_ret)
    }
    pub fn get_playAutomatically(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_playAutomatically", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_wrapMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::WrapMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::WrapMode = __cordl_object
            .invoke("get_wrapMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_animateOnlyIfVisible(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_animateOnlyIfVisible", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_animatePhysics(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_animatePhysics", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_clip(
        &mut self,
        value: *mut crate::UnityEngine::AnimationClip,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_clip", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_cullingType(
        &mut self,
        value: crate::UnityEngine::AnimationCullingType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_cullingType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_localBounds(
        &mut self,
        value: crate::UnityEngine::Bounds,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_localBounds", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_localBounds_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_localBounds_Injected", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_playAutomatically(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_playAutomatically", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_wrapMode(
        &mut self,
        value: crate::UnityEngine::WrapMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_wrapMode", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Animation")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Animation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Animation+Enumerator")]
#[repr(C)]
#[derive(Debug)]
pub struct Animation_Enumerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Outer: *mut crate::UnityEngine::Animation,
    pub m_CurrentIndex: i32,
}
#[cfg(feature = "UnityEngine+Animation+Enumerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Animation_Enumerator =>
    "UnityEngine"."Animation/Enumerator"
);
#[cfg(feature = "UnityEngine+Animation+Enumerator")]
impl std::ops::Deref for crate::UnityEngine::Animation_Enumerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Animation+Enumerator")]
impl std::ops::DerefMut for crate::UnityEngine::Animation_Enumerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Animation+Enumerator")]
impl crate::UnityEngine::Animation_Enumerator {
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveNext", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        outer: *mut crate::UnityEngine::Animation,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (outer))?;
        Ok(__cordl_object)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        outer: *mut crate::UnityEngine::Animation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (outer))?;
        Ok(__cordl_ret)
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("get_Current", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Animation+Enumerator")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Animation_Enumerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
