#[cfg(feature = "UnityEngine+Animator")]
#[repr(C)]
#[derive(Debug)]
pub struct Animator {
    __cordl_parent: crate::UnityEngine::Behaviour,
}
#[cfg(feature = "UnityEngine+Animator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Animator => "UnityEngine"
    ."Animator"
);
#[cfg(feature = "UnityEngine+Animator")]
impl std::ops::Deref for crate::UnityEngine::Animator {
    type Target = crate::UnityEngine::Behaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Animator")]
impl std::ops::DerefMut for crate::UnityEngine::Animator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Animator")]
impl crate::UnityEngine::Animator {
    pub fn ApplyBuiltinRootMotion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyBuiltinRootMotion", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckIfInIKPass(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckIfInIKPass", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearInternalControllerPlayable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearInternalControllerPlayable", ())?;
        Ok(__cordl_ret)
    }
    pub fn CrossFadeInFixedTime_String0(
        &mut self,
        stateName: *mut crate::System::String,
        fixedTransitionDuration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CrossFadeInFixedTime", (stateName, fixedTransitionDuration))?;
        Ok(__cordl_ret)
    }
    pub fn CrossFadeInFixedTime_String_i32_1(
        &mut self,
        stateName: *mut crate::System::String,
        fixedTransitionDuration: f32,
        layer: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CrossFadeInFixedTime",
                (stateName, fixedTransitionDuration, layer),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CrossFadeInFixedTime_String_i32_f32_2(
        &mut self,
        stateName: *mut crate::System::String,
        fixedTransitionDuration: f32,
        layer: i32,
        fixedTimeOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CrossFadeInFixedTime",
                (stateName, fixedTransitionDuration, layer, fixedTimeOffset),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CrossFadeInFixedTime_String_i32_f32_f32_3(
        &mut self,
        stateName: *mut crate::System::String,
        fixedTransitionDuration: f32,
        layer: i32,
        fixedTimeOffset: f32,
        normalizedTransitionTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CrossFadeInFixedTime",
                (
                    stateName,
                    fixedTransitionDuration,
                    layer,
                    fixedTimeOffset,
                    normalizedTransitionTime,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CrossFadeInFixedTime_i32_6(
        &mut self,
        stateHashName: i32,
        fixedTransitionDuration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CrossFadeInFixedTime", (stateHashName, fixedTransitionDuration))?;
        Ok(__cordl_ret)
    }
    pub fn CrossFadeInFixedTime_i32_i32_5(
        &mut self,
        stateHashName: i32,
        fixedTransitionDuration: f32,
        layer: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CrossFadeInFixedTime",
                (stateHashName, fixedTransitionDuration, layer),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CrossFadeInFixedTime_i32_i32_f32_4(
        &mut self,
        stateHashName: i32,
        fixedTransitionDuration: f32,
        layer: i32,
        fixedTimeOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CrossFadeInFixedTime",
                (stateHashName, fixedTransitionDuration, layer, fixedTimeOffset),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CrossFadeInFixedTime_i32_i32_f32_f32_7(
        &mut self,
        stateHashName: i32,
        fixedTransitionDuration: f32,
        layer: i32,
        fixedTimeOffset: f32,
        normalizedTransitionTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CrossFadeInFixedTime",
                (
                    stateHashName,
                    fixedTransitionDuration,
                    layer,
                    fixedTimeOffset,
                    normalizedTransitionTime,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CrossFade_String2(
        &mut self,
        stateName: *mut crate::System::String,
        normalizedTransitionDuration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CrossFade", (stateName, normalizedTransitionDuration))?;
        Ok(__cordl_ret)
    }
    pub fn CrossFade_String_i32_1(
        &mut self,
        stateName: *mut crate::System::String,
        normalizedTransitionDuration: f32,
        layer: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CrossFade", (stateName, normalizedTransitionDuration, layer))?;
        Ok(__cordl_ret)
    }
    pub fn CrossFade_String_i32_f32_0(
        &mut self,
        stateName: *mut crate::System::String,
        normalizedTransitionDuration: f32,
        layer: i32,
        normalizedTimeOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CrossFade",
                (stateName, normalizedTransitionDuration, layer, normalizedTimeOffset),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CrossFade_String_i32_f32_f32_3(
        &mut self,
        stateName: *mut crate::System::String,
        normalizedTransitionDuration: f32,
        layer: i32,
        normalizedTimeOffset: f32,
        normalizedTransitionTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CrossFade",
                (
                    stateName,
                    normalizedTransitionDuration,
                    layer,
                    normalizedTimeOffset,
                    normalizedTransitionTime,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CrossFade_i32_7(
        &mut self,
        stateHashName: i32,
        normalizedTransitionDuration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CrossFade", (stateHashName, normalizedTransitionDuration))?;
        Ok(__cordl_ret)
    }
    pub fn CrossFade_i32_i32_6(
        &mut self,
        stateHashName: i32,
        normalizedTransitionDuration: f32,
        layer: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CrossFade", (stateHashName, normalizedTransitionDuration, layer))?;
        Ok(__cordl_ret)
    }
    pub fn CrossFade_i32_i32_f32_5(
        &mut self,
        stateHashName: i32,
        normalizedTransitionDuration: f32,
        layer: i32,
        normalizedTimeOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CrossFade",
                (
                    stateHashName,
                    normalizedTransitionDuration,
                    layer,
                    normalizedTimeOffset,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CrossFade_i32_i32_f32_f32_4(
        &mut self,
        stateHashName: i32,
        normalizedTransitionDuration: f32,
        layer: i32,
        normalizedTimeOffset: f32,
        normalizedTransitionTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CrossFade",
                (
                    stateHashName,
                    normalizedTransitionDuration,
                    layer,
                    normalizedTimeOffset,
                    normalizedTransitionTime,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EvaluateController_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EvaluateController", ())?;
        Ok(__cordl_ret)
    }
    pub fn EvaluateController_f32_1(
        &mut self,
        deltaTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EvaluateController", (deltaTime))?;
        Ok(__cordl_ret)
    }
    pub fn ForceStateNormalizedTime(
        &mut self,
        normalizedTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ForceStateNormalizedTime", (normalizedTime))?;
        Ok(__cordl_ret)
    }
    pub fn GetAnimatorClipInfoCount(
        &mut self,
        layerIndex: i32,
        current: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetAnimatorClipInfoCount", (layerIndex, current))?;
        Ok(__cordl_ret)
    }
    pub fn GetAnimatorClipInfoInternal(
        &mut self,
        layerIndex: i32,
        isCurrent: bool,
        clips: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetAnimatorClipInfoInternal", (layerIndex, isCurrent, clips))?;
        Ok(__cordl_ret)
    }
    pub fn GetAnimatorStateInfo(
        &mut self,
        layerIndex: i32,
        stateInfoIndex: crate::UnityEngine::StateInfoIndex,
        info: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::AnimatorStateInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetAnimatorStateInfo", (layerIndex, stateInfoIndex, info))?;
        Ok(__cordl_ret)
    }
    pub fn GetAnimatorStateName(
        &mut self,
        layerIndex: i32,
        current: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetAnimatorStateName", (layerIndex, current))?;
        Ok(__cordl_ret)
    }
    pub fn GetAnimatorTransitionInfo_ByRefMut0(
        &mut self,
        layerIndex: i32,
        info: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::AnimatorTransitionInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetAnimatorTransitionInfo", (layerIndex, info))?;
        Ok(__cordl_ret)
    }
    pub fn GetAnimatorTransitionInfo_i32_1(
        &mut self,
        layerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::AnimatorTransitionInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::AnimatorTransitionInfo = __cordl_object
            .invoke("GetAnimatorTransitionInfo", (layerIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetBehaviour_1<T>(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("GetBehaviour", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetBehaviour_Type0(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::ScriptableObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ScriptableObject = __cordl_object
            .invoke("GetBehaviour", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn GetBehaviours_0<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<T> = __cordl_object
            .invoke("GetBehaviours", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetBehaviours_i32_i32_1(
        &mut self,
        fullPathHash: i32,
        layerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::StateMachineBehaviour,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::StateMachineBehaviour,
        > = __cordl_object.invoke("GetBehaviours", (fullPathHash, layerIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetBoneTransform(
        &mut self,
        humanBoneId: crate::UnityEngine::HumanBodyBones,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("GetBoneTransform", (humanBoneId))?;
        Ok(__cordl_ret)
    }
    pub fn GetBoneTransformInternal(
        &mut self,
        humanBoneId: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("GetBoneTransformInternal", (humanBoneId))?;
        Ok(__cordl_ret)
    }
    pub fn GetBoolID(&mut self, id: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetBoolID", (id))?;
        Ok(__cordl_ret)
    }
    pub fn GetBoolString(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetBoolString", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetBool_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetBool", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetBool_i32_1(&mut self, id: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetBool", (id))?;
        Ok(__cordl_ret)
    }
    pub fn GetCurrentAnimatorClipInfoCount(
        &mut self,
        layerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetCurrentAnimatorClipInfoCount", (layerIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetCurrentAnimatorClipInfo_List_1_1(
        &mut self,
        layerIndex: i32,
        clips: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::AnimatorClipInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetCurrentAnimatorClipInfo", (layerIndex, clips))?;
        Ok(__cordl_ret)
    }
    pub fn GetCurrentAnimatorClipInfo_i32_0(
        &mut self,
        layerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::AnimatorClipInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::AnimatorClipInfo,
        > = __cordl_object.invoke("GetCurrentAnimatorClipInfo", (layerIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetCurrentAnimatorStateInfo(
        &mut self,
        layerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::AnimatorStateInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::AnimatorStateInfo = __cordl_object
            .invoke("GetCurrentAnimatorStateInfo", (layerIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetCurrentGraph(
        &mut self,
        graph: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableGraph,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetCurrentGraph", (graph))?;
        Ok(__cordl_ret)
    }
    pub fn GetCurrentStateName(
        &mut self,
        layerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetCurrentStateName", (layerIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetFloatID(&mut self, id: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetFloatID", (id))?;
        Ok(__cordl_ret)
    }
    pub fn GetFloatString(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetFloatString", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetFloat_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetFloat", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetFloat_i32_1(&mut self, id: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetFloat", (id))?;
        Ok(__cordl_ret)
    }
    pub fn GetGoalPosition(
        &mut self,
        goal: crate::UnityEngine::AvatarIKGoal,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("GetGoalPosition", (goal))?;
        Ok(__cordl_ret)
    }
    pub fn GetGoalPosition_Injected(
        &mut self,
        goal: crate::UnityEngine::AvatarIKGoal,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetGoalPosition_Injected", (goal, ret))?;
        Ok(__cordl_ret)
    }
    pub fn GetGoalRotation(
        &mut self,
        goal: crate::UnityEngine::AvatarIKGoal,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("GetGoalRotation", (goal))?;
        Ok(__cordl_ret)
    }
    pub fn GetGoalRotation_Injected(
        &mut self,
        goal: crate::UnityEngine::AvatarIKGoal,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetGoalRotation_Injected", (goal, ret))?;
        Ok(__cordl_ret)
    }
    pub fn GetGoalWeightPosition(
        &mut self,
        goal: crate::UnityEngine::AvatarIKGoal,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetGoalWeightPosition", (goal))?;
        Ok(__cordl_ret)
    }
    pub fn GetGoalWeightRotation(
        &mut self,
        goal: crate::UnityEngine::AvatarIKGoal,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetGoalWeightRotation", (goal))?;
        Ok(__cordl_ret)
    }
    pub fn GetHintPosition(
        &mut self,
        hint: crate::UnityEngine::AvatarIKHint,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("GetHintPosition", (hint))?;
        Ok(__cordl_ret)
    }
    pub fn GetHintPosition_Injected(
        &mut self,
        hint: crate::UnityEngine::AvatarIKHint,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetHintPosition_Injected", (hint, ret))?;
        Ok(__cordl_ret)
    }
    pub fn GetHintWeightPosition(
        &mut self,
        hint: crate::UnityEngine::AvatarIKHint,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetHintWeightPosition", (hint))?;
        Ok(__cordl_ret)
    }
    pub fn GetIKHintPosition(
        &mut self,
        hint: crate::UnityEngine::AvatarIKHint,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("GetIKHintPosition", (hint))?;
        Ok(__cordl_ret)
    }
    pub fn GetIKHintPositionWeight(
        &mut self,
        hint: crate::UnityEngine::AvatarIKHint,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetIKHintPositionWeight", (hint))?;
        Ok(__cordl_ret)
    }
    pub fn GetIKPosition(
        &mut self,
        goal: crate::UnityEngine::AvatarIKGoal,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("GetIKPosition", (goal))?;
        Ok(__cordl_ret)
    }
    pub fn GetIKPositionWeight(
        &mut self,
        goal: crate::UnityEngine::AvatarIKGoal,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetIKPositionWeight", (goal))?;
        Ok(__cordl_ret)
    }
    pub fn GetIKRotation(
        &mut self,
        goal: crate::UnityEngine::AvatarIKGoal,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("GetIKRotation", (goal))?;
        Ok(__cordl_ret)
    }
    pub fn GetIKRotationWeight(
        &mut self,
        goal: crate::UnityEngine::AvatarIKGoal,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetIKRotationWeight", (goal))?;
        Ok(__cordl_ret)
    }
    pub fn GetIntegerID(&mut self, id: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetIntegerID", (id))?;
        Ok(__cordl_ret)
    }
    pub fn GetIntegerString(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetIntegerString", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetInteger_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetInteger", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetInteger_i32_1(&mut self, id: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetInteger", (id))?;
        Ok(__cordl_ret)
    }
    pub fn GetLayerIndex(
        &mut self,
        layerName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetLayerIndex", (layerName))?;
        Ok(__cordl_ret)
    }
    pub fn GetLayerName(
        &mut self,
        layerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetLayerName", (layerIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetLayerWeight(
        &mut self,
        layerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetLayerWeight", (layerIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetNextAnimatorClipInfoCount(
        &mut self,
        layerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetNextAnimatorClipInfoCount", (layerIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetNextAnimatorClipInfo_List_1_1(
        &mut self,
        layerIndex: i32,
        clips: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::AnimatorClipInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetNextAnimatorClipInfo", (layerIndex, clips))?;
        Ok(__cordl_ret)
    }
    pub fn GetNextAnimatorClipInfo_i32_0(
        &mut self,
        layerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::AnimatorClipInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::AnimatorClipInfo,
        > = __cordl_object.invoke("GetNextAnimatorClipInfo", (layerIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetNextAnimatorStateInfo(
        &mut self,
        layerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::AnimatorStateInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::AnimatorStateInfo = __cordl_object
            .invoke("GetNextAnimatorStateInfo", (layerIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetNextStateName(
        &mut self,
        layerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetNextStateName", (layerIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetParameter(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::AnimatorControllerParameter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AnimatorControllerParameter = __cordl_object
            .invoke("GetParameter", (index))?;
        Ok(__cordl_ret)
    }
    pub fn GetParameterInternal(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::AnimatorControllerParameter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AnimatorControllerParameter = __cordl_object
            .invoke("GetParameterInternal", (index))?;
        Ok(__cordl_ret)
    }
    pub fn GetQuaternion_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("GetQuaternion", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetQuaternion_i32_1(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("GetQuaternion", (id))?;
        Ok(__cordl_ret)
    }
    pub fn GetRecorderStartTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetRecorderStartTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetRecorderStopTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetRecorderStopTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetStats(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetStats", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetVector_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("GetVector", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetVector_i32_1(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("GetVector", (id))?;
        Ok(__cordl_ret)
    }
    pub fn HasState(
        &mut self,
        layerIndex: i32,
        stateID: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasState", (layerIndex, stateID))?;
        Ok(__cordl_ret)
    }
    pub fn InternalGetBehaviours(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::ScriptableObject,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::ScriptableObject,
        > = __cordl_object.invoke("InternalGetBehaviours", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn InternalGetBehavioursByKey(
        &mut self,
        fullPathHash: i32,
        layerIndex: i32,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::ScriptableObject,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::ScriptableObject,
        > = __cordl_object
            .invoke(
                "InternalGetBehavioursByKey",
                (fullPathHash, layerIndex, _cordl_type),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InterruptMatchTarget_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InterruptMatchTarget", ())?;
        Ok(__cordl_ret)
    }
    pub fn InterruptMatchTarget__cordl_bool1(
        &mut self,
        completeMatch: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InterruptMatchTarget", (completeMatch))?;
        Ok(__cordl_ret)
    }
    pub fn IsBoneTransform(
        &mut self,
        transform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsBoneTransform", (transform))?;
        Ok(__cordl_ret)
    }
    pub fn IsControlled(
        &mut self,
        transform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsControlled", (transform))?;
        Ok(__cordl_ret)
    }
    pub fn IsInIKPass(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsInIKPass", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsInTransition(
        &mut self,
        layerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsInTransition", (layerIndex))?;
        Ok(__cordl_ret)
    }
    pub fn IsParameterControlledByCurveID(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsParameterControlledByCurveID", (id))?;
        Ok(__cordl_ret)
    }
    pub fn IsParameterControlledByCurveString(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsParameterControlledByCurveString", (name))?;
        Ok(__cordl_ret)
    }
    pub fn IsParameterControlledByCurve_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsParameterControlledByCurve", (name))?;
        Ok(__cordl_ret)
    }
    pub fn IsParameterControlledByCurve_i32_1(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsParameterControlledByCurve", (id))?;
        Ok(__cordl_ret)
    }
    pub fn MatchTarget_AvatarTarget1(
        &mut self,
        matchPosition: crate::UnityEngine::Vector3,
        matchRotation: crate::UnityEngine::Quaternion,
        targetBodyPart: crate::UnityEngine::AvatarTarget,
        weightMask: crate::UnityEngine::MatchTargetWeightMask,
        startNormalizedTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "MatchTarget",
                (
                    matchPosition,
                    matchRotation,
                    targetBodyPart,
                    weightMask,
                    startNormalizedTime,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn MatchTarget_AvatarTarget_f32_2(
        &mut self,
        matchPosition: crate::UnityEngine::Vector3,
        matchRotation: crate::UnityEngine::Quaternion,
        targetBodyPart: crate::UnityEngine::AvatarTarget,
        weightMask: crate::UnityEngine::MatchTargetWeightMask,
        startNormalizedTime: f32,
        targetNormalizedTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "MatchTarget",
                (
                    matchPosition,
                    matchRotation,
                    targetBodyPart,
                    weightMask,
                    startNormalizedTime,
                    targetNormalizedTime,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn MatchTarget_AvatarTarget_f32__cordl_bool3(
        &mut self,
        matchPosition: crate::UnityEngine::Vector3,
        matchRotation: crate::UnityEngine::Quaternion,
        targetBodyPart: crate::UnityEngine::AvatarTarget,
        weightMask: crate::UnityEngine::MatchTargetWeightMask,
        startNormalizedTime: f32,
        targetNormalizedTime: f32,
        completeMatch: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "MatchTarget",
                (
                    matchPosition,
                    matchRotation,
                    targetBodyPart,
                    weightMask,
                    startNormalizedTime,
                    targetNormalizedTime,
                    completeMatch,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn MatchTarget_Injected(
        &mut self,
        matchPosition: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        matchRotation: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
        targetBodyPart: i32,
        weightMask: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::MatchTargetWeightMask,
        >,
        startNormalizedTime: f32,
        targetNormalizedTime: f32,
        completeMatch: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "MatchTarget_Injected",
                (
                    matchPosition,
                    matchRotation,
                    targetBodyPart,
                    weightMask,
                    startNormalizedTime,
                    targetNormalizedTime,
                    completeMatch,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn MatchTarget_i32_f32__cordl_bool0(
        &mut self,
        matchPosition: crate::UnityEngine::Vector3,
        matchRotation: crate::UnityEngine::Quaternion,
        targetBodyPart: i32,
        weightMask: crate::UnityEngine::MatchTargetWeightMask,
        startNormalizedTime: f32,
        targetNormalizedTime: f32,
        completeMatch: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "MatchTarget",
                (
                    matchPosition,
                    matchRotation,
                    targetBodyPart,
                    weightMask,
                    startNormalizedTime,
                    targetNormalizedTime,
                    completeMatch,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnCullingModeChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCullingModeChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnUpdateModeChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnUpdateModeChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn PlayInFixedTime_String1(
        &mut self,
        stateName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlayInFixedTime", (stateName))?;
        Ok(__cordl_ret)
    }
    pub fn PlayInFixedTime_String_i32_0(
        &mut self,
        stateName: *mut crate::System::String,
        layer: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlayInFixedTime", (stateName, layer))?;
        Ok(__cordl_ret)
    }
    pub fn PlayInFixedTime_String_i32_f32_2(
        &mut self,
        stateName: *mut crate::System::String,
        layer: i32,
        fixedTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlayInFixedTime", (stateName, layer, fixedTime))?;
        Ok(__cordl_ret)
    }
    pub fn PlayInFixedTime_i32_5(
        &mut self,
        stateNameHash: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlayInFixedTime", (stateNameHash))?;
        Ok(__cordl_ret)
    }
    pub fn PlayInFixedTime_i32_i32_4(
        &mut self,
        stateNameHash: i32,
        layer: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlayInFixedTime", (stateNameHash, layer))?;
        Ok(__cordl_ret)
    }
    pub fn PlayInFixedTime_i32_i32_f32_3(
        &mut self,
        stateNameHash: i32,
        layer: i32,
        fixedTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlayInFixedTime", (stateNameHash, layer, fixedTime))?;
        Ok(__cordl_ret)
    }
    pub fn Play_String1(
        &mut self,
        stateName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Play", (stateName))?;
        Ok(__cordl_ret)
    }
    pub fn Play_String_i32_0(
        &mut self,
        stateName: *mut crate::System::String,
        layer: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Play", (stateName, layer))?;
        Ok(__cordl_ret)
    }
    pub fn Play_String_i32_f32_2(
        &mut self,
        stateName: *mut crate::System::String,
        layer: i32,
        normalizedTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Play", (stateName, layer, normalizedTime))?;
        Ok(__cordl_ret)
    }
    pub fn Play_i32_5(
        &mut self,
        stateNameHash: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Play", (stateNameHash))?;
        Ok(__cordl_ret)
    }
    pub fn Play_i32_i32_4(
        &mut self,
        stateNameHash: i32,
        layer: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Play", (stateNameHash, layer))?;
        Ok(__cordl_ret)
    }
    pub fn Play_i32_i32_f32_3(
        &mut self,
        stateNameHash: i32,
        layer: i32,
        normalizedTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Play", (stateNameHash, layer, normalizedTime))?;
        Ok(__cordl_ret)
    }
    pub fn Rebind_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Rebind", ())?;
        Ok(__cordl_ret)
    }
    pub fn Rebind__cordl_bool1(
        &mut self,
        writeDefaultValues: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Rebind", (writeDefaultValues))?;
        Ok(__cordl_ret)
    }
    pub fn ResetTriggerID(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetTriggerID", (id))?;
        Ok(__cordl_ret)
    }
    pub fn ResetTriggerString(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetTriggerString", (name))?;
        Ok(__cordl_ret)
    }
    pub fn ResetTrigger_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetTrigger", (name))?;
        Ok(__cordl_ret)
    }
    pub fn ResetTrigger_i32_1(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetTrigger", (id))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveHash(
        &mut self,
        hash: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ResolveHash", (hash))?;
        Ok(__cordl_ret)
    }
    pub fn SetBoneLocalRotation(
        &mut self,
        humanBoneId: crate::UnityEngine::HumanBodyBones,
        rotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBoneLocalRotation", (humanBoneId, rotation))?;
        Ok(__cordl_ret)
    }
    pub fn SetBoneLocalRotationInternal(
        &mut self,
        humanBoneId: i32,
        rotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBoneLocalRotationInternal", (humanBoneId, rotation))?;
        Ok(__cordl_ret)
    }
    pub fn SetBoneLocalRotationInternal_Injected(
        &mut self,
        humanBoneId: i32,
        rotation: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBoneLocalRotationInternal_Injected", (humanBoneId, rotation))?;
        Ok(__cordl_ret)
    }
    pub fn SetBoolID(
        &mut self,
        id: i32,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBoolID", (id, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetBoolString(
        &mut self,
        name: *mut crate::System::String,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBoolString", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetBool_String0(
        &mut self,
        name: *mut crate::System::String,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBool", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetBool_i32_1(
        &mut self,
        id: i32,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBool", (id, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetFloatID(
        &mut self,
        id: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFloatID", (id, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetFloatIDDamp(
        &mut self,
        id: i32,
        value: f32,
        dampTime: f32,
        deltaTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFloatIDDamp", (id, value, dampTime, deltaTime))?;
        Ok(__cordl_ret)
    }
    pub fn SetFloatString(
        &mut self,
        name: *mut crate::System::String,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFloatString", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetFloatStringDamp(
        &mut self,
        name: *mut crate::System::String,
        value: f32,
        dampTime: f32,
        deltaTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFloatStringDamp", (name, value, dampTime, deltaTime))?;
        Ok(__cordl_ret)
    }
    pub fn SetFloat_String0(
        &mut self,
        name: *mut crate::System::String,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFloat", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetFloat_String_f32_f32_1(
        &mut self,
        name: *mut crate::System::String,
        value: f32,
        dampTime: f32,
        deltaTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFloat", (name, value, dampTime, deltaTime))?;
        Ok(__cordl_ret)
    }
    pub fn SetFloat_i32_2(
        &mut self,
        id: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFloat", (id, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetFloat_i32_f32_f32_3(
        &mut self,
        id: i32,
        value: f32,
        dampTime: f32,
        deltaTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFloat", (id, value, dampTime, deltaTime))?;
        Ok(__cordl_ret)
    }
    pub fn SetGoalPosition(
        &mut self,
        goal: crate::UnityEngine::AvatarIKGoal,
        goalPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGoalPosition", (goal, goalPosition))?;
        Ok(__cordl_ret)
    }
    pub fn SetGoalPosition_Injected(
        &mut self,
        goal: crate::UnityEngine::AvatarIKGoal,
        goalPosition: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGoalPosition_Injected", (goal, goalPosition))?;
        Ok(__cordl_ret)
    }
    pub fn SetGoalRotation(
        &mut self,
        goal: crate::UnityEngine::AvatarIKGoal,
        goalRotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGoalRotation", (goal, goalRotation))?;
        Ok(__cordl_ret)
    }
    pub fn SetGoalRotation_Injected(
        &mut self,
        goal: crate::UnityEngine::AvatarIKGoal,
        goalRotation: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGoalRotation_Injected", (goal, goalRotation))?;
        Ok(__cordl_ret)
    }
    pub fn SetGoalWeightPosition(
        &mut self,
        goal: crate::UnityEngine::AvatarIKGoal,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGoalWeightPosition", (goal, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetGoalWeightRotation(
        &mut self,
        goal: crate::UnityEngine::AvatarIKGoal,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGoalWeightRotation", (goal, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetHintPosition(
        &mut self,
        hint: crate::UnityEngine::AvatarIKHint,
        hintPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetHintPosition", (hint, hintPosition))?;
        Ok(__cordl_ret)
    }
    pub fn SetHintPosition_Injected(
        &mut self,
        hint: crate::UnityEngine::AvatarIKHint,
        hintPosition: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetHintPosition_Injected", (hint, hintPosition))?;
        Ok(__cordl_ret)
    }
    pub fn SetHintWeightPosition(
        &mut self,
        hint: crate::UnityEngine::AvatarIKHint,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetHintWeightPosition", (hint, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetIKHintPosition(
        &mut self,
        hint: crate::UnityEngine::AvatarIKHint,
        hintPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIKHintPosition", (hint, hintPosition))?;
        Ok(__cordl_ret)
    }
    pub fn SetIKHintPositionWeight(
        &mut self,
        hint: crate::UnityEngine::AvatarIKHint,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIKHintPositionWeight", (hint, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetIKPosition(
        &mut self,
        goal: crate::UnityEngine::AvatarIKGoal,
        goalPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIKPosition", (goal, goalPosition))?;
        Ok(__cordl_ret)
    }
    pub fn SetIKPositionWeight(
        &mut self,
        goal: crate::UnityEngine::AvatarIKGoal,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIKPositionWeight", (goal, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetIKRotation(
        &mut self,
        goal: crate::UnityEngine::AvatarIKGoal,
        goalRotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIKRotation", (goal, goalRotation))?;
        Ok(__cordl_ret)
    }
    pub fn SetIKRotationWeight(
        &mut self,
        goal: crate::UnityEngine::AvatarIKGoal,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIKRotationWeight", (goal, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetIntegerID(
        &mut self,
        id: i32,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIntegerID", (id, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetIntegerString(
        &mut self,
        name: *mut crate::System::String,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIntegerString", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetInteger_String0(
        &mut self,
        name: *mut crate::System::String,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetInteger", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetInteger_i32_1(
        &mut self,
        id: i32,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetInteger", (id, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetLayerWeight(
        &mut self,
        layerIndex: i32,
        weight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLayerWeight", (layerIndex, weight))?;
        Ok(__cordl_ret)
    }
    pub fn SetLookAtPosition(
        &mut self,
        lookAtPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLookAtPosition", (lookAtPosition))?;
        Ok(__cordl_ret)
    }
    pub fn SetLookAtPositionInternal(
        &mut self,
        lookAtPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLookAtPositionInternal", (lookAtPosition))?;
        Ok(__cordl_ret)
    }
    pub fn SetLookAtPositionInternal_Injected(
        &mut self,
        lookAtPosition: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLookAtPositionInternal_Injected", (lookAtPosition))?;
        Ok(__cordl_ret)
    }
    pub fn SetLookAtWeightInternal(
        &mut self,
        weight: f32,
        bodyWeight: f32,
        headWeight: f32,
        eyesWeight: f32,
        clampWeight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetLookAtWeightInternal",
                (weight, bodyWeight, headWeight, eyesWeight, clampWeight),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetLookAtWeight_f32_0(
        &mut self,
        weight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLookAtWeight", (weight))?;
        Ok(__cordl_ret)
    }
    pub fn SetLookAtWeight_f32_1(
        &mut self,
        weight: f32,
        bodyWeight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLookAtWeight", (weight, bodyWeight))?;
        Ok(__cordl_ret)
    }
    pub fn SetLookAtWeight_f32_f32_2(
        &mut self,
        weight: f32,
        bodyWeight: f32,
        headWeight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLookAtWeight", (weight, bodyWeight, headWeight))?;
        Ok(__cordl_ret)
    }
    pub fn SetLookAtWeight_f32_f32_f32_3(
        &mut self,
        weight: f32,
        bodyWeight: f32,
        headWeight: f32,
        eyesWeight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLookAtWeight", (weight, bodyWeight, headWeight, eyesWeight))?;
        Ok(__cordl_ret)
    }
    pub fn SetLookAtWeight_f32_f32_f32_f32_4(
        &mut self,
        weight: f32,
        bodyWeight: f32,
        headWeight: f32,
        eyesWeight: f32,
        clampWeight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetLookAtWeight",
                (weight, bodyWeight, headWeight, eyesWeight, clampWeight),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetQuaternion_String0(
        &mut self,
        name: *mut crate::System::String,
        value: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetQuaternion", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetQuaternion_i32_1(
        &mut self,
        id: i32,
        value: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetQuaternion", (id, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetTarget(
        &mut self,
        targetIndex: crate::UnityEngine::AvatarTarget,
        targetNormalizedTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTarget", (targetIndex, targetNormalizedTime))?;
        Ok(__cordl_ret)
    }
    pub fn SetTriggerID(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTriggerID", (id))?;
        Ok(__cordl_ret)
    }
    pub fn SetTriggerString(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTriggerString", (name))?;
        Ok(__cordl_ret)
    }
    pub fn SetTrigger_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTrigger", (name))?;
        Ok(__cordl_ret)
    }
    pub fn SetTrigger_i32_1(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTrigger", (id))?;
        Ok(__cordl_ret)
    }
    pub fn SetVector_String0(
        &mut self,
        name: *mut crate::System::String,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVector", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetVector_i32_1(
        &mut self,
        id: i32,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVector", (id, value))?;
        Ok(__cordl_ret)
    }
    pub fn StartPlayback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartPlayback", ())?;
        Ok(__cordl_ret)
    }
    pub fn StartRecording(
        &mut self,
        frameCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartRecording", (frameCount))?;
        Ok(__cordl_ret)
    }
    pub fn StopPlayback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopPlayback", ())?;
        Ok(__cordl_ret)
    }
    pub fn StopRecording(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopRecording", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
        deltaTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (deltaTime))?;
        Ok(__cordl_ret)
    }
    pub fn WriteDefaultPose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteDefaultPose", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteDefaultValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteDefaultValues", ())?;
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
    pub fn get_allowConstantClipSamplingOptimization(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_allowConstantClipSamplingOptimization", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_angularVelocity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_angularVelocity", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_angularVelocity_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_angularVelocity_Injected", (ret))?;
        Ok(__cordl_ret)
    }
    pub fn get_animatePhysics(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_animatePhysics", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_applyRootMotion(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_applyRootMotion", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_avatar(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Avatar> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Avatar = __cordl_object
            .invoke("get_avatar", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_avatarRoot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_avatarRoot", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_bodyPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_bodyPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_bodyPositionInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_bodyPositionInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_bodyPositionInternal_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_bodyPositionInternal_Injected", (ret))?;
        Ok(__cordl_ret)
    }
    pub fn get_bodyRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("get_bodyRotation", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_bodyRotationInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("get_bodyRotationInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_bodyRotationInternal_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_bodyRotationInternal_Injected", (ret))?;
        Ok(__cordl_ret)
    }
    pub fn get_cullingMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::AnimatorCullingMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::AnimatorCullingMode = __cordl_object
            .invoke("get_cullingMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_deltaPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_deltaPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_deltaPosition_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_deltaPosition_Injected", (ret))?;
        Ok(__cordl_ret)
    }
    pub fn get_deltaRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("get_deltaRotation", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_deltaRotation_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_deltaRotation_Injected", (ret))?;
        Ok(__cordl_ret)
    }
    pub fn get_feetPivotActive(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_feetPivotActive", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fireEvents(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_fireEvents", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_gravityWeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_gravityWeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasBoundPlayables(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasBoundPlayables", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasRootMotion(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasRootMotion", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasTransformHierarchy(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasTransformHierarchy", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_humanScale(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_humanScale", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isHuman(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isHuman", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isInitialized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isInitialized", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isMatchingTarget(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isMatchingTarget", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isOptimizable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isOptimizable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isRootPositionOrRotationControlledByCurves(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_isRootPositionOrRotationControlledByCurves", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_keepAnimatorControllerStateOnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_keepAnimatorControllerStateOnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_keepAnimatorStateOnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_keepAnimatorStateOnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_layerCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_layerCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_layersAffectMassCenter(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_layersAffectMassCenter", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_leftFeetBottomHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_leftFeetBottomHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_linearVelocityBlending(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_linearVelocityBlending", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_logWarnings(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_logWarnings", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_parameterCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_parameterCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_parameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::AnimatorControllerParameter,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::AnimatorControllerParameter,
        > = __cordl_object.invoke("get_parameters", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_pivotPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_pivotPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_pivotPosition_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_pivotPosition_Injected", (ret))?;
        Ok(__cordl_ret)
    }
    pub fn get_pivotWeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_pivotWeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_playableGraph(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::PlayableGraph> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::PlayableGraph = __cordl_object
            .invoke("get_playableGraph", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_playbackTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_playbackTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_recorderMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::AnimatorRecorderMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::AnimatorRecorderMode = __cordl_object
            .invoke("get_recorderMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_recorderStartTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_recorderStartTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_recorderStopTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_recorderStopTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rightFeetBottomHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_rightFeetBottomHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rootPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_rootPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rootPosition_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_rootPosition_Injected", (ret))?;
        Ok(__cordl_ret)
    }
    pub fn get_rootRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("get_rootRotation", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rootRotation_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_rootRotation_Injected", (ret))?;
        Ok(__cordl_ret)
    }
    pub fn get_runtimeAnimatorController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::RuntimeAnimatorController,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::RuntimeAnimatorController = __cordl_object
            .invoke("get_runtimeAnimatorController", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_speed(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_speed", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_stabilizeFeet(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_stabilizeFeet", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_supportsOnAnimatorMove(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_supportsOnAnimatorMove", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_targetPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_targetPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_targetPosition_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_targetPosition_Injected", (ret))?;
        Ok(__cordl_ret)
    }
    pub fn get_targetRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("get_targetRotation", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_targetRotation_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_targetRotation_Injected", (ret))?;
        Ok(__cordl_ret)
    }
    pub fn get_updateMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::AnimatorUpdateMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::AnimatorUpdateMode = __cordl_object
            .invoke("get_updateMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_velocity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_velocity", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_velocity_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_velocity_Injected", (ret))?;
        Ok(__cordl_ret)
    }
    pub fn get_writeDefaultValuesOnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_writeDefaultValuesOnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_allowConstantClipSamplingOptimization(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_allowConstantClipSamplingOptimization", (value))?;
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
    pub fn set_applyRootMotion(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_applyRootMotion", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_avatar(
        &mut self,
        value: *mut crate::UnityEngine::Avatar,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_avatar", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_bodyPosition(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_bodyPosition", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_bodyPositionInternal(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_bodyPositionInternal", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_bodyPositionInternal_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_bodyPositionInternal_Injected", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_bodyRotation(
        &mut self,
        value: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_bodyRotation", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_bodyRotationInternal(
        &mut self,
        value: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_bodyRotationInternal", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_bodyRotationInternal_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_bodyRotationInternal_Injected", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_cullingMode(
        &mut self,
        value: crate::UnityEngine::AnimatorCullingMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_cullingMode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_feetPivotActive(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_feetPivotActive", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_fireEvents(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fireEvents", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_keepAnimatorControllerStateOnDisable(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_keepAnimatorControllerStateOnDisable", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_keepAnimatorStateOnDisable(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_keepAnimatorStateOnDisable", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_layersAffectMassCenter(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_layersAffectMassCenter", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_linearVelocityBlending(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_linearVelocityBlending", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_logWarnings(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_logWarnings", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_playbackTime(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_playbackTime", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_recorderStartTime(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_recorderStartTime", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_recorderStopTime(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_recorderStopTime", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_rootPosition(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rootPosition", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_rootPosition_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rootPosition_Injected", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_rootRotation(
        &mut self,
        value: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rootRotation", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_rootRotation_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rootRotation_Injected", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_runtimeAnimatorController(
        &mut self,
        value: *mut crate::UnityEngine::RuntimeAnimatorController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_runtimeAnimatorController", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_speed(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_speed", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_stabilizeFeet(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_stabilizeFeet", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_updateMode(
        &mut self,
        value: crate::UnityEngine::AnimatorUpdateMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_updateMode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_writeDefaultValuesOnDisable(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_writeDefaultValuesOnDisable", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Animator")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Animator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
