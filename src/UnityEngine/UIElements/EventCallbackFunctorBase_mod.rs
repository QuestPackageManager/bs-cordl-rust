#[cfg(feature = "cordl_class_UnityEngine+UIElements+EventCallbackFunctorBase")]
#[repr(C)]
#[derive(Debug)]
pub struct EventCallbackFunctorBase {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _phase_k__BackingField: crate::UnityEngine::UIElements::CallbackPhase,
    pub _invokePolicy_k__BackingField: crate::UnityEngine::UIElements::InvokePolicy,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+EventCallbackFunctorBase")]
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
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventCallbackFunctorBase")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::EventCallbackFunctorBase {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::EventBase,
                            >,
                            crate::UnityEngine::UIElements::PropagationPhase,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Invoke",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (evt, propagationPhase))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsEquivalentTo(
        &mut self,
        eventTypeId: i64,
        callback: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
        phase: crate::UnityEngine::UIElements::CallbackPhase,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i64,
                            quest_hook::libil2cpp::Gc<crate::System::Delegate>,
                            crate::UnityEngine::UIElements::CallbackPhase,
                        ),
                        bool,
                        3usize,
                    >("IsEquivalentTo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsEquivalentTo", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (eventTypeId, callback, phase))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::PropagationPhase),
                        bool,
                        1usize,
                    >("PhaseMatches")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PhaseMatches", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (propagationPhase))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        phase: crate::UnityEngine::UIElements::CallbackPhase,
        invokePolicy: crate::UnityEngine::UIElements::InvokePolicy,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::UIElements::CallbackPhase,
                            crate::UnityEngine::UIElements::InvokePolicy,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (phase, invokePolicy))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_invokePolicy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::InvokePolicy> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::UIElements::InvokePolicy,
                        0usize,
                    >("get_invokePolicy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_invokePolicy", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::InvokePolicy = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_phase(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::CallbackPhase> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::UIElements::CallbackPhase,
                        0usize,
                    >("get_phase")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_phase", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::CallbackPhase = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+EventCallbackFunctorBase")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::EventCallbackFunctorBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
