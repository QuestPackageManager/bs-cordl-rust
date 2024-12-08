#[cfg(feature = "TriggerFloatFxGroupEffectManager")]
#[repr(C)]
#[derive(Debug)]
pub struct TriggerFloatFxGroupEffectManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _floatFxGroups: *mut quest_hook::libil2cpp::Il2CppArray<*mut FloatFxGroup>,
    pub _container: *mut crate::Zenject::DiContainer,
    pub _floatFxGroupEffects: *mut crate::System::Collections::Generic::List_1<
        *mut TriggerFloatFxGroupEffect,
    >,
}
#[cfg(feature = "TriggerFloatFxGroupEffectManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for TriggerFloatFxGroupEffectManager => ""
    ."TriggerFloatFxGroupEffectManager"
);
#[cfg(feature = "TriggerFloatFxGroupEffectManager")]
impl std::ops::Deref for TriggerFloatFxGroupEffectManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TriggerFloatFxGroupEffectManager")]
impl std::ops::DerefMut for TriggerFloatFxGroupEffectManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TriggerFloatFxGroupEffectManager")]
impl TriggerFloatFxGroupEffectManager {
    #[cfg(feature = "TriggerFloatFxGroupEffectManager+__c")]
    pub type __c = crate::GlobalNamespace::TriggerFloatFxGroupEffectManager___c;
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
#[cfg(feature = "TriggerFloatFxGroupEffectManager")]
impl quest_hook::libil2cpp::ObjectType for TriggerFloatFxGroupEffectManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
