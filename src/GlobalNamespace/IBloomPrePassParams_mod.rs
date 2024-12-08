#[cfg(feature = "IBloomPrePassParams")]
#[repr(C)]
#[derive(Debug)]
pub struct IBloomPrePassParams {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IBloomPrePassParams")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for IBloomPrePassParams => ""."IBloomPrePassParams"
);
#[cfg(feature = "IBloomPrePassParams")]
impl std::ops::Deref for IBloomPrePassParams {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IBloomPrePassParams")]
impl std::ops::DerefMut for IBloomPrePassParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IBloomPrePassParams")]
impl IBloomPrePassParams {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
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
#[cfg(feature = "IBloomPrePassParams")]
impl quest_hook::libil2cpp::ObjectType for IBloomPrePassParams {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
