#[cfg(feature = "UnityEngine+Yoga+MeasureOutput")]
#[repr(C)]
#[derive(Debug)]
pub struct MeasureOutput {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Yoga+MeasureOutput")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Yoga::MeasureOutput {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Yoga";
    const CLASS_NAME: &'static str = "MeasureOutput";
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
#[cfg(feature = "UnityEngine+Yoga+MeasureOutput")]
impl std::ops::Deref for crate::UnityEngine::Yoga::MeasureOutput {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Yoga+MeasureOutput")]
impl std::ops::DerefMut for crate::UnityEngine::Yoga::MeasureOutput {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Yoga+MeasureOutput")]
impl crate::UnityEngine::Yoga::MeasureOutput {
    pub fn Make(
        width: f32,
        height: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Yoga::YogaSize> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (f32, f32),
                        crate::UnityEngine::Yoga::YogaSize,
                        2usize,
                    >("Make")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Make",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Yoga::YogaSize = unsafe {
            cordl_method_info.invoke_unchecked((), (width, height))?
        };
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
