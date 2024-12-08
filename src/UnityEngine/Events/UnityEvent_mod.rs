#[cfg(feature = "UnityEngine+Events+UnityEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityEvent {
    __cordl_parent: crate::UnityEngine::Events::UnityEventBase,
    pub m_InvokeArray: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Object,
    >,
}
#[cfg(feature = "UnityEngine+Events+UnityEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Events::UnityEvent =>
    "UnityEngine.Events"."UnityEvent"
);
#[cfg(feature = "UnityEngine+Events+UnityEvent")]
impl std::ops::Deref for crate::UnityEngine::Events::UnityEvent {
    type Target = crate::UnityEngine::Events::UnityEventBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Events+UnityEvent")]
impl std::ops::DerefMut for crate::UnityEngine::Events::UnityEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Events+UnityEvent")]
impl crate::UnityEngine::Events::UnityEvent {
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn FindMethod_Impl(
        &mut self,
        name: *mut crate::System::String,
        targetObjType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::MethodInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::MethodInfo = __cordl_object
            .invoke("FindMethod_Impl", (name, targetObjType))?;
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
    pub fn AddListener(
        &mut self,
        call: *mut crate::UnityEngine::Events::UnityAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddListener", (call))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveListener(
        &mut self,
        call: *mut crate::UnityEngine::Events::UnityAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveListener", (call))?;
        Ok(__cordl_ret)
    }
    pub fn GetDelegate(
        &mut self,
        target: *mut crate::System::Object,
        theFunction: *mut crate::System::Reflection::MethodInfo,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::Events::BaseInvokableCall,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Events::BaseInvokableCall = __cordl_object
            .invoke("GetDelegate", (target, theFunction))?;
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
#[cfg(feature = "UnityEngine+Events+UnityEvent")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Events::UnityEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
