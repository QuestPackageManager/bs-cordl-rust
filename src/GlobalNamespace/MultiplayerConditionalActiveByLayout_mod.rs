#[cfg(feature = "MultiplayerConditionalActiveByLayout+Condition")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplayerConditionalActiveByLayout_Condition {
    HideIf = 1i32,
    ShowIf = 0i32,
}
#[cfg(feature = "MultiplayerConditionalActiveByLayout+Condition")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerConditionalActiveByLayout_Condition => ""
    ."MultiplayerConditionalActiveByLayout/Condition"
);
#[cfg(feature = "MultiplayerConditionalActiveByLayout")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerConditionalActiveByLayout {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _condition: crate::GlobalNamespace::MultiplayerConditionalActiveByLayout_Condition,
    pub _layout: crate::GlobalNamespace::MultiplayerPlayerLayout,
    pub _layoutProvider: *mut crate::GlobalNamespace::MultiplayerLayoutProvider,
}
#[cfg(feature = "MultiplayerConditionalActiveByLayout")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerConditionalActiveByLayout => ""
    ."MultiplayerConditionalActiveByLayout"
);
#[cfg(feature = "MultiplayerConditionalActiveByLayout")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerConditionalActiveByLayout {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConditionalActiveByLayout")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerConditionalActiveByLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConditionalActiveByLayout")]
impl crate::GlobalNamespace::MultiplayerConditionalActiveByLayout {
    #[cfg(feature = "MultiplayerConditionalActiveByLayout+Condition")]
    pub type Condition = crate::GlobalNamespace::MultiplayerConditionalActiveByLayout_Condition;
    pub fn HandlePlayersLayoutWasCalculated(
        &mut self,
        layout: crate::GlobalNamespace::MultiplayerPlayerLayout,
        playersCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayersLayoutWasCalculated", (layout, playersCount))?;
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
#[cfg(feature = "MultiplayerConditionalActiveByLayout")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerConditionalActiveByLayout {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
