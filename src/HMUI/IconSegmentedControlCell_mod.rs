#[cfg(feature = "HMUI+IconSegmentedControlCell")]
#[repr(C)]
#[derive(Debug)]
pub struct IconSegmentedControlCell {
    __cordl_parent: crate::HMUI::SegmentedControlCell,
    pub _icon: *mut crate::UnityEngine::UI::Image,
    pub _hoverHint: *mut crate::HMUI::HoverHint,
    pub _backgroundGameObject: *mut crate::UnityEngine::GameObject,
}
#[cfg(feature = "HMUI+IconSegmentedControlCell")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::IconSegmentedControlCell => "HMUI"
    ."IconSegmentedControlCell"
);
#[cfg(feature = "HMUI+IconSegmentedControlCell")]
impl std::ops::Deref for crate::HMUI::IconSegmentedControlCell {
    type Target = crate::HMUI::SegmentedControlCell;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+IconSegmentedControlCell")]
impl std::ops::DerefMut for crate::HMUI::IconSegmentedControlCell {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+IconSegmentedControlCell")]
impl crate::HMUI::IconSegmentedControlCell {
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
    pub fn get_sprite(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Sprite> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Sprite = __cordl_object
            .invoke("get_sprite", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_hideBackgroundImage(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hideBackgroundImage", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_hintText(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hintText", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_iconSize(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_iconSize", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_sprite(
        &mut self,
        value: *mut crate::UnityEngine::Sprite,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sprite", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HMUI+IconSegmentedControlCell")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::IconSegmentedControlCell {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
