#[cfg(feature = "UnityEngine+Events+PersistentCall")]
#[repr(C)]
#[derive(Debug)]
pub struct PersistentCall {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Target: *mut crate::UnityEngine::Object,
    pub m_TargetAssemblyTypeName: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_MethodName: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_Mode: crate::UnityEngine::Events::PersistentListenerMode,
    pub m_Arguments: *mut crate::UnityEngine::Events::ArgumentCache,
    pub m_CallState: crate::UnityEngine::Events::UnityEventCallState,
}
#[cfg(feature = "UnityEngine+Events+PersistentCall")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Events::PersistentCall =>
    "UnityEngine.Events"."PersistentCall"
);
#[cfg(feature = "UnityEngine+Events+PersistentCall")]
impl std::ops::Deref for crate::UnityEngine::Events::PersistentCall {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Events+PersistentCall")]
impl std::ops::DerefMut for crate::UnityEngine::Events::PersistentCall {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Events+PersistentCall")]
impl crate::UnityEngine::Events::PersistentCall {
    pub fn GetRuntimeCall(
        &mut self,
        theEvent: *mut crate::UnityEngine::Events::UnityEventBase,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::Events::BaseInvokableCall,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Events::BaseInvokableCall = __cordl_object
            .invoke("GetRuntimeCall", (theEvent))?;
        Ok(__cordl_ret)
    }
    pub fn IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsValid", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnAfterDeserialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnAfterDeserialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnBeforeSerialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnBeforeSerialize", ())?;
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
    pub fn get_arguments(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Events::ArgumentCache> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Events::ArgumentCache = __cordl_object
            .invoke("get_arguments", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_methodName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_methodName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_mode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Events::PersistentListenerMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Events::PersistentListenerMode = __cordl_object
            .invoke("get_mode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_target(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Object = __cordl_object
            .invoke("get_target", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_targetAssemblyTypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_targetAssemblyTypeName", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Events+PersistentCall")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Events::PersistentCall {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
