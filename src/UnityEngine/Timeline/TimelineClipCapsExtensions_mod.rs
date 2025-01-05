#[cfg(feature = "UnityEngine+Timeline+TimelineClipCapsExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TimelineClipCapsExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClipCapsExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Timeline::TimelineClipCapsExtensions => "UnityEngine.Timeline"
    ."TimelineClipCapsExtensions"
);
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasAll", (caps, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasAny(
        caps: crate::UnityEngine::Timeline::ClipCaps,
        flags: crate::UnityEngine::Timeline::ClipCaps,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasAny", (caps, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SupportsBlending(
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SupportsBlending", (clip))?;
        Ok(__cordl_ret.into())
    }
    pub fn SupportsClipIn(
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SupportsClipIn", (clip))?;
        Ok(__cordl_ret.into())
    }
    pub fn SupportsExtrapolation(
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SupportsExtrapolation", (clip))?;
        Ok(__cordl_ret.into())
    }
    pub fn SupportsLooping(
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SupportsLooping", (clip))?;
        Ok(__cordl_ret.into())
    }
    pub fn SupportsSpeedMultiplier(
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SupportsSpeedMultiplier", (clip))?;
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
