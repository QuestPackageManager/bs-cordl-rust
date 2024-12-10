#[cfg(feature = "Tayx+Graphy+GraphyManager")]
#[repr(C)]
#[derive(Debug)]
pub struct GraphyManager {
    __cordl_parent: crate::Tayx::Graphy::Utils::G_Singleton_1<
        *mut crate::Tayx::Graphy::GraphyManager,
    >,
    pub m_graphyMode: crate::Tayx::Graphy::GraphyManager_Mode,
    pub m_enableOnStartup: bool,
    pub m_keepAlive: bool,
    pub m_background: bool,
    pub m_backgroundColor: crate::UnityEngine::Color,
    pub m_enableHotkeys: bool,
    pub m_toggleModeKeyCode: crate::UnityEngine::InputSystem::Key,
    pub m_toggleModeCtrl: bool,
    pub m_toggleModeAlt: bool,
    pub m_toggleActiveKeyCode: crate::UnityEngine::InputSystem::Key,
    pub m_toggleActiveCtrl: bool,
    pub m_toggleActiveAlt: bool,
    pub m_graphModulePosition: crate::Tayx::Graphy::GraphyManager_ModulePosition,
    pub m_fpsModuleState: crate::Tayx::Graphy::GraphyManager_ModuleState,
    pub m_goodFpsColor: crate::UnityEngine::Color,
    pub m_goodFpsThreshold: i32,
    pub m_cautionFpsColor: crate::UnityEngine::Color,
    pub m_cautionFpsThreshold: i32,
    pub m_criticalFpsColor: crate::UnityEngine::Color,
    pub m_fpsGraphResolution: i32,
    pub m_fpsTextUpdateRate: i32,
    pub m_ramModuleState: crate::Tayx::Graphy::GraphyManager_ModuleState,
    pub m_allocatedRamColor: crate::UnityEngine::Color,
    pub m_reservedRamColor: crate::UnityEngine::Color,
    pub m_monoRamColor: crate::UnityEngine::Color,
    pub m_ramGraphResolution: i32,
    pub m_ramTextUpdateRate: i32,
    pub m_audioModuleState: crate::Tayx::Graphy::GraphyManager_ModuleState,
    pub m_findAudioListenerInCameraIfNull: crate::Tayx::Graphy::GraphyManager_LookForAudioListener,
    pub m_audioListener: *mut crate::UnityEngine::AudioListener,
    pub m_audioGraphColor: crate::UnityEngine::Color,
    pub m_audioGraphResolution: i32,
    pub m_audioTextUpdateRate: i32,
    pub m_FFTWindow: crate::UnityEngine::FFTWindow,
    pub m_spectrumSize: i32,
    pub m_advancedModulePosition: crate::Tayx::Graphy::GraphyManager_ModulePosition,
    pub m_advancedModuleState: crate::Tayx::Graphy::GraphyManager_ModuleState,
    pub m_initialized: bool,
    pub m_active: bool,
    pub m_focused: bool,
    pub m_fpsManager: *mut crate::Tayx::Graphy::Fps::G_FpsManager,
    pub m_ramManager: *mut crate::Tayx::Graphy::Ram::G_RamManager,
    pub m_audioManager: *mut crate::Tayx::Graphy::Audio::G_AudioManager,
    pub m_advancedData: *mut crate::Tayx::Graphy::Advanced::G_AdvancedData,
    pub m_fpsMonitor: *mut crate::Tayx::Graphy::Fps::G_FpsMonitor,
    pub m_ramMonitor: *mut crate::Tayx::Graphy::Ram::G_RamMonitor,
    pub m_audioMonitor: *mut crate::Tayx::Graphy::Audio::G_AudioMonitor,
    pub m_modulePresetState: crate::Tayx::Graphy::GraphyManager_ModulePreset,
}
#[cfg(feature = "Tayx+Graphy+GraphyManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Tayx::Graphy::GraphyManager => "Tayx.Graphy"
    ."GraphyManager"
);
#[cfg(feature = "Tayx+Graphy+GraphyManager")]
impl std::ops::Deref for crate::Tayx::Graphy::GraphyManager {
    type Target = crate::Tayx::Graphy::Utils::G_Singleton_1<
        *mut crate::Tayx::Graphy::GraphyManager,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Tayx+Graphy+GraphyManager")]
impl std::ops::DerefMut for crate::Tayx::Graphy::GraphyManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Tayx+Graphy+GraphyManager")]
impl crate::Tayx::Graphy::GraphyManager {
    #[cfg(feature = "Tayx+Graphy+GraphyManager+LookForAudioListener")]
    pub type LookForAudioListener = crate::Tayx::Graphy::GraphyManager_LookForAudioListener;
    #[cfg(feature = "Tayx+Graphy+GraphyManager+Mode")]
    pub type Mode = crate::Tayx::Graphy::GraphyManager_Mode;
    #[cfg(feature = "Tayx+Graphy+GraphyManager+ModulePosition")]
    pub type ModulePosition = crate::Tayx::Graphy::GraphyManager_ModulePosition;
    #[cfg(feature = "Tayx+Graphy+GraphyManager+ModulePreset")]
    pub type ModulePreset = crate::Tayx::Graphy::GraphyManager_ModulePreset;
    #[cfg(feature = "Tayx+Graphy+GraphyManager+ModuleState")]
    pub type ModuleState = crate::Tayx::Graphy::GraphyManager_ModuleState;
    #[cfg(feature = "Tayx+Graphy+GraphyManager+ModuleType")]
    pub type ModuleType = crate::Tayx::Graphy::GraphyManager_ModuleType;
    pub fn CheckFor1KeyPress(
        &mut self,
        key: crate::UnityEngine::InputSystem::Key,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CheckFor1KeyPress", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckFor2KeyPress(
        &mut self,
        key1: crate::UnityEngine::InputSystem::Key,
        key2: crate::UnityEngine::InputSystem::Key,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckFor2KeyPress", (key1, key2))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckFor3KeyPress(
        &mut self,
        key1: crate::UnityEngine::InputSystem::Key,
        key2: crate::UnityEngine::InputSystem::Key,
        key3: crate::UnityEngine::InputSystem::Key,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckFor3KeyPress", (key1, key2, key3))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckForHotkeyPresses(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckForHotkeyPresses", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Disable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Disable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Enable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Enable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnApplicationFocus(
        &mut self,
        isFocused: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnApplicationFocus", (isFocused))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshAllParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshAllParameters", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetModuleMode(
        &mut self,
        moduleType: crate::Tayx::Graphy::GraphyManager_ModuleType,
        moduleState: crate::Tayx::Graphy::GraphyManager_ModuleState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetModuleMode", (moduleType, moduleState))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetModulePosition(
        &mut self,
        moduleType: crate::Tayx::Graphy::GraphyManager_ModuleType,
        modulePosition: crate::Tayx::Graphy::GraphyManager_ModulePosition,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetModulePosition", (moduleType, modulePosition))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPreset(
        &mut self,
        modulePreset: crate::Tayx::Graphy::GraphyManager_ModulePreset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPreset", (modulePreset))?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToggleActive(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ToggleActive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToggleModes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ToggleModes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAllParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAllParameters", ())?;
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
    pub fn get_AdvancedModulePosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Tayx::Graphy::GraphyManager_ModulePosition,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Tayx::Graphy::GraphyManager_ModulePosition = __cordl_object
            .invoke("get_AdvancedModulePosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AdvancedModuleState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Tayx::Graphy::GraphyManager_ModuleState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Tayx::Graphy::GraphyManager_ModuleState = __cordl_object
            .invoke("get_AdvancedModuleState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AllocatedRam(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_AllocatedRam", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AllocatedRamColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_AllocatedRamColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AudioGraphColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_AudioGraphColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AudioGraphResolution(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_AudioGraphResolution", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AudioListener(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioListener>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioListener> = __cordl_object
            .invoke("get_AudioListener", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AudioModuleState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Tayx::Graphy::GraphyManager_ModuleState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Tayx::Graphy::GraphyManager_ModuleState = __cordl_object
            .invoke("get_AudioModuleState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AudioTextUpdateRate(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_AudioTextUpdateRate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AverageFPS(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_AverageFPS", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Background(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Background", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BackgroundColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_BackgroundColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CautionFPSColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_CautionFPSColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CautionFPSThreshold(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_CautionFPSThreshold", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CriticalFPSColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_CriticalFPSColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CurrentFPS(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_CurrentFPS", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EnableOnStartup(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_EnableOnStartup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_FftWindow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::FFTWindow> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::FFTWindow = __cordl_object
            .invoke("get_FftWindow", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_FindAudioListenerInCameraIfNull(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Tayx::Graphy::GraphyManager_LookForAudioListener,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Tayx::Graphy::GraphyManager_LookForAudioListener = __cordl_object
            .invoke("get_FindAudioListenerInCameraIfNull", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_FpsGraphResolution(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_FpsGraphResolution", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_FpsModuleState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Tayx::Graphy::GraphyManager_ModuleState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Tayx::Graphy::GraphyManager_ModuleState = __cordl_object
            .invoke("get_FpsModuleState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_FpsTextUpdateRate(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_FpsTextUpdateRate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_GoodFPSColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_GoodFPSColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_GoodFPSThreshold(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_GoodFPSThreshold", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_GraphModulePosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Tayx::Graphy::GraphyManager_ModulePosition,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Tayx::Graphy::GraphyManager_ModulePosition = __cordl_object
            .invoke("get_GraphModulePosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_GraphyMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Tayx::Graphy::GraphyManager_Mode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Tayx::Graphy::GraphyManager_Mode = __cordl_object
            .invoke("get_GraphyMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_KeepAlive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_KeepAlive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MaxDB(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_MaxDB", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MaxFPS(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_MaxFPS", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MinFPS(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_MinFPS", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MonoRam(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_MonoRam", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MonoRamColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_MonoRamColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RamGraphResolution(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_RamGraphResolution", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RamModuleState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Tayx::Graphy::GraphyManager_ModuleState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Tayx::Graphy::GraphyManager_ModuleState = __cordl_object
            .invoke("get_RamModuleState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RamTextUpdateRate(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_RamTextUpdateRate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ReservedRam(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_ReservedRam", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ReservedRamColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_ReservedRamColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Spectrum(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<f32>,
        > = __cordl_object.invoke("get_Spectrum", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SpectrumSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_SpectrumSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_AdvancedModulePosition(
        &mut self,
        value: crate::Tayx::Graphy::GraphyManager_ModulePosition,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AdvancedModulePosition", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_AdvancedModuleState(
        &mut self,
        value: crate::Tayx::Graphy::GraphyManager_ModuleState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AdvancedModuleState", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_AllocatedRamColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AllocatedRamColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_AudioGraphColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AudioGraphColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_AudioGraphResolution(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AudioGraphResolution", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_AudioListener(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioListener>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AudioListener", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_AudioModuleState(
        &mut self,
        value: crate::Tayx::Graphy::GraphyManager_ModuleState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AudioModuleState", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_AudioTextUpdateRate(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AudioTextUpdateRate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Background(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Background", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_BackgroundColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_BackgroundColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CautionFPSColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CautionFPSColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CautionFPSThreshold(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CautionFPSThreshold", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CriticalFPSColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CriticalFPSColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_FftWindow(
        &mut self,
        value: crate::UnityEngine::FFTWindow,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_FftWindow", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_FindAudioListenerInCameraIfNull(
        &mut self,
        value: crate::Tayx::Graphy::GraphyManager_LookForAudioListener,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_FindAudioListenerInCameraIfNull", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_FpsGraphResolution(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_FpsGraphResolution", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_FpsModuleState(
        &mut self,
        value: crate::Tayx::Graphy::GraphyManager_ModuleState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_FpsModuleState", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_FpsTextUpdateRate(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_FpsTextUpdateRate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_GoodFPSColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_GoodFPSColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_GoodFPSThreshold(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_GoodFPSThreshold", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_GraphModulePosition(
        &mut self,
        value: crate::Tayx::Graphy::GraphyManager_ModulePosition,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_GraphModulePosition", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_GraphyMode(
        &mut self,
        value: crate::Tayx::Graphy::GraphyManager_Mode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_GraphyMode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_MonoRamColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MonoRamColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_RamGraphResolution(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RamGraphResolution", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_RamModuleState(
        &mut self,
        value: crate::Tayx::Graphy::GraphyManager_ModuleState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RamModuleState", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_RamTextUpdateRate(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RamTextUpdateRate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ReservedRamColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ReservedRamColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SpectrumSize(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SpectrumSize", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Tayx+Graphy+GraphyManager")]
impl quest_hook::libil2cpp::ObjectType for crate::Tayx::Graphy::GraphyManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Tayx+Graphy+GraphyManager+LookForAudioListener")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphyManager_LookForAudioListener {
    ALWAYS = 0i32,
    NEVER = 2i32,
    ON_SCENE_LOAD = 1i32,
}
#[cfg(feature = "Tayx+Graphy+GraphyManager+LookForAudioListener")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Tayx::Graphy::GraphyManager_LookForAudioListener
    => "Tayx.Graphy"."GraphyManager/LookForAudioListener"
);
#[cfg(feature = "Tayx+Graphy+GraphyManager+Mode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphyManager_Mode {
    FULL = 0i32,
    LIGHT = 1i32,
}
#[cfg(feature = "Tayx+Graphy+GraphyManager+Mode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Tayx::Graphy::GraphyManager_Mode =>
    "Tayx.Graphy"."GraphyManager/Mode"
);
#[cfg(feature = "Tayx+Graphy+GraphyManager+ModulePosition")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphyManager_ModulePosition {
    BOTTOM_LEFT = 3i32,
    BOTTOM_RIGHT = 2i32,
    FREE = 4i32,
    TOP_LEFT = 1i32,
    TOP_RIGHT = 0i32,
}
#[cfg(feature = "Tayx+Graphy+GraphyManager+ModulePosition")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Tayx::Graphy::GraphyManager_ModulePosition =>
    "Tayx.Graphy"."GraphyManager/ModulePosition"
);
#[cfg(feature = "Tayx+Graphy+GraphyManager+ModulePreset")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphyManager_ModulePreset {
    FPS_BASIC = 0i32,
    FPS_BASIC_ADVANCED_FULL = 11i32,
    FPS_FULL = 2i32,
    FPS_FULL_RAM_FULL = 5i32,
    FPS_FULL_RAM_FULL_AUDIO_FULL = 9i32,
    FPS_FULL_RAM_FULL_AUDIO_FULL_ADVANCED_FULL = 10i32,
    FPS_FULL_RAM_FULL_AUDIO_TEXT = 8i32,
    FPS_FULL_RAM_TEXT = 4i32,
    FPS_FULL_RAM_TEXT_AUDIO_TEXT = 7i32,
    FPS_TEXT = 1i32,
    FPS_TEXT_RAM_TEXT = 3i32,
    FPS_TEXT_RAM_TEXT_AUDIO_TEXT = 6i32,
}
#[cfg(feature = "Tayx+Graphy+GraphyManager+ModulePreset")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Tayx::Graphy::GraphyManager_ModulePreset =>
    "Tayx.Graphy"."GraphyManager/ModulePreset"
);
#[cfg(feature = "Tayx+Graphy+GraphyManager+ModuleState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphyManager_ModuleState {
    BACKGROUND = 3i32,
    BASIC = 2i32,
    FULL = 0i32,
    OFF = 4i32,
    TEXT = 1i32,
}
#[cfg(feature = "Tayx+Graphy+GraphyManager+ModuleState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Tayx::Graphy::GraphyManager_ModuleState =>
    "Tayx.Graphy"."GraphyManager/ModuleState"
);
#[cfg(feature = "Tayx+Graphy+GraphyManager+ModuleType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphyManager_ModuleType {
    ADVANCED = 3i32,
    AUDIO = 2i32,
    FPS = 0i32,
    RAM = 1i32,
}
#[cfg(feature = "Tayx+Graphy+GraphyManager+ModuleType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Tayx::Graphy::GraphyManager_ModuleType =>
    "Tayx.Graphy"."GraphyManager/ModuleType"
);
