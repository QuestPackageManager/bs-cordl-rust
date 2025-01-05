#[cfg(feature = "ColorTransitionSO")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorTransitionSO {
    __cordl_parent: crate::GlobalNamespace::BaseTransitionSO,
    pub _normalColor: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub _highlightedColor: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub _pressedColor: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub _disabledColor: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub _selectedColor: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub _selectedAndHighlightedColor: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ColorSO,
    >,
}
#[cfg(feature = "ColorTransitionSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ColorTransitionSO => ""
    ."ColorTransitionSO"
);
#[cfg(feature = "ColorTransitionSO")]
impl std::ops::Deref for crate::GlobalNamespace::ColorTransitionSO {
    type Target = crate::GlobalNamespace::BaseTransitionSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ColorTransitionSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::ColorTransitionSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ColorTransitionSO")]
impl crate::GlobalNamespace::ColorTransitionSO {
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
    pub fn get_disabledColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_disabledColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_highlightedColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_highlightedColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_normalColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_normalColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pressedColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_pressedColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedAndHighlightedColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_selectedAndHighlightedColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_selectedColor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ColorTransitionSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ColorTransitionSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
