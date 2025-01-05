#[cfg(feature = "UnityEngine+Sprites+DataUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct DataUtility {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+Sprites+DataUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Sprites::DataUtility =>
    "UnityEngine.Sprites"."DataUtility"
);
#[cfg(feature = "UnityEngine+Sprites+DataUtility")]
impl std::ops::Deref for crate::UnityEngine::Sprites::DataUtility {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Sprites+DataUtility")]
impl std::ops::DerefMut for crate::UnityEngine::Sprites::DataUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Sprites+DataUtility")]
impl crate::UnityEngine::Sprites::DataUtility {
    pub fn GetInnerUV(
        sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_ret: crate::UnityEngine::Vector4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInnerUV", (sprite))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMinSize(
        sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMinSize", (sprite))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOuterUV(
        sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_ret: crate::UnityEngine::Vector4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOuterUV", (sprite))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPadding(
        sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_ret: crate::UnityEngine::Vector4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPadding", (sprite))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Sprites+DataUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Sprites::DataUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
