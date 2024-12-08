#[cfg(feature = "BloomPrePassEffectContainerSO")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassEffectContainerSO {
    __cordl_parent: PersistentScriptableObject,
    pub _bloomPrePassEffect: *mut BloomPrePassEffectSO,
}
#[cfg(feature = "BloomPrePassEffectContainerSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BloomPrePassEffectContainerSO => ""
    ."BloomPrePassEffectContainerSO"
);
#[cfg(feature = "BloomPrePassEffectContainerSO")]
impl std::ops::Deref for BloomPrePassEffectContainerSO {
    type Target = PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassEffectContainerSO")]
impl std::ops::DerefMut for BloomPrePassEffectContainerSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassEffectContainerSO")]
impl BloomPrePassEffectContainerSO {
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
    pub fn Init(
        &mut self,
        bloomPrePassEffect: *mut BloomPrePassEffectSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (bloomPrePassEffect))?;
        Ok(__cordl_ret)
    }
    pub fn get_bloomPrePassEffect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut BloomPrePassEffectSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BloomPrePassEffectSO = __cordl_object
            .invoke("get_bloomPrePassEffect", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BloomPrePassEffectContainerSO")]
impl quest_hook::libil2cpp::ObjectType for BloomPrePassEffectContainerSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
