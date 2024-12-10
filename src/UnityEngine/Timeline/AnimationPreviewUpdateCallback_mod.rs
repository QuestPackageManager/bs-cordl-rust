#[cfg(feature = "UnityEngine+Timeline+AnimationPreviewUpdateCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct AnimationPreviewUpdateCallback {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Output: crate::UnityEngine::Animations::AnimationPlayableOutput,
    pub m_Graph: crate::UnityEngine::Playables::PlayableGraph,
    pub m_PreviewComponents: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::Animations::IAnimationWindowPreview,
    >,
}
#[cfg(feature = "UnityEngine+Timeline+AnimationPreviewUpdateCallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Timeline::AnimationPreviewUpdateCallback => "UnityEngine.Timeline"
    ."AnimationPreviewUpdateCallback"
);
#[cfg(feature = "UnityEngine+Timeline+AnimationPreviewUpdateCallback")]
impl std::ops::Deref for crate::UnityEngine::Timeline::AnimationPreviewUpdateCallback {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationPreviewUpdateCallback")]
impl std::ops::DerefMut
for crate::UnityEngine::Timeline::AnimationPreviewUpdateCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationPreviewUpdateCallback")]
impl crate::UnityEngine::Timeline::AnimationPreviewUpdateCallback {
    pub fn Evaluate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Evaluate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FetchPreviewComponents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FetchPreviewComponents", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        output: crate::UnityEngine::Animations::AnimationPlayableOutput,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (output))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        output: crate::UnityEngine::Animations::AnimationPlayableOutput,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (output))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationPreviewUpdateCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::AnimationPreviewUpdateCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
