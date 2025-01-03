#[cfg(feature = "UnityEngine+U2D+SpriteAtlasManager")]
#[repr(C)]
#[derive(Debug)]
pub struct SpriteAtlasManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+U2D+SpriteAtlasManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::U2D::SpriteAtlasManager =>
    "UnityEngine.U2D"."SpriteAtlasManager"
);
#[cfg(feature = "UnityEngine+U2D+SpriteAtlasManager")]
impl std::ops::Deref for crate::UnityEngine::U2D::SpriteAtlasManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+U2D+SpriteAtlasManager")]
impl std::ops::DerefMut for crate::UnityEngine::U2D::SpriteAtlasManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+U2D+SpriteAtlasManager")]
impl crate::UnityEngine::U2D::SpriteAtlasManager {
    pub fn PostRegisteredAtlas(
        spriteAtlas: quest_hook::libil2cpp::Gc<crate::UnityEngine::U2D::SpriteAtlas>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PostRegisteredAtlas", (spriteAtlas))?;
        Ok(__cordl_ret.into())
    }
    pub fn Register(
        spriteAtlas: quest_hook::libil2cpp::Gc<crate::UnityEngine::U2D::SpriteAtlas>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Register", (spriteAtlas))?;
        Ok(__cordl_ret.into())
    }
    pub fn RequestAtlas(
        tag: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RequestAtlas", (tag))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_atlasRegistered(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::UnityEngine::U2D::SpriteAtlas>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_atlasRegistered", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_atlasRegistered(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::UnityEngine::U2D::SpriteAtlas>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_atlasRegistered", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+U2D+SpriteAtlasManager")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::U2D::SpriteAtlasManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
