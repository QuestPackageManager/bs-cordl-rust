#[cfg(feature = "GhostEffectBehaviour")]
#[repr(C)]
#[derive(Debug)]
pub struct GhostEffectBehaviour {
    __cordl_parent: crate::UnityEngine::Playables::PlayableBehaviour,
    pub alphaCurve: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationCurve>,
    pub sizeCurve: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationCurve>,
    pub distanceCurve: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationCurve>,
    pub _distanceMultiplier: crate::UnityEngine::Vector3,
    pub _useStartTransform: bool,
    pub _useEndTransform: bool,
    pub _startLocalPosition: crate::UnityEngine::Vector3,
    pub _startTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _endLocalPosition: crate::UnityEngine::Vector3,
    pub _endTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _positionEasing: crate::GlobalNamespace::EaseType,
    pub _endBehavior: crate::GlobalNamespace::GhostEffectBehaviour_EndBehavior,
    pub progress: f32,
    pub textMeshPros: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshPro>,
        >,
    >,
    pub _canvasGroups: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::CanvasGroup>,
        >,
    >,
    pub _ghostEffectType: crate::GlobalNamespace::GhostEffectBehaviour_GhostEffectType,
    pub _ghostEffectTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _direction: crate::UnityEngine::Vector3,
    pub _finished: bool,
}
#[cfg(feature = "GhostEffectBehaviour")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GhostEffectBehaviour {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GhostEffectBehaviour";
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
#[cfg(feature = "GhostEffectBehaviour")]
impl std::ops::Deref for crate::GlobalNamespace::GhostEffectBehaviour {
    type Target = crate::UnityEngine::Playables::PlayableBehaviour;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GhostEffectBehaviour")]
impl std::ops::DerefMut for crate::GlobalNamespace::GhostEffectBehaviour {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GhostEffectBehaviour")]
impl crate::GlobalNamespace::GhostEffectBehaviour {
    #[cfg(feature = "GhostEffectBehaviour+EndBehavior")]
    pub type EndBehavior = crate::GlobalNamespace::GhostEffectBehaviour_EndBehavior;
    #[cfg(feature = "GhostEffectBehaviour+GhostEffectType")]
    pub type GhostEffectType = crate::GlobalNamespace::GhostEffectBehaviour_GhostEffectType;
    pub fn EnableObjects(
        &mut self,
        on: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (bool),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EnableObjects")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "EnableObjects", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (on))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnBehaviourPlay(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
        info: crate::UnityEngine::Playables::FrameData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Playables::Playable,
                            crate::UnityEngine::Playables::FrameData,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("OnBehaviourPlay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "OnBehaviourPlay", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (playable, info))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessFrame(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
        info: crate::UnityEngine::Playables::FrameData,
        playerData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Playables::Playable,
                            crate::UnityEngine::Playables::FrameData,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ProcessFrame")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ProcessFrame", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (playable, info, playerData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GhostEffectBehaviour")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::GhostEffectBehaviour {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GhostEffectBehaviour+EndBehavior")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GhostEffectBehaviour_EndBehavior {
    #[default]
    DisableAll = 0i32,
    DisableCopies = 1i32,
    Nothing = 2i32,
}
#[cfg(feature = "GhostEffectBehaviour+EndBehavior")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GhostEffectBehaviour_EndBehavior {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GhostEffectBehaviour/EndBehavior";
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
#[cfg(feature = "GhostEffectBehaviour+EndBehavior")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::GhostEffectBehaviour_EndBehavior {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "GhostEffectBehaviour+EndBehavior")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::GhostEffectBehaviour_EndBehavior {
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
#[cfg(feature = "GhostEffectBehaviour+EndBehavior")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::GhostEffectBehaviour_EndBehavior {
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
#[cfg(feature = "GhostEffectBehaviour+EndBehavior")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::GhostEffectBehaviour_EndBehavior {
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
#[cfg(feature = "GhostEffectBehaviour+GhostEffectType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GhostEffectBehaviour_GhostEffectType {
    #[default]
    Canvas = 1i32,
    TextMeshPro = 0i32,
}
#[cfg(feature = "GhostEffectBehaviour+GhostEffectType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GhostEffectBehaviour_GhostEffectType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GhostEffectBehaviour/GhostEffectType";
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
#[cfg(feature = "GhostEffectBehaviour+GhostEffectType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::GhostEffectBehaviour_GhostEffectType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "GhostEffectBehaviour+GhostEffectType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::GhostEffectBehaviour_GhostEffectType {
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
#[cfg(feature = "GhostEffectBehaviour+GhostEffectType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::GhostEffectBehaviour_GhostEffectType {
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
#[cfg(feature = "GhostEffectBehaviour+GhostEffectType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::GhostEffectBehaviour_GhostEffectType {
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
