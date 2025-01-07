#[cfg(feature = "UnityEngine+InputSystem+InputSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct InputSettings {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub m_SupportedDevices: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub m_UpdateMode: crate::UnityEngine::InputSystem::InputSettings_UpdateMode,
    pub m_MaxEventBytesPerUpdate: i32,
    pub m_MaxQueuedEventsPerUpdate: i32,
    pub m_CompensateForScreenOrientation: bool,
    pub m_BackgroundBehavior: crate::UnityEngine::InputSystem::InputSettings_BackgroundBehavior,
    pub m_EditorInputBehaviorInPlayMode: crate::UnityEngine::InputSystem::InputSettings_EditorInputBehaviorInPlayMode,
    pub m_DefaultDeadzoneMin: f32,
    pub m_DefaultDeadzoneMax: f32,
    pub m_DefaultButtonPressPoint: f32,
    pub m_ButtonReleaseThreshold: f32,
    pub m_DefaultTapTime: f32,
    pub m_DefaultSlowTapTime: f32,
    pub m_DefaultHoldTime: f32,
    pub m_TapRadius: f32,
    pub m_MultiTapDelayTime: f32,
    pub m_DisableRedundantEventsMerging: bool,
    pub m_ShortcutKeysConsumeInputs: bool,
    pub m_FeatureFlags: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+InputSettings")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputSettings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputSettings";
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
#[cfg(feature = "UnityEngine+InputSystem+InputSettings")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputSettings {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputSettings")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::InputSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputSettings")]
impl crate::UnityEngine::InputSystem::InputSettings {
    pub const s_OldUnsupportedFixedAndDynamicUpdateSetting: i32 = 0i32;
    #[cfg(feature = "UnityEngine+InputSystem+InputSettings+BackgroundBehavior")]
    pub type BackgroundBehavior = crate::UnityEngine::InputSystem::InputSettings_BackgroundBehavior;
    #[cfg(
        feature = "UnityEngine+InputSystem+InputSettings+EditorInputBehaviorInPlayMode"
    )]
    pub type EditorInputBehaviorInPlayMode = crate::UnityEngine::InputSystem::InputSettings_EditorInputBehaviorInPlayMode;
    #[cfg(feature = "UnityEngine+InputSystem+InputSettings+UpdateMode")]
    pub type UpdateMode = crate::UnityEngine::InputSystem::InputSettings_UpdateMode;
    pub fn IsFeatureEnabled(
        &mut self,
        featureName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsFeatureEnabled", (featureName))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnChange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnChange", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetInternalFeatureFlag(
        &mut self,
        featureName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        enabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetInternalFeatureFlag", (featureName, enabled))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_backgroundBehavior(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputSettings_BackgroundBehavior,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::InputSettings_BackgroundBehavior = __cordl_object
            .invoke("get_backgroundBehavior", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_buttonReleaseThreshold(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_buttonReleaseThreshold", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_compensateForScreenOrientation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_compensateForScreenOrientation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultButtonPressPoint(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_defaultButtonPressPoint", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultDeadzoneMax(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_defaultDeadzoneMax", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultDeadzoneMin(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_defaultDeadzoneMin", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultHoldTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_defaultHoldTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultSlowTapTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_defaultSlowTapTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultTapTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_defaultTapTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_disableRedundantEventsMerging(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_disableRedundantEventsMerging", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_editorInputBehaviorInPlayMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputSettings_EditorInputBehaviorInPlayMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::InputSettings_EditorInputBehaviorInPlayMode = __cordl_object
            .invoke("get_editorInputBehaviorInPlayMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_filterNoiseOnCurrent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_filterNoiseOnCurrent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxEventBytesPerUpdate(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_maxEventBytesPerUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxQueuedEventsPerUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_maxQueuedEventsPerUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_multiTapDelayTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_multiTapDelayTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_shortcutKeysConsumeInput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_shortcutKeysConsumeInput", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_supportedDevices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("get_supportedDevices", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_tapRadius(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_tapRadius", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_updateMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputSettings_UpdateMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::InputSettings_UpdateMode = __cordl_object
            .invoke("get_updateMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_backgroundBehavior(
        &mut self,
        value: crate::UnityEngine::InputSystem::InputSettings_BackgroundBehavior,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_backgroundBehavior", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_buttonReleaseThreshold(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_buttonReleaseThreshold", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_compensateForScreenOrientation(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_compensateForScreenOrientation", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultButtonPressPoint(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_defaultButtonPressPoint", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultDeadzoneMax(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_defaultDeadzoneMax", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultDeadzoneMin(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_defaultDeadzoneMin", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultHoldTime(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_defaultHoldTime", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultSlowTapTime(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_defaultSlowTapTime", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultTapTime(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_defaultTapTime", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_disableRedundantEventsMerging(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_disableRedundantEventsMerging", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_editorInputBehaviorInPlayMode(
        &mut self,
        value: crate::UnityEngine::InputSystem::InputSettings_EditorInputBehaviorInPlayMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_editorInputBehaviorInPlayMode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_filterNoiseOnCurrent(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_filterNoiseOnCurrent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maxEventBytesPerUpdate(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maxEventBytesPerUpdate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maxQueuedEventsPerUpdate(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maxQueuedEventsPerUpdate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_multiTapDelayTime(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_multiTapDelayTime", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_shortcutKeysConsumeInput(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_shortcutKeysConsumeInput", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_supportedDevices(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_supportedDevices", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_tapRadius(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_tapRadius", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_updateMode(
        &mut self,
        value: crate::UnityEngine::InputSystem::InputSettings_UpdateMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_updateMode", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputSettings+BackgroundBehavior")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputSettings_BackgroundBehavior {
    #[default]
    IgnoreFocus = 2i32,
    ResetAndDisableAllDevices = 1i32,
    ResetAndDisableNonBackgroundDevices = 0i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputSettings+BackgroundBehavior")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputSettings_BackgroundBehavior {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "BackgroundBehavior";
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
#[cfg(feature = "UnityEngine+InputSystem+InputSettings+BackgroundBehavior")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::InputSettings_BackgroundBehavior {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputSettings+BackgroundBehavior")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::InputSettings_BackgroundBehavior {
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
#[cfg(feature = "UnityEngine+InputSystem+InputSettings+BackgroundBehavior")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::InputSettings_BackgroundBehavior {
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
#[cfg(feature = "UnityEngine+InputSystem+InputSettings+BackgroundBehavior")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::InputSettings_BackgroundBehavior {
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
#[cfg(feature = "UnityEngine+InputSystem+InputSettings+EditorInputBehaviorInPlayMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputSettings_EditorInputBehaviorInPlayMode {
    #[default]
    AllDeviceInputAlwaysGoesToGameView = 2i32,
    AllDevicesRespectGameViewFocus = 1i32,
    PointersAndKeyboardsRespectGameViewFocus = 0i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputSettings+EditorInputBehaviorInPlayMode")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputSettings_EditorInputBehaviorInPlayMode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "EditorInputBehaviorInPlayMode";
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
#[cfg(feature = "UnityEngine+InputSystem+InputSettings+EditorInputBehaviorInPlayMode")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::InputSettings_EditorInputBehaviorInPlayMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputSettings+EditorInputBehaviorInPlayMode")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::InputSettings_EditorInputBehaviorInPlayMode {
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
#[cfg(feature = "UnityEngine+InputSystem+InputSettings+EditorInputBehaviorInPlayMode")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::InputSettings_EditorInputBehaviorInPlayMode {
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
#[cfg(feature = "UnityEngine+InputSystem+InputSettings+EditorInputBehaviorInPlayMode")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::InputSettings_EditorInputBehaviorInPlayMode {
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
#[cfg(feature = "UnityEngine+InputSystem+InputSettings+UpdateMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputSettings_UpdateMode {
    #[default]
    ProcessEventsInDynamicUpdate = 1i32,
    ProcessEventsInFixedUpdate = 2i32,
    ProcessEventsManually = 3i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputSettings+UpdateMode")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputSettings_UpdateMode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "UpdateMode";
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
#[cfg(feature = "UnityEngine+InputSystem+InputSettings+UpdateMode")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::InputSettings_UpdateMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputSettings+UpdateMode")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::InputSettings_UpdateMode {
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
#[cfg(feature = "UnityEngine+InputSystem+InputSettings+UpdateMode")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::InputSettings_UpdateMode {
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
#[cfg(feature = "UnityEngine+InputSystem+InputSettings+UpdateMode")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::InputSettings_UpdateMode {
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
