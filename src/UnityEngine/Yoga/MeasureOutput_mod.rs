#[cfg(feature = "UnityEngine+Yoga+MeasureOutput")]
#[repr(C)]
#[derive(Debug)]
pub struct MeasureOutput {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+Yoga+MeasureOutput")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Yoga::MeasureOutput =>
    "UnityEngine.Yoga"."MeasureOutput"
);
#[cfg(feature = "UnityEngine+Yoga+MeasureOutput")]
impl std::ops::Deref for crate::UnityEngine::Yoga::MeasureOutput {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Yoga+MeasureOutput")]
impl std::ops::DerefMut for crate::UnityEngine::Yoga::MeasureOutput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Yoga+MeasureOutput")]
impl crate::UnityEngine::Yoga::MeasureOutput {
    pub fn Make(
        width: f32,
        height: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Yoga::YogaSize> {
        let __cordl_ret: crate::UnityEngine::Yoga::YogaSize = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Make", (width, height))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Yoga+MeasureOutput")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Yoga::MeasureOutput {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
