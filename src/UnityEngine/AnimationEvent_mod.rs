#[cfg(feature = "UnityEngine+AnimationEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct AnimationEvent {
    __cordl_parent: crate::System::Object,
    pub m_Time: f32,
    pub m_FunctionName: *mut crate::System::String,
    pub m_StringParameter: *mut crate::System::String,
    pub m_ObjectReferenceParameter: *mut crate::UnityEngine::Object,
    pub m_FloatParameter: f32,
    pub m_IntParameter: i32,
    pub m_MessageOptions: i32,
    pub m_Source: crate::UnityEngine::AnimationEventSource,
    pub m_StateSender: *mut crate::UnityEngine::AnimationState,
    pub m_AnimatorStateInfo: crate::UnityEngine::AnimatorStateInfo,
    pub m_AnimatorClipInfo: crate::UnityEngine::AnimatorClipInfo,
}
#[cfg(feature = "UnityEngine+AnimationEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AnimationEvent => "UnityEngine"
    ."AnimationEvent"
);
#[cfg(feature = "UnityEngine+AnimationEvent")]
impl std::ops::Deref for crate::UnityEngine::AnimationEvent {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AnimationEvent")]
impl std::ops::DerefMut for crate::UnityEngine::AnimationEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AnimationEvent")]
impl crate::UnityEngine::AnimationEvent {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "UnityEngine+AnimationEvent")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AnimationEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}