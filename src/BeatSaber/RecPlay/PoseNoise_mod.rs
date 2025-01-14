#[cfg(feature = "BeatSaber+RecPlay+PoseNoise")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PoseNoise {
    pub frequency: f32,
    pub _cordl_move: f32,
    pub rotate: f32,
}
#[cfg(feature = "BeatSaber+RecPlay+PoseNoise")]
unsafe impl quest_hook::libil2cpp::Type for crate::BeatSaber::RecPlay::PoseNoise {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatSaber.RecPlay";
    const CLASS_NAME: &'static str = "PoseNoise";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "BeatSaber+RecPlay+PoseNoise")]
unsafe impl quest_hook::libil2cpp::Argument for crate::BeatSaber::RecPlay::PoseNoise {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "BeatSaber+RecPlay+PoseNoise")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::BeatSaber::RecPlay::PoseNoise {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "BeatSaber+RecPlay+PoseNoise")]
unsafe impl quest_hook::libil2cpp::Returned for crate::BeatSaber::RecPlay::PoseNoise {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "BeatSaber+RecPlay+PoseNoise")]
unsafe impl quest_hook::libil2cpp::Return for crate::BeatSaber::RecPlay::PoseNoise {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "BeatSaber+RecPlay+PoseNoise")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::RecPlay::PoseNoise {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+RecPlay+PoseNoise")]
impl crate::BeatSaber::RecPlay::PoseNoise {
    pub fn Sample(
        &mut self,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32), crate::UnityEngine::Pose, 1usize>("Sample")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Sample", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Pose = unsafe {
            method.invoke_unchecked(self, (_cordl_time))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SampleLemniscateOfBernoulli(
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32),
                crate::UnityEngine::Vector2,
                1usize,
            >("SampleLemniscateOfBernoulli")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SampleLemniscateOfBernoulli", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked((), (_cordl_time))
        };
        Ok(__cordl_ret.into())
    }
}
