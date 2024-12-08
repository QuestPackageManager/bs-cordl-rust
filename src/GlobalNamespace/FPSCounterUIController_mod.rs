#[cfg(feature = "FPSCounterUIController")]
#[repr(C)]
#[derive(Debug)]
pub struct FPSCounterUIController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _uiUpdateTimeInterval: f32,
    pub _currentFPSText: *mut crate::TMPro::TextMeshProUGUI,
    pub _lowestFPSText: *mut crate::TMPro::TextMeshProUGUI,
    pub _highestFPSText: *mut crate::TMPro::TextMeshProUGUI,
    pub _droppedFramesText: *mut crate::TMPro::TextMeshProUGUI,
    pub _gameScenesManager: *mut GameScenesManager,
    pub _fpsCounter: *mut FPSCounter,
    pub _timeToUpdateUI: f32,
}
#[cfg(feature = "FPSCounterUIController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for FPSCounterUIController => ""."FPSCounterUIController"
);
#[cfg(feature = "FPSCounterUIController")]
impl std::ops::Deref for FPSCounterUIController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FPSCounterUIController")]
impl std::ops::DerefMut for FPSCounterUIController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FPSCounterUIController")]
impl FPSCounterUIController {
    #[cfg(feature = "FPSCounterUIController+_Start_d__9")]
    pub type _Start_d__9 = crate::GlobalNamespace::FPSCounterUIController__Start_d__9;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "FPSCounterUIController")]
impl quest_hook::libil2cpp::ObjectType for FPSCounterUIController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
