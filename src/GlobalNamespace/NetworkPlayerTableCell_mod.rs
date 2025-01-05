#[cfg(feature = "NetworkPlayerTableCell")]
#[repr(C)]
#[derive(Debug)]
pub struct NetworkPlayerTableCell {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::HMUI::TableCell>,
    pub _playerNameText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _separator: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _privateIcon: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
    pub _spectateIcon: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
    pub _partyLeaderIcon: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
    pub _bgImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
    pub _highlightImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
    pub _textColorNormal: crate::UnityEngine::Color,
    pub _textColorMe: crate::UnityEngine::Color,
    pub _textColorSelected: crate::UnityEngine::Color,
    pub _isMe: bool,
}
#[cfg(feature = "NetworkPlayerTableCell")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NetworkPlayerTableCell => ""
    ."NetworkPlayerTableCell"
);
#[cfg(feature = "NetworkPlayerTableCell")]
impl std::ops::Deref for crate::GlobalNamespace::NetworkPlayerTableCell {
    type Target = quest_hook::libil2cpp::Gc<crate::HMUI::TableCell>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NetworkPlayerTableCell")]
impl std::ops::DerefMut for crate::GlobalNamespace::NetworkPlayerTableCell {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NetworkPlayerTableCell")]
impl crate::GlobalNamespace::NetworkPlayerTableCell {
    pub fn HighlightDidChange(
        &mut self,
        transitionType: crate::HMUI::SelectableCell_TransitionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HighlightDidChange", (transitionType))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RefreshVisuals(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshVisuals", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SelectionDidChange(
        &mut self,
        transitionType: crate::HMUI::SelectableCell_TransitionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectionDidChange", (transitionType))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetData(
        &mut self,
        userName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isOpenParty: bool,
        wantsToPlayNextLevel: bool,
        isMyPartyOwner: bool,
        isMe: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetData",
                (userName, isOpenParty, wantsToPlayNextLevel, isMyPartyOwner, isMe),
            )?;
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
    pub fn get_activeColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_activeColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_showSeparator(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_showSeparator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_showSeparator(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_showSeparator", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NetworkPlayerTableCell")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NetworkPlayerTableCell {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
