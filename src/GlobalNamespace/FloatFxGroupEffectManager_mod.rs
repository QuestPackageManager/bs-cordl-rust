#[cfg(feature = "FloatFxGroupEffectManager")]
#[repr(C)]
#[derive(Debug)]
pub struct FloatFxGroupEffectManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _floatFxGroups: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::FloatFxGroup,
    >,
    pub _container: *mut crate::Zenject::DiContainer,
    pub _floatFxGroupEffects: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::FloatFxGroupEffect,
    >,
}
#[cfg(feature = "FloatFxGroupEffectManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FloatFxGroupEffectManager => ""
    ."FloatFxGroupEffectManager"
);
#[cfg(feature = "FloatFxGroupEffectManager")]
impl std::ops::Deref for crate::GlobalNamespace::FloatFxGroupEffectManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FloatFxGroupEffectManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::FloatFxGroupEffectManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FloatFxGroupEffectManager")]
impl crate::GlobalNamespace::FloatFxGroupEffectManager {
    #[cfg(feature = "FloatFxGroupEffectManager+__c")]
    pub type __c = crate::GlobalNamespace::FloatFxGroupEffectManager___c;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
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
}
#[cfg(feature = "FloatFxGroupEffectManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FloatFxGroupEffectManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
