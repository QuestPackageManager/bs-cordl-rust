#[cfg(feature = "UnityEngine+Timeline+TimelineClipCapsExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TimelineClipCapsExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClipCapsExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Timeline::TimelineClipCapsExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "TimelineClipCapsExtensions";
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
#[cfg(feature = "UnityEngine+Timeline+TimelineClipCapsExtensions")]
impl std::ops::Deref for crate::UnityEngine::Timeline::TimelineClipCapsExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClipCapsExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::TimelineClipCapsExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClipCapsExtensions")]
impl crate::UnityEngine::Timeline::TimelineClipCapsExtensions {
    pub fn HasAll(
        caps: crate::UnityEngine::Timeline::ClipCaps,
        flags: crate::UnityEngine::Timeline::ClipCaps,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Timeline::TimelineClipCapsExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::Timeline::ClipCaps,
                    crate::UnityEngine::Timeline::ClipCaps,
                ),
                bool,
                2usize,
            >("HasAll")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Timeline::TimelineClipCapsExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "HasAll", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (caps, flags))? };
        Ok(__cordl_ret.into())
    }
    pub fn HasAny(
        caps: crate::UnityEngine::Timeline::ClipCaps,
        flags: crate::UnityEngine::Timeline::ClipCaps,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Timeline::TimelineClipCapsExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::Timeline::ClipCaps,
                    crate::UnityEngine::Timeline::ClipCaps,
                ),
                bool,
                2usize,
            >("HasAny")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Timeline::TimelineClipCapsExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "HasAny", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (caps, flags))? };
        Ok(__cordl_ret.into())
    }
    pub fn SupportsBlending(
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Timeline::TimelineClipCapsExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>),
                bool,
                1usize,
            >("SupportsBlending")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Timeline::TimelineClipCapsExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "SupportsBlending", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (clip))? };
        Ok(__cordl_ret.into())
    }
    pub fn SupportsClipIn(
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Timeline::TimelineClipCapsExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>),
                bool,
                1usize,
            >("SupportsClipIn")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Timeline::TimelineClipCapsExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "SupportsClipIn", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (clip))? };
        Ok(__cordl_ret.into())
    }
    pub fn SupportsExtrapolation(
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Timeline::TimelineClipCapsExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>),
                bool,
                1usize,
            >("SupportsExtrapolation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Timeline::TimelineClipCapsExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "SupportsExtrapolation",
                    1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (clip))? };
        Ok(__cordl_ret.into())
    }
    pub fn SupportsLooping(
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Timeline::TimelineClipCapsExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>),
                bool,
                1usize,
            >("SupportsLooping")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Timeline::TimelineClipCapsExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "SupportsLooping", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (clip))? };
        Ok(__cordl_ret.into())
    }
    pub fn SupportsSpeedMultiplier(
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Timeline::TimelineClipCapsExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>),
                bool,
                1usize,
            >("SupportsSpeedMultiplier")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Timeline::TimelineClipCapsExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "SupportsSpeedMultiplier",
                    1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (clip))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClipCapsExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::TimelineClipCapsExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
