#[cfg(feature = "UnityEngine+Timeline+WeightUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct WeightUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Timeline+WeightUtility")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Timeline::WeightUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "WeightUtility";
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
#[cfg(feature = "UnityEngine+Timeline+WeightUtility")]
impl std::ops::Deref for crate::UnityEngine::Timeline::WeightUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+WeightUtility")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::WeightUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+WeightUtility")]
impl crate::UnityEngine::Timeline::WeightUtility {
    pub fn NormalizeMixer(
        mixer: crate::UnityEngine::Playables::Playable,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Timeline::WeightUtility as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Playables::Playable),
                f32,
                1usize,
            >("NormalizeMixer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Timeline::WeightUtility as
                    quest_hook::libil2cpp::Type > ::class(), "NormalizeMixer", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (mixer))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+WeightUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Timeline::WeightUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
