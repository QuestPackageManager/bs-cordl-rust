#[cfg(feature = "GradientTransitionSO")]
#[repr(C)]
#[derive(Debug)]
pub struct GradientTransitionSO {
    __cordl_parent: crate::GlobalNamespace::BaseTransitionSO,
    pub _normalColor1: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub _normalColor2: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub _highlightColor1: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub _highlightColor2: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub _pressedColor1: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub _pressedColor2: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub _disabledColor1: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub _disabledColor2: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub _selectedColor1: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub _selectedColor2: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub _selectedAndHighlightedColor1: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ColorSO,
    >,
    pub _selectedAndHighlightedColor2: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ColorSO,
    >,
}
#[cfg(feature = "GradientTransitionSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GradientTransitionSO => ""
    ."GradientTransitionSO"
);
#[cfg(feature = "GradientTransitionSO")]
impl std::ops::Deref for crate::GlobalNamespace::GradientTransitionSO {
    type Target = crate::GlobalNamespace::BaseTransitionSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GradientTransitionSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::GradientTransitionSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GradientTransitionSO")]
impl crate::GlobalNamespace::GradientTransitionSO {
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
    pub fn get_disabledColor1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_disabledColor1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_disabledColor2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_disabledColor2", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_highlightColor1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_highlightColor1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_highlightColor2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_highlightColor2", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_normalColor1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_normalColor1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_normalColor2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_normalColor2", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pressedColor1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_pressedColor1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pressedColor2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_pressedColor2", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedAndHighlightedColor1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_selectedAndHighlightedColor1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedAndHighlightedColor2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_selectedAndHighlightedColor2", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedColor1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_selectedColor1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedColor2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_selectedColor2", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GradientTransitionSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::GradientTransitionSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
