#[cfg(feature = "UnityEngine+Timeline+TimelineClipExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TimelineClipExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClipExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Timeline::TimelineClipExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "TimelineClipExtensions";
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
#[cfg(feature = "UnityEngine+Timeline+TimelineClipExtensions")]
impl std::ops::Deref for crate::UnityEngine::Timeline::TimelineClipExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClipExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::TimelineClipExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClipExtensions")]
impl crate::UnityEngine::Timeline::TimelineClipExtensions {
    pub fn MoveToTrack(
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
        destinationTrack: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TrackAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Timeline::TimelineClip,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Timeline::TrackAsset,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("MoveToTrack")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "MoveToTrack", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (clip, destinationTrack))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MoveToTrack_Impl(
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
        destinationTrack: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TrackAsset,
        >,
        asset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        parentTrack: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Timeline::TimelineClip,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Timeline::TrackAsset,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Timeline::TrackAsset,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("MoveToTrack_Impl")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "MoveToTrack_Impl", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (clip, destinationTrack, asset, parentTrack))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryMoveToTrack(
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
        destinationTrack: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TrackAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Timeline::TimelineClip,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Timeline::TrackAsset,
                            >,
                        ),
                        bool,
                        2usize,
                    >("TryMoveToTrack")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "TryMoveToTrack", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (clip, destinationTrack))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClipExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::TimelineClipExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
