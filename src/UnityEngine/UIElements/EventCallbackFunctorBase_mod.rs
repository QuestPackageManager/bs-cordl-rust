#[cfg(feature = "UnityEngine+UIElements+EventCallbackFunctorBase")]
#[repr(C)]
#[derive(Debug)]
pub struct EventCallbackFunctorBase {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _phase_k__BackingField: crate::UnityEngine::UIElements::CallbackPhase,
    pub _invokePolicy_k__BackingField: crate::UnityEngine::UIElements::InvokePolicy,
}
#[cfg(feature = "UnityEngine+UIElements+EventCallbackFunctorBase")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::EventCallbackFunctorBase {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "EventCallbackFunctorBase";
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
#[cfg(feature = "UnityEngine+UIElements+EventCallbackFunctorBase")]
impl std::ops::Deref for crate::UnityEngine::UIElements::EventCallbackFunctorBase {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventCallbackFunctorBase")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::EventCallbackFunctorBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventCallbackFunctorBase")]
impl crate::UnityEngine::UIElements::EventCallbackFunctorBase {
    pub fn Invoke(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
        propagationPhase: crate::UnityEngine::UIElements::PropagationPhase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (evt, propagationPhase))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEquivalentTo(
        &mut self,
        eventTypeId: i64,
        callback: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
        phase: crate::UnityEngine::UIElements::CallbackPhase,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsEquivalentTo", (eventTypeId, callback, phase))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        phase: crate::UnityEngine::UIElements::CallbackPhase,
        invokePolicy: crate::UnityEngine::UIElements::InvokePolicy,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (phase, invokePolicy))?;
        Ok(__cordl_object.into())
    }
    pub fn PhaseMatches(
        &mut self,
        propagationPhase: crate::UnityEngine::UIElements::PropagationPhase,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("PhaseMatches", (propagationPhase))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        phase: crate::UnityEngine::UIElements::CallbackPhase,
        invokePolicy: crate::UnityEngine::UIElements::InvokePolicy,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (phase, invokePolicy))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_invokePolicy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::InvokePolicy> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::InvokePolicy = __cordl_object
            .invoke("get_invokePolicy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_phase(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::CallbackPhase> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::CallbackPhase = __cordl_object
            .invoke("get_phase", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventCallbackFunctorBase")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::EventCallbackFunctorBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
