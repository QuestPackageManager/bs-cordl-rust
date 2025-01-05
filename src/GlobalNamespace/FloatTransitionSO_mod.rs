#[cfg(feature = "FloatTransitionSO")]
#[repr(C)]
#[derive(Debug)]
pub struct FloatTransitionSO {
    __cordl_parent: crate::GlobalNamespace::BaseTransitionSO,
    pub _normalState: f32,
    pub _highlightedState: f32,
    pub _pressedState: f32,
    pub _disabledState: f32,
    pub _selectedState: f32,
    pub _selectedAndHighlightedState: f32,
}
#[cfg(feature = "FloatTransitionSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FloatTransitionSO => ""
    ."FloatTransitionSO"
);
#[cfg(feature = "FloatTransitionSO")]
impl std::ops::Deref for crate::GlobalNamespace::FloatTransitionSO {
    type Target = crate::GlobalNamespace::BaseTransitionSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FloatTransitionSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::FloatTransitionSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FloatTransitionSO")]
impl crate::GlobalNamespace::FloatTransitionSO {
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
    pub fn get_disabledState(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_disabledState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_highlightedState(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_highlightedState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_normalState(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_normalState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pressedState(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_pressedState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedAndHighlightedState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_selectedAndHighlightedState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedState(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_selectedState", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "FloatTransitionSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::FloatTransitionSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
