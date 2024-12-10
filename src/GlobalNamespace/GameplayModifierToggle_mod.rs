#[cfg(feature = "GameplayModifierToggle")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayModifierToggle {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _gameplayModifier: *mut crate::GlobalNamespace::GameplayModifierParamsSO,
    pub _nameText: *mut crate::TMPro::TextMeshProUGUI,
    pub _multiplierText: *mut crate::TMPro::TextMeshProUGUI,
    pub _hoverTextSetter: *mut crate::HMUI::HoverTextSetter,
    pub _icon: *mut crate::UnityEngine::UI::Image,
    pub _toggle: *mut crate::UnityEngine::UI::Toggle,
    pub _positiveColor: crate::UnityEngine::Color,
}
#[cfg(feature = "GameplayModifierToggle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameplayModifierToggle => ""
    ."GameplayModifierToggle"
);
#[cfg(feature = "GameplayModifierToggle")]
impl std::ops::Deref for crate::GlobalNamespace::GameplayModifierToggle {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifierToggle")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameplayModifierToggle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifierToggle")]
impl crate::GlobalNamespace::GameplayModifierToggle {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_gameplayModifier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifierParamsSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifierParamsSO,
        > = __cordl_object.invoke("get_gameplayModifier", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_toggle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Toggle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Toggle> = __cordl_object
            .invoke("get_toggle", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GameplayModifierToggle")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayModifierToggle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
