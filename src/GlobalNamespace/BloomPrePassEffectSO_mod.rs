#[cfg(feature = "BloomPrePassEffectSO")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassEffectSO {
    __cordl_parent: crate::GlobalNamespace::TextureEffectSO,
    pub _textureWidth: i32,
    pub _textureHeight: i32,
    pub _fov: crate::UnityEngine::Vector2,
    pub _linesWidth: f32,
}
#[cfg(feature = "BloomPrePassEffectSO")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BloomPrePassEffectSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BloomPrePassEffectSO";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "BloomPrePassEffectSO")]
impl std::ops::Deref for crate::GlobalNamespace::BloomPrePassEffectSO {
    type Target = crate::GlobalNamespace::TextureEffectSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassEffectSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::BloomPrePassEffectSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassEffectSO")]
impl crate::GlobalNamespace::BloomPrePassEffectSO {
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
    pub fn get_fov(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_fov", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_linesWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_linesWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_textureEffect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::TextureEffectSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::TextureEffectSO,
        > = __cordl_object.invoke("get_textureEffect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_textureHeight(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_textureHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_textureWidth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_textureWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_toneMapping(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::ToneMapping> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::ToneMapping = __cordl_object
            .invoke("get_toneMapping", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BloomPrePassEffectSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BloomPrePassEffectSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BloomPrePassEffectSO")]
impl AsRef<crate::GlobalNamespace::IBloomPrePassParams>
for crate::GlobalNamespace::BloomPrePassEffectSO {
    fn as_ref(&self) -> &crate::GlobalNamespace::IBloomPrePassParams {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BloomPrePassEffectSO")]
impl AsMut<crate::GlobalNamespace::IBloomPrePassParams>
for crate::GlobalNamespace::BloomPrePassEffectSO {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IBloomPrePassParams {
        unsafe { std::mem::transmute(self) }
    }
}
