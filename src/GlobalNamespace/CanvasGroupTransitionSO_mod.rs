#[cfg(feature = "CanvasGroupTransitionSO")]
#[repr(C)]
#[derive(Debug)]
pub struct CanvasGroupTransitionSO {
    __cordl_parent: crate::GlobalNamespace::BaseTransitionSO,
    pub _normalAlpha: f32,
    pub _highlightedAlpha: f32,
    pub _pressedAlpha: f32,
    pub _disabledAlpha: f32,
    pub _selectedAlpha: f32,
    pub _selectedAndHighlightedAlpha: f32,
}
#[cfg(feature = "CanvasGroupTransitionSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CanvasGroupTransitionSO => ""
    ."CanvasGroupTransitionSO"
);
#[cfg(feature = "CanvasGroupTransitionSO")]
impl std::ops::Deref for crate::GlobalNamespace::CanvasGroupTransitionSO {
    type Target = crate::GlobalNamespace::BaseTransitionSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CanvasGroupTransitionSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::CanvasGroupTransitionSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CanvasGroupTransitionSO")]
impl crate::GlobalNamespace::CanvasGroupTransitionSO {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn get_disabledAlpha(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_disabledAlpha", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_highlightedAlpha(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_highlightedAlpha", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_normalAlpha(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_normalAlpha", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_pressedAlpha(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_pressedAlpha", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selectedAlpha(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_selectedAlpha", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selectedAndHighlightedAlpha(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_selectedAndHighlightedAlpha", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "CanvasGroupTransitionSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::CanvasGroupTransitionSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
