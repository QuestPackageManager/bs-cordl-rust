#[cfg(feature = "cordl_class_UnityEngine+Rendering+APVLeakReductionModeParameter")]
#[repr(C)]
#[derive(Debug)]
pub struct APVLeakReductionModeParameter {
    __cordl_parent: crate::UnityEngine::Rendering::VolumeParameter_1<
        crate::UnityEngine::Rendering::APVLeakReductionMode,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+APVLeakReductionModeParameter")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::APVLeakReductionModeParameter
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "APVLeakReductionModeParameter";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+Rendering+APVLeakReductionModeParameter")]
impl std::ops::Deref for crate::UnityEngine::Rendering::APVLeakReductionModeParameter {
    type Target = crate::UnityEngine::Rendering::VolumeParameter_1<
        crate::UnityEngine::Rendering::APVLeakReductionMode,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+APVLeakReductionModeParameter")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::APVLeakReductionModeParameter {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+APVLeakReductionModeParameter")]
impl crate::UnityEngine::Rendering::APVLeakReductionModeParameter {
    pub fn New(
        value: crate::UnityEngine::Rendering::APVLeakReductionMode,
        overrideState: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (value, overrideState))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        value: crate::UnityEngine::Rendering::APVLeakReductionMode,
        overrideState: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::APVLeakReductionMode, bool),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value, overrideState))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+APVLeakReductionModeParameter")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::APVLeakReductionModeParameter
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
