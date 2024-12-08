#[cfg(feature = "ScoreUIController+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct ScoreUIController_InitData {
    __cordl_parent: crate::System::Object,
    pub scoreDisplayType: crate::GlobalNamespace::ScoreUIController_ScoreDisplayType,
}
#[cfg(feature = "ScoreUIController+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ScoreUIController_InitData =>
    ""."ScoreUIController/InitData"
);
#[cfg(feature = "ScoreUIController+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::ScoreUIController_InitData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ScoreUIController+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::ScoreUIController_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ScoreUIController+InitData")]
impl crate::GlobalNamespace::ScoreUIController_InitData {
    pub fn New(
        scoreDisplayType: crate::GlobalNamespace::ScoreUIController_ScoreDisplayType,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (scoreDisplayType))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        scoreDisplayType: crate::GlobalNamespace::ScoreUIController_ScoreDisplayType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (scoreDisplayType))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "ScoreUIController+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ScoreUIController_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ScoreUIController+ScoreDisplayType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScoreUIController_ScoreDisplayType {
    ModifiedScore = 1i32,
    MultipliedScore = 0i32,
}
#[cfg(feature = "ScoreUIController+ScoreDisplayType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ScoreUIController_ScoreDisplayType => ""
    ."ScoreUIController/ScoreDisplayType"
);
#[cfg(feature = "ScoreUIController")]
#[repr(C)]
#[derive(Debug)]
pub struct ScoreUIController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _scoreText: *mut crate::TMPro::TextMeshProUGUI,
    pub _initData: *mut crate::GlobalNamespace::ScoreUIController_InitData,
    pub _scoreController: *mut IScoreController,
    pub _stringBuilder: *mut crate::System::Text::StringBuilder,
}
#[cfg(feature = "ScoreUIController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ScoreUIController => ""."ScoreUIController"
);
#[cfg(feature = "ScoreUIController")]
impl std::ops::Deref for ScoreUIController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ScoreUIController")]
impl std::ops::DerefMut for ScoreUIController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ScoreUIController")]
impl ScoreUIController {
    pub const kMaxNumberOfDigits: i32 = 9i32;
    #[cfg(feature = "ScoreUIController+InitData")]
    pub type InitData = crate::GlobalNamespace::ScoreUIController_InitData;
    #[cfg(feature = "ScoreUIController+ScoreDisplayType")]
    pub type ScoreDisplayType = crate::GlobalNamespace::ScoreUIController_ScoreDisplayType;
    pub fn HandleScoreDidChangeRealtime(
        &mut self,
        multipliedScore: i32,
        modifiedScore: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleScoreDidChangeRealtime", (multipliedScore, modifiedScore))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn RegisterForEvents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterForEvents", ())?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnregisterFromEvents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterFromEvents", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateScore(
        &mut self,
        multipliedScore: i32,
        modifiedScore: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateScore", (multipliedScore, modifiedScore))?;
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
#[cfg(feature = "ScoreUIController")]
impl quest_hook::libil2cpp::ObjectType for ScoreUIController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
