#[cfg(feature = "RumbleHapticFeedbackPlayer")]
#[repr(C)]
#[derive(Debug)]
pub struct RumbleHapticFeedbackPlayer {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _vrPlatformHelper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IVRPlatformHelper,
    >,
    pub _rumblesByNode: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::XR::XRNode,
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::Dictionary_2<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::RumbleHapticFeedbackPlayer_RumbleData,
                    >,
                >,
            >,
        >,
    >,
}
#[cfg(feature = "RumbleHapticFeedbackPlayer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::RumbleHapticFeedbackPlayer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "RumbleHapticFeedbackPlayer";
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
#[cfg(feature = "RumbleHapticFeedbackPlayer")]
impl std::ops::Deref for crate::GlobalNamespace::RumbleHapticFeedbackPlayer {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RumbleHapticFeedbackPlayer")]
impl std::ops::DerefMut for crate::GlobalNamespace::RumbleHapticFeedbackPlayer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RumbleHapticFeedbackPlayer")]
impl crate::GlobalNamespace::RumbleHapticFeedbackPlayer {
    pub const kContinuousRumbleFrameDuration: f32 = 0.016666668f32;
    #[cfg(feature = "RumbleHapticFeedbackPlayer+RumbleData")]
    pub type RumbleData = crate::GlobalNamespace::RumbleHapticFeedbackPlayer_RumbleData;
    pub fn CanPlayHapticPreset(
        &mut self,
        hapticPreset: quest_hook::libil2cpp::Gc<
            crate::Libraries::HM::HMLib::VR::HapticPresetSO,
        >,
        node: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CanPlayHapticPreset", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (hapticPreset, node))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRumble(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
        preset: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RumbleHapticFeedbackPlayer_RumbleData,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::XR::XRNode,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::RumbleHapticFeedbackPlayer_RumbleData,
                        >,
                        2usize,
                    >("GetRumble")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRumble", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RumbleHapticFeedbackPlayer_RumbleData,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (node, preset))? };
        Ok(__cordl_ret.into())
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("LateUpdate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LateUpdate", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
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
    pub fn PlayHapticFeedback(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
        hapticPreset: quest_hook::libil2cpp::Gc<
            crate::Libraries::HM::HMLib::VR::HapticPresetSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PlayHapticFeedback", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (node, hapticPreset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateRumbles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("UpdateRumbles")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateRumbles", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "RumbleHapticFeedbackPlayer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::RumbleHapticFeedbackPlayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "RumbleHapticFeedbackPlayer")]
impl AsRef<crate::GlobalNamespace::IHapticFeedbackPlayer>
for crate::GlobalNamespace::RumbleHapticFeedbackPlayer {
    fn as_ref(&self) -> &crate::GlobalNamespace::IHapticFeedbackPlayer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "RumbleHapticFeedbackPlayer")]
impl AsMut<crate::GlobalNamespace::IHapticFeedbackPlayer>
for crate::GlobalNamespace::RumbleHapticFeedbackPlayer {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IHapticFeedbackPlayer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "RumbleHapticFeedbackPlayer+RumbleData")]
#[repr(C)]
#[derive(Debug)]
pub struct RumbleHapticFeedbackPlayer_RumbleData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub active: bool,
    pub continuous: bool,
    pub strength: f32,
    pub endTime: f32,
    pub frequency: f32,
}
#[cfg(feature = "RumbleHapticFeedbackPlayer+RumbleData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::RumbleHapticFeedbackPlayer_RumbleData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "RumbleHapticFeedbackPlayer/RumbleData";
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
#[cfg(feature = "RumbleHapticFeedbackPlayer+RumbleData")]
impl std::ops::Deref for crate::GlobalNamespace::RumbleHapticFeedbackPlayer_RumbleData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RumbleHapticFeedbackPlayer+RumbleData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::RumbleHapticFeedbackPlayer_RumbleData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RumbleHapticFeedbackPlayer+RumbleData")]
impl crate::GlobalNamespace::RumbleHapticFeedbackPlayer_RumbleData {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "RumbleHapticFeedbackPlayer+RumbleData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::RumbleHapticFeedbackPlayer_RumbleData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
