#[cfg(feature = "PS5AdvancedHapticsPlayerController")]
#[repr(C)]
#[derive(Debug)]
pub struct PS5AdvancedHapticsPlayerController {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _hapticsPlayerPool: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::HapticsAudioClipPlayer_Pool,
    >,
    pub _coroutineStarter: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ICoroutineStarter,
    >,
    pub _activePlayers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::System::ValueTuple_2<
                crate::UnityEngine::XR::XRNode,
                quest_hook::libil2cpp::Gc<
                    crate::Libraries::HM::HMLib::VR::HapticPresetSO,
                >,
            >,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::HapticsAudioClipPlayer>,
        >,
    >,
}
#[cfg(feature = "PS5AdvancedHapticsPlayerController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PS5AdvancedHapticsPlayerController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PS5AdvancedHapticsPlayerController";
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
#[cfg(feature = "PS5AdvancedHapticsPlayerController")]
impl std::ops::Deref for crate::GlobalNamespace::PS5AdvancedHapticsPlayerController {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PS5AdvancedHapticsPlayerController")]
impl std::ops::DerefMut for crate::GlobalNamespace::PS5AdvancedHapticsPlayerController {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PS5AdvancedHapticsPlayerController")]
impl crate::GlobalNamespace::PS5AdvancedHapticsPlayerController {
    pub fn CanPlayHapticPreset(
        &mut self,
        hapticPreset: quest_hook::libil2cpp::Gc<
            crate::Libraries::HM::HMLib::VR::HapticPresetSO,
        >,
        node: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Libraries::HM::HMLib::VR::HapticPresetSO,
                            >,
                            crate::UnityEngine::XR::XRNode,
                        ),
                        bool,
                        2usize,
                    >("CanPlayHapticPreset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CanPlayHapticPreset", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (hapticPreset, node))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Initialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
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
    pub fn OnHapticPlayFinishedCallback(
        &mut self,
        player: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::HapticsAudioClipPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::HapticsAudioClipPlayer,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnHapticPlayFinishedCallback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "OnHapticPlayFinishedCallback", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (player))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PlayContinuousHapticPreset(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
        hapticPreset: quest_hook::libil2cpp::Gc<
            crate::Libraries::HM::HMLib::VR::HapticPresetSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::XR::XRNode,
                            quest_hook::libil2cpp::Gc<
                                crate::Libraries::HM::HMLib::VR::HapticPresetSO,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("PlayContinuousHapticPreset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "PlayContinuousHapticPreset", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (node, hapticPreset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PlayHapticFeedback(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
        hapticPreset: quest_hook::libil2cpp::Gc<
            crate::Libraries::HM::HMLib::VR::HapticPresetSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::XR::XRNode,
                            quest_hook::libil2cpp::Gc<
                                crate::Libraries::HM::HMLib::VR::HapticPresetSO,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("PlayHapticFeedback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "PlayHapticFeedback", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (node, hapticPreset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PlayOneShotHapticPreset(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
        hapticPreset: quest_hook::libil2cpp::Gc<
            crate::Libraries::HM::HMLib::VR::HapticPresetSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::XR::XRNode,
                            quest_hook::libil2cpp::Gc<
                                crate::Libraries::HM::HMLib::VR::HapticPresetSO,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("PlayOneShotHapticPreset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "PlayOneShotHapticPreset", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (node, hapticPreset))?
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
#[cfg(feature = "PS5AdvancedHapticsPlayerController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PS5AdvancedHapticsPlayerController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PS5AdvancedHapticsPlayerController")]
impl AsRef<crate::GlobalNamespace::IHapticFeedbackPlayer>
for crate::GlobalNamespace::PS5AdvancedHapticsPlayerController {
    fn as_ref(&self) -> &crate::GlobalNamespace::IHapticFeedbackPlayer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PS5AdvancedHapticsPlayerController")]
impl AsMut<crate::GlobalNamespace::IHapticFeedbackPlayer>
for crate::GlobalNamespace::PS5AdvancedHapticsPlayerController {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IHapticFeedbackPlayer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PS5AdvancedHapticsPlayerController")]
impl AsRef<crate::Zenject::IInitializable>
for crate::GlobalNamespace::PS5AdvancedHapticsPlayerController {
    fn as_ref(&self) -> &crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PS5AdvancedHapticsPlayerController")]
impl AsMut<crate::Zenject::IInitializable>
for crate::GlobalNamespace::PS5AdvancedHapticsPlayerController {
    fn as_mut(&mut self) -> &mut crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
