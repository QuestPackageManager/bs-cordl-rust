#[cfg(feature = "ModifiersSelectionView")]
#[repr(C)]
#[derive(Debug)]
pub struct ModifiersSelectionView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _modifierInfoList: *mut GameplayModifierInfoListItemsList,
    pub _noModifiersText: *mut crate::TMPro::TextMeshProUGUI,
    pub _gameplayModifiersModel: *mut GameplayModifiersModelSO,
}
#[cfg(feature = "ModifiersSelectionView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ModifiersSelectionView => ""."ModifiersSelectionView"
);
#[cfg(feature = "ModifiersSelectionView")]
impl std::ops::Deref for ModifiersSelectionView {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ModifiersSelectionView")]
impl std::ops::DerefMut for ModifiersSelectionView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ModifiersSelectionView")]
impl ModifiersSelectionView {
    #[cfg(feature = "ModifiersSelectionView+__c__DisplayClass3_0")]
    pub type __c__DisplayClass3_0 = crate::GlobalNamespace::ModifiersSelectionView___c__DisplayClass3_0;
    pub fn SetGameplayModifiers(
        &mut self,
        gameplayModifiers: *mut GameplayModifiers,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGameplayModifiers", (gameplayModifiers))?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "ModifiersSelectionView")]
impl quest_hook::libil2cpp::ObjectType for ModifiersSelectionView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
