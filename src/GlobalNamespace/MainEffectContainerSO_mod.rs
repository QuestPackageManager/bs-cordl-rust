#[cfg(feature = "MainEffectContainerSO")]
#[repr(C)]
#[derive(Debug)]
pub struct MainEffectContainerSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _mainEffect: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MainEffectSO>,
    pub _postProcessEnabled: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BoolSO>,
}
#[cfg(feature = "MainEffectContainerSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MainEffectContainerSO => ""
    ."MainEffectContainerSO"
);
#[cfg(feature = "MainEffectContainerSO")]
impl std::ops::Deref for crate::GlobalNamespace::MainEffectContainerSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MainEffectContainerSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::MainEffectContainerSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MainEffectContainerSO")]
impl crate::GlobalNamespace::MainEffectContainerSO {
    pub fn Init(
        &mut self,
        mainEffect: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MainEffectSO>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (mainEffect))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
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
    pub fn get_mainEffect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MainEffectSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MainEffectSO,
        > = __cordl_object.invoke("get_mainEffect", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MainEffectContainerSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MainEffectContainerSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
