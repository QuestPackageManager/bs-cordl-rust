#[cfg(feature = "BloomPrePassEffectSO")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassEffectSO {
    __cordl_parent: TextureEffectSO,
    pub _textureWidth: i32,
    pub _textureHeight: i32,
    pub _fov: crate::UnityEngine::Vector2,
    pub _linesWidth: f32,
}
#[cfg(feature = "BloomPrePassEffectSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BloomPrePassEffectSO => ""."BloomPrePassEffectSO"
);
#[cfg(feature = "BloomPrePassEffectSO")]
impl std::ops::Deref for BloomPrePassEffectSO {
    type Target = TextureEffectSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassEffectSO")]
impl std::ops::DerefMut for BloomPrePassEffectSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassEffectSO")]
impl BloomPrePassEffectSO {
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
    pub fn get_fov(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_fov", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_linesWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_linesWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_textureEffect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut TextureEffectSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut TextureEffectSO = __cordl_object
            .invoke("get_textureEffect", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_textureHeight(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_textureHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_textureWidth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_textureWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_toneMapping(&mut self) -> quest_hook::libil2cpp::Result<ToneMapping> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: ToneMapping = __cordl_object.invoke("get_toneMapping", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BloomPrePassEffectSO")]
impl quest_hook::libil2cpp::ObjectType for BloomPrePassEffectSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
