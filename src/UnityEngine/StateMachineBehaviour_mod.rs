#[cfg(feature = "UnityEngine+StateMachineBehaviour")]
#[repr(C)]
#[derive(Debug)]
pub struct StateMachineBehaviour {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
}
#[cfg(feature = "UnityEngine+StateMachineBehaviour")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::StateMachineBehaviour =>
    "UnityEngine"."StateMachineBehaviour"
);
#[cfg(feature = "UnityEngine+StateMachineBehaviour")]
impl std::ops::Deref for crate::UnityEngine::StateMachineBehaviour {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+StateMachineBehaviour")]
impl std::ops::DerefMut for crate::UnityEngine::StateMachineBehaviour {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+StateMachineBehaviour")]
impl crate::UnityEngine::StateMachineBehaviour {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnStateEnter_AnimatorControllerPlayable1(
        &mut self,
        animator: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
        stateInfo: crate::UnityEngine::AnimatorStateInfo,
        layerIndex: i32,
        controller: crate::UnityEngine::Animations::AnimatorControllerPlayable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStateEnter", (animator, stateInfo, layerIndex, controller))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnStateEnter_Animator_AnimatorStateInfo_i32_0(
        &mut self,
        animator: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
        stateInfo: crate::UnityEngine::AnimatorStateInfo,
        layerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStateEnter", (animator, stateInfo, layerIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnStateExit_AnimatorControllerPlayable1(
        &mut self,
        animator: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
        stateInfo: crate::UnityEngine::AnimatorStateInfo,
        layerIndex: i32,
        controller: crate::UnityEngine::Animations::AnimatorControllerPlayable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStateExit", (animator, stateInfo, layerIndex, controller))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnStateExit_Animator_AnimatorStateInfo_i32_0(
        &mut self,
        animator: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
        stateInfo: crate::UnityEngine::AnimatorStateInfo,
        layerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStateExit", (animator, stateInfo, layerIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnStateIK_AnimatorControllerPlayable1(
        &mut self,
        animator: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
        stateInfo: crate::UnityEngine::AnimatorStateInfo,
        layerIndex: i32,
        controller: crate::UnityEngine::Animations::AnimatorControllerPlayable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStateIK", (animator, stateInfo, layerIndex, controller))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnStateIK_Animator_AnimatorStateInfo_i32_0(
        &mut self,
        animator: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
        stateInfo: crate::UnityEngine::AnimatorStateInfo,
        layerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStateIK", (animator, stateInfo, layerIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnStateMachineEnter_AnimatorControllerPlayable1(
        &mut self,
        animator: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
        stateMachinePathHash: i32,
        controller: crate::UnityEngine::Animations::AnimatorControllerPlayable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OnStateMachineEnter",
                (animator, stateMachinePathHash, controller),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OnStateMachineEnter_Animator_i32_0(
        &mut self,
        animator: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
        stateMachinePathHash: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStateMachineEnter", (animator, stateMachinePathHash))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnStateMachineExit_AnimatorControllerPlayable1(
        &mut self,
        animator: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
        stateMachinePathHash: i32,
        controller: crate::UnityEngine::Animations::AnimatorControllerPlayable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStateMachineExit", (animator, stateMachinePathHash, controller))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnStateMachineExit_Animator_i32_0(
        &mut self,
        animator: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
        stateMachinePathHash: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStateMachineExit", (animator, stateMachinePathHash))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnStateMove_AnimatorControllerPlayable1(
        &mut self,
        animator: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
        stateInfo: crate::UnityEngine::AnimatorStateInfo,
        layerIndex: i32,
        controller: crate::UnityEngine::Animations::AnimatorControllerPlayable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStateMove", (animator, stateInfo, layerIndex, controller))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnStateMove_Animator_AnimatorStateInfo_i32_0(
        &mut self,
        animator: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
        stateInfo: crate::UnityEngine::AnimatorStateInfo,
        layerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStateMove", (animator, stateInfo, layerIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnStateUpdate_AnimatorControllerPlayable1(
        &mut self,
        animator: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
        stateInfo: crate::UnityEngine::AnimatorStateInfo,
        layerIndex: i32,
        controller: crate::UnityEngine::Animations::AnimatorControllerPlayable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStateUpdate", (animator, stateInfo, layerIndex, controller))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnStateUpdate_Animator_AnimatorStateInfo_i32_0(
        &mut self,
        animator: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
        stateInfo: crate::UnityEngine::AnimatorStateInfo,
        layerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStateUpdate", (animator, stateInfo, layerIndex))?;
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
#[cfg(feature = "UnityEngine+StateMachineBehaviour")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::StateMachineBehaviour {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
