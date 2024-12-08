#[cfg(feature = "UnityEngine+U2D+SpriteAtlas")]
#[repr(C)]
#[derive(Debug)]
pub struct SpriteAtlas {
    __cordl_parent: crate::UnityEngine::Object,
}
#[cfg(feature = "UnityEngine+U2D+SpriteAtlas")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::U2D::SpriteAtlas =>
    "UnityEngine.U2D"."SpriteAtlas"
);
#[cfg(feature = "UnityEngine+U2D+SpriteAtlas")]
impl std::ops::Deref for crate::UnityEngine::U2D::SpriteAtlas {
    type Target = crate::UnityEngine::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+U2D+SpriteAtlas")]
impl std::ops::DerefMut for crate::UnityEngine::U2D::SpriteAtlas {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+U2D+SpriteAtlas")]
impl crate::UnityEngine::U2D::SpriteAtlas {
    pub fn CanBindTo(
        &mut self,
        sprite: *mut crate::UnityEngine::Sprite,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanBindTo", (sprite))?;
        Ok(__cordl_ret)
    }
    pub fn GetSprite(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Sprite> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Sprite = __cordl_object
            .invoke("GetSprite", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetSpritesScripting(
        &mut self,
        sprites: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetSpritesScripting", (sprites))?;
        Ok(__cordl_ret)
    }
    pub fn GetSpritesWithNameScripting(
        &mut self,
        sprites: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Sprite,
        >,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetSpritesWithNameScripting", (sprites, name))?;
        Ok(__cordl_ret)
    }
    pub fn GetSprites_Il2CppArray0(
        &mut self,
        sprites: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetSprites", (sprites))?;
        Ok(__cordl_ret)
    }
    pub fn GetSprites_String1(
        &mut self,
        sprites: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Sprite,
        >,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetSprites", (sprites, name))?;
        Ok(__cordl_ret)
    }
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
    pub fn get_isVariant(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isVariant", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_spriteCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_spriteCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_tag(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_tag", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+U2D+SpriteAtlas")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::U2D::SpriteAtlas {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}