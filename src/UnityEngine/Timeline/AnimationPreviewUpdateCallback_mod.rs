#[cfg(feature = "UnityEngine+Timeline+AnimationPreviewUpdateCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct AnimationPreviewUpdateCallback {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Output: crate::UnityEngine::Animations::AnimationPlayableOutput,
    pub m_Graph: crate::UnityEngine::Playables::PlayableGraph,
    pub m_PreviewComponents: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Animations::IAnimationWindowPreview,
            >,
        >,
    >,
}
#[cfg(feature = "UnityEngine+Timeline+AnimationPreviewUpdateCallback")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Timeline::AnimationPreviewUpdateCallback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "AnimationPreviewUpdateCallback";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Evaluate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Evaluate", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn FetchPreviewComponents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("FetchPreviewComponents")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FetchPreviewComponents", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Animations::AnimationPlayableOutput),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (output))
        };
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
#[cfg(feature = "UnityEngine+Timeline+AnimationPreviewUpdateCallback")]
impl AsRef<crate::UnityEngine::Timeline::ITimelineEvaluateCallback>
for crate::UnityEngine::Timeline::AnimationPreviewUpdateCallback {
    fn as_ref(&self) -> &crate::UnityEngine::Timeline::ITimelineEvaluateCallback {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationPreviewUpdateCallback")]
impl AsMut<crate::UnityEngine::Timeline::ITimelineEvaluateCallback>
for crate::UnityEngine::Timeline::AnimationPreviewUpdateCallback {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::Timeline::ITimelineEvaluateCallback {
        unsafe { std::mem::transmute(self) }
    }
}
