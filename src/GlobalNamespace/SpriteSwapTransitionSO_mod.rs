#[cfg(feature = "SpriteSwapTransitionSO")]
#[repr(C)]
#[derive(Debug)]
pub struct SpriteSwapTransitionSO {
    __cordl_parent: crate::GlobalNamespace::BaseTransitionSO,
    pub _normalSprite: *mut crate::UnityEngine::Sprite,
    pub _highlightedSprite: *mut crate::UnityEngine::Sprite,
    pub _pressedSprite: *mut crate::UnityEngine::Sprite,
    pub _disabledSprite: *mut crate::UnityEngine::Sprite,
    pub _selectedSprite: *mut crate::UnityEngine::Sprite,
    pub _selectedAndHighlightedSprite: *mut crate::UnityEngine::Sprite,
}
#[cfg(feature = "SpriteSwapTransitionSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SpriteSwapTransitionSO => ""
    ."SpriteSwapTransitionSO"
);
#[cfg(feature = "SpriteSwapTransitionSO")]
impl std::ops::Deref for crate::GlobalNamespace::SpriteSwapTransitionSO {
    type Target = crate::GlobalNamespace::BaseTransitionSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SpriteSwapTransitionSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::SpriteSwapTransitionSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SpriteSwapTransitionSO")]
impl crate::GlobalNamespace::SpriteSwapTransitionSO {
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
    pub fn get_disabledSprite(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Sprite> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Sprite = __cordl_object
            .invoke("get_disabledSprite", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_highlightedSprite(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Sprite> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Sprite = __cordl_object
            .invoke("get_highlightedSprite", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_normalSprite(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Sprite> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Sprite = __cordl_object
            .invoke("get_normalSprite", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_pressedSprite(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Sprite> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Sprite = __cordl_object
            .invoke("get_pressedSprite", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selectedAndHighlightedSprite(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Sprite> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Sprite = __cordl_object
            .invoke("get_selectedAndHighlightedSprite", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selectedSprite(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Sprite> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Sprite = __cordl_object
            .invoke("get_selectedSprite", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SpriteSwapTransitionSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SpriteSwapTransitionSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
