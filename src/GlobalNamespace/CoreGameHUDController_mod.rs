#[cfg(feature = "CoreGameHUDController")]
#[repr(C)]
#[derive(Debug)]
pub struct CoreGameHUDController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _songProgressPanelGO: *mut crate::UnityEngine::GameObject,
    pub _relativeScoreGO: *mut crate::UnityEngine::GameObject,
    pub _immediateRankGO: *mut crate::UnityEngine::GameObject,
    pub _energyPanelGO: *mut crate::UnityEngine::GameObject,
    pub _canvasGroup: *mut crate::UnityEngine::CanvasGroup,
}
#[cfg(feature = "CoreGameHUDController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for CoreGameHUDController => ""."CoreGameHUDController"
);
#[cfg(feature = "CoreGameHUDController")]
impl std::ops::Deref for CoreGameHUDController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CoreGameHUDController")]
impl std::ops::DerefMut for CoreGameHUDController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CoreGameHUDController")]
impl CoreGameHUDController {
    #[cfg(feature = "CoreGameHUDController+InitData")]
    pub type InitData = crate::GlobalNamespace::CoreGameHUDController_InitData;
    pub fn get_songProgressPanelGO(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("get_songProgressPanelGO", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_alpha(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_alpha", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
        initData: *mut crate::GlobalNamespace::CoreGameHUDController_InitData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", (initData))?;
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
    pub fn get_relativeScoreGo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("get_relativeScoreGo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_immediateRankGo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("get_immediateRankGo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_energyPanelGo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("get_energyPanelGo", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "CoreGameHUDController")]
impl quest_hook::libil2cpp::ObjectType for CoreGameHUDController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "CoreGameHUDController+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct CoreGameHUDController_InitData {
    __cordl_parent: crate::System::Object,
    pub hide: bool,
    pub showEnergyPanel: bool,
    pub advancedHUD: bool,
}
#[cfg(feature = "CoreGameHUDController+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CoreGameHUDController_InitData
    => ""."CoreGameHUDController/InitData"
);
#[cfg(feature = "CoreGameHUDController+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::CoreGameHUDController_InitData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CoreGameHUDController+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::CoreGameHUDController_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CoreGameHUDController+InitData")]
impl crate::GlobalNamespace::CoreGameHUDController_InitData {
    pub fn _ctor(
        &mut self,
        hide: bool,
        showEnergyPanel: bool,
        advancedHUD: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (hide, showEnergyPanel, advancedHUD))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        hide: bool,
        showEnergyPanel: bool,
        advancedHUD: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (hide, showEnergyPanel, advancedHUD))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "CoreGameHUDController+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::CoreGameHUDController_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
