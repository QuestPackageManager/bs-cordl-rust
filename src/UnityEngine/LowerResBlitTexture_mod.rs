#[cfg(feature = "UnityEngine+LowerResBlitTexture")]
#[repr(C)]
#[derive(Debug)]
pub struct LowerResBlitTexture {
    __cordl_parent: crate::UnityEngine::Object,
}
#[cfg(feature = "UnityEngine+LowerResBlitTexture")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::LowerResBlitTexture =>
    "UnityEngine"."LowerResBlitTexture"
);
#[cfg(feature = "UnityEngine+LowerResBlitTexture")]
impl std::ops::Deref for crate::UnityEngine::LowerResBlitTexture {
    type Target = crate::UnityEngine::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+LowerResBlitTexture")]
impl std::ops::DerefMut for crate::UnityEngine::LowerResBlitTexture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+LowerResBlitTexture")]
impl crate::UnityEngine::LowerResBlitTexture {
    pub fn LowerResBlitTextureDontStripMe(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LowerResBlitTextureDontStripMe", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+LowerResBlitTexture")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::LowerResBlitTexture {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
