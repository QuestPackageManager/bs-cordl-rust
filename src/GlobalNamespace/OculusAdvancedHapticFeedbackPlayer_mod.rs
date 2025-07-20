#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusAdvancedHapticFeedbackPlayer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _vrPlatformHelper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IVRPlatformHelper,
    >,
    pub _oculusVRHelper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OculusVRHelper,
    >,
    pub _hapticPlayerStatesDictionary: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::System::ValueTuple_2<
                quest_hook::libil2cpp::Gc<
                    crate::Libraries::HM::HMLib::VR::HapticPresetSO,
                >,
                crate::UnityEngine::XR::XRNode,
            >,
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer_HapticPlayerState,
            >,
        >,
    >,
    pub _hasAtLeastOneLoopingClipPlaying: bool,
    pub _isLeftHandSupported: crate::System::Nullable_1<bool>,
    pub _isRightHandSupported: crate::System::Nullable_1<bool>,
}
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OculusAdvancedHapticFeedbackPlayer";
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
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer")]
impl std::ops::Deref for crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer")]
impl std::ops::DerefMut for crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer")]
impl crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer {
    #[cfg(feature = "OculusAdvancedHapticFeedbackPlayer+HapticPlayerState")]
    pub type HapticPlayerState = crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer_HapticPlayerState;
    pub fn CanPlayHapticPreset(
        &mut self,
        hapticPreset: quest_hook::libil2cpp::Gc<
            crate::Libraries::HM::HMLib::VR::HapticPresetSO,
        >,
        node: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer as quest_hook::libil2cpp::Type>::class()
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
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer as
                    quest_hook::libil2cpp::Type > ::class(), "CanPlayHapticPreset",
                    2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (hapticPreset, node))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer as
                    quest_hook::libil2cpp::Type > ::class(), "Dispose", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetHapticClip(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
        hapticPreset: quest_hook::libil2cpp::Gc<
            crate::Libraries::HM::HMLib::VR::HapticPresetSO,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Haptics::HapticClip>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::XR::XRNode,
                    quest_hook::libil2cpp::Gc<
                        crate::Libraries::HM::HMLib::VR::HapticPresetSO,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::Oculus::Haptics::HapticClip>,
                2usize,
            >("GetHapticClip")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer as
                    quest_hook::libil2cpp::Type > ::class(), "GetHapticClip", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Haptics::HapticClip> = unsafe {
            method.invoke_unchecked(self, (node, hapticPreset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleApplicationQuitting(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleApplicationQuitting")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer as
                    quest_hook::libil2cpp::Type > ::class(), "HandleApplicationQuitting",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Initialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer as
                    quest_hook::libil2cpp::Type > ::class(), "Initialize", 0usize
                )
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
    pub fn PlayHapticFeedback(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
        hapticPreset: quest_hook::libil2cpp::Gc<
            crate::Libraries::HM::HMLib::VR::HapticPresetSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer as quest_hook::libil2cpp::Type>::class()
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
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer as
                    quest_hook::libil2cpp::Type > ::class(), "PlayHapticFeedback", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (node, hapticPreset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Tick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Tick")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer as
                    quest_hook::libil2cpp::Type > ::class(), "Tick", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer")]
impl AsRef<crate::GlobalNamespace::IHapticFeedbackPlayer>
for crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer {
    fn as_ref(&self) -> &crate::GlobalNamespace::IHapticFeedbackPlayer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer")]
impl AsMut<crate::GlobalNamespace::IHapticFeedbackPlayer>
for crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IHapticFeedbackPlayer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer")]
impl AsRef<crate::Zenject::IInitializable>
for crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer {
    fn as_ref(&self) -> &crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer")]
impl AsMut<crate::Zenject::IInitializable>
for crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer {
    fn as_mut(&mut self) -> &mut crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer")]
impl AsRef<crate::Zenject::ITickable>
for crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer {
    fn as_ref(&self) -> &crate::Zenject::ITickable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer")]
impl AsMut<crate::Zenject::ITickable>
for crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer {
    fn as_mut(&mut self) -> &mut crate::Zenject::ITickable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer+HapticPlayerState")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusAdvancedHapticFeedbackPlayer_HapticPlayerState {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub player: quest_hook::libil2cpp::Gc<crate::Oculus::Haptics::HapticClipPlayer>,
    pub lastFrameTriggered: i32,
    pub isPlayingLoopingClip: bool,
}
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer+HapticPlayerState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer_HapticPlayerState {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OculusAdvancedHapticFeedbackPlayer/HapticPlayerState";
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
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer+HapticPlayerState")]
impl std::ops::Deref
for crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer_HapticPlayerState {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer+HapticPlayerState")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer_HapticPlayerState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer+HapticPlayerState")]
impl crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer_HapticPlayerState {
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer_HapticPlayerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer_HapticPlayerState
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OculusAdvancedHapticFeedbackPlayer+HapticPlayerState")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OculusAdvancedHapticFeedbackPlayer_HapticPlayerState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
