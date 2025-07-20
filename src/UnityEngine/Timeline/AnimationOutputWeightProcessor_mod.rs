#[cfg(feature = "UnityEngine+Timeline+AnimationOutputWeightProcessor")]
#[repr(C)]
#[derive(Debug)]
pub struct AnimationOutputWeightProcessor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Output: crate::UnityEngine::Animations::AnimationPlayableOutput,
    pub m_MotionXPlayable: crate::UnityEngine::Animations::AnimationMotionXToDeltaPlayable,
    pub m_Mixers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::Timeline::AnimationOutputWeightProcessor_WeightInfo,
        >,
    >,
}
#[cfg(feature = "UnityEngine+Timeline+AnimationOutputWeightProcessor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Timeline::AnimationOutputWeightProcessor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "AnimationOutputWeightProcessor";
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
#[cfg(feature = "UnityEngine+Timeline+AnimationOutputWeightProcessor")]
impl std::ops::Deref for crate::UnityEngine::Timeline::AnimationOutputWeightProcessor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationOutputWeightProcessor")]
impl std::ops::DerefMut
for crate::UnityEngine::Timeline::AnimationOutputWeightProcessor {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Evaluate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Evaluate", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindMixers_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("FindMixers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "FindMixers", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindMixers_Playable_i32_Playable1(
        &mut self,
        parent: crate::UnityEngine::Playables::Playable,
        port: i32,
        node: crate::UnityEngine::Playables::Playable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Playables::Playable,
                            i32,
                            crate::UnityEngine::Playables::Playable,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("FindMixers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "FindMixers", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (parent, port, node))?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::UnityEngine::Animations::AnimationPlayableOutput),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (output))?
        };
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimationOutputWeightProcessor_WeightInfo {
    pub mixer: crate::UnityEngine::Playables::Playable,
    pub parentMixer: crate::UnityEngine::Playables::Playable,
    pub port: i32,
}
#[cfg(feature = "UnityEngine+Timeline+AnimationOutputWeightProcessor+WeightInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Timeline::AnimationOutputWeightProcessor_WeightInfo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "AnimationOutputWeightProcessor/WeightInfo";
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
#[cfg(feature = "UnityEngine+Timeline+AnimationOutputWeightProcessor+WeightInfo")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Timeline::AnimationOutputWeightProcessor_WeightInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationOutputWeightProcessor+WeightInfo")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Timeline::AnimationOutputWeightProcessor_WeightInfo {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+Timeline+AnimationOutputWeightProcessor+WeightInfo")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Timeline::AnimationOutputWeightProcessor_WeightInfo {
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
#[cfg(feature = "UnityEngine+Timeline+AnimationOutputWeightProcessor+WeightInfo")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Timeline::AnimationOutputWeightProcessor_WeightInfo {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
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
