#[cfg(feature = "UnityEngine+Timeline+AnimationOutputWeightProcessor")]
#[repr(C)]
#[derive(Debug)]
pub struct AnimationOutputWeightProcessor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Output: crate::UnityEngine::Animations::AnimationPlayableOutput,
    pub m_MotionXPlayable: crate::UnityEngine::Animations::AnimationMotionXToDeltaPlayable,
    pub m_Mixers: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::Timeline::AnimationOutputWeightProcessor_WeightInfo,
    >,
}
#[cfg(feature = "UnityEngine+Timeline+AnimationOutputWeightProcessor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Timeline::AnimationOutputWeightProcessor => "UnityEngine.Timeline"
    ."AnimationOutputWeightProcessor"
);
#[cfg(feature = "UnityEngine+Timeline+AnimationOutputWeightProcessor")]
impl std::ops::Deref for crate::UnityEngine::Timeline::AnimationOutputWeightProcessor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationOutputWeightProcessor")]
impl std::ops::DerefMut
for crate::UnityEngine::Timeline::AnimationOutputWeightProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationOutputWeightProcessor")]
impl crate::UnityEngine::Timeline::AnimationOutputWeightProcessor {
    #[cfg(feature = "UnityEngine+Timeline+AnimationOutputWeightProcessor+WeightInfo")]
    pub type WeightInfo = crate::UnityEngine::Timeline::AnimationOutputWeightProcessor_WeightInfo;
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
    pub fn FindMixers_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FindMixers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FindMixers_Playable_i32_Playable1(
        &mut self,
        parent: crate::UnityEngine::Playables::Playable,
        port: i32,
        node: crate::UnityEngine::Playables::Playable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FindMixers", (parent, port, node))?;
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
#[cfg(feature = "UnityEngine+Timeline+AnimationOutputWeightProcessor")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::AnimationOutputWeightProcessor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationOutputWeightProcessor")]
impl AsRef<crate::UnityEngine::Timeline::ITimelineEvaluateCallback>
for crate::UnityEngine::Timeline::AnimationOutputWeightProcessor {
    fn as_ref(&self) -> &crate::UnityEngine::Timeline::ITimelineEvaluateCallback {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationOutputWeightProcessor")]
impl AsMut<crate::UnityEngine::Timeline::ITimelineEvaluateCallback>
for crate::UnityEngine::Timeline::AnimationOutputWeightProcessor {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::Timeline::ITimelineEvaluateCallback {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationOutputWeightProcessor+WeightInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AnimationOutputWeightProcessor_WeightInfo {
    pub mixer: crate::UnityEngine::Playables::Playable,
    pub parentMixer: crate::UnityEngine::Playables::Playable,
    pub port: i32,
}
#[cfg(feature = "UnityEngine+Timeline+AnimationOutputWeightProcessor+WeightInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Timeline::AnimationOutputWeightProcessor_WeightInfo =>
    "UnityEngine.Timeline"."AnimationOutputWeightProcessor/WeightInfo"
);
#[cfg(feature = "UnityEngine+Timeline+AnimationOutputWeightProcessor+WeightInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Timeline::AnimationOutputWeightProcessor_WeightInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationOutputWeightProcessor+WeightInfo")]
impl crate::UnityEngine::Timeline::AnimationOutputWeightProcessor_WeightInfo {}
