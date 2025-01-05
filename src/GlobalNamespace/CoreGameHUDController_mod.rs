#[cfg(feature = "CoreGameHUDController")]
#[repr(C)]
#[derive(Debug)]
pub struct CoreGameHUDController {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    pub _songProgressPanelGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _relativeScoreGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _immediateRankGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _energyPanelGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _canvasGroup: quest_hook::libil2cpp::Gc<crate::UnityEngine::CanvasGroup>,
}
#[cfg(feature = "CoreGameHUDController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CoreGameHUDController => ""
    ."CoreGameHUDController"
);
#[cfg(feature = "CoreGameHUDController")]
impl std::ops::Deref for crate::GlobalNamespace::CoreGameHUDController {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CoreGameHUDController")]
impl std::ops::DerefMut for crate::GlobalNamespace::CoreGameHUDController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CoreGameHUDController")]
impl crate::GlobalNamespace::CoreGameHUDController {
    #[cfg(feature = "CoreGameHUDController+InitData")]
    pub type InitData = crate::GlobalNamespace::CoreGameHUDController_InitData;
    pub fn Initialize(
        &mut self,
        initData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::CoreGameHUDController_InitData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", (initData))?;
        Ok(__cordl_ret.into())
    }
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_energyPanelGo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke("get_energyPanelGo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_immediateRankGo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke("get_immediateRankGo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_relativeScoreGo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke("get_relativeScoreGo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_songProgressPanelGO(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke("get_songProgressPanelGO", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "CoreGameHUDController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::CoreGameHUDController {
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
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
    pub fn New(
        hide: bool,
        showEnergyPanel: bool,
        advancedHUD: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (hide, showEnergyPanel, advancedHUD))?;
        Ok(__cordl_object.into())
    }
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
        Ok(__cordl_ret.into())
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
