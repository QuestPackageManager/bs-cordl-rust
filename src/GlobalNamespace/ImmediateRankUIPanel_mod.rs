#[cfg(feature = "ImmediateRankUIPanel")]
#[repr(C)]
#[derive(Debug)]
pub struct ImmediateRankUIPanel {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _rankText: *mut crate::TMPro::TextMeshProUGUI,
    pub _relativeScoreText: *mut crate::TMPro::TextMeshProUGUI,
    pub _relativeScoreAndImmediateRankCounter: *mut crate::GlobalNamespace::RelativeScoreAndImmediateRankCounter,
    pub _stringBuilder: *mut crate::System::Text::StringBuilder,
    pub _prevRelativeScore: f32,
    pub _prevImmediateRank: crate::GlobalNamespace::RankModel_Rank,
}
#[cfg(feature = "ImmediateRankUIPanel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ImmediateRankUIPanel => ""
    ."ImmediateRankUIPanel"
);
#[cfg(feature = "ImmediateRankUIPanel")]
impl std::ops::Deref for crate::GlobalNamespace::ImmediateRankUIPanel {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ImmediateRankUIPanel")]
impl std::ops::DerefMut for crate::GlobalNamespace::ImmediateRankUIPanel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ImmediateRankUIPanel")]
impl crate::GlobalNamespace::ImmediateRankUIPanel {
    pub fn HandleRelativeScoreAndImmediateRankCounterRelativeScoreOrImmediateRankDidChange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleRelativeScoreAndImmediateRankCounterRelativeScoreOrImmediateRankDidChange",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn RefreshUI(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshUI", ())?;
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
#[cfg(feature = "ImmediateRankUIPanel")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ImmediateRankUIPanel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
