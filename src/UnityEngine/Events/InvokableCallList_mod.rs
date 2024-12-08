#[cfg(feature = "UnityEngine+Events+InvokableCallList")]
#[repr(C)]
#[derive(Debug)]
pub struct InvokableCallList {
    __cordl_parent: crate::System::Object,
    pub m_PersistentCalls: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::Events::BaseInvokableCall,
    >,
    pub m_RuntimeCalls: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::Events::BaseInvokableCall,
    >,
    pub m_ExecutingCalls: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::Events::BaseInvokableCall,
    >,
    pub m_NeedsUpdate: bool,
}
#[cfg(feature = "UnityEngine+Events+InvokableCallList")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Events::InvokableCallList =>
    "UnityEngine.Events"."InvokableCallList"
);
#[cfg(feature = "UnityEngine+Events+InvokableCallList")]
impl std::ops::Deref for crate::UnityEngine::Events::InvokableCallList {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Events+InvokableCallList")]
impl std::ops::DerefMut for crate::UnityEngine::Events::InvokableCallList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Events+InvokableCallList")]
impl crate::UnityEngine::Events::InvokableCallList {
    pub fn AddListener(
        &mut self,
        call: *mut crate::UnityEngine::Events::BaseInvokableCall,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddListener", (call))?;
        Ok(__cordl_ret)
    }
    pub fn AddPersistentInvokableCall(
        &mut self,
        call: *mut crate::UnityEngine::Events::BaseInvokableCall,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddPersistentInvokableCall", (call))?;
        Ok(__cordl_ret)
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearPersistent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearPersistent", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn PrepareInvoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::Events::BaseInvokableCall,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::Events::BaseInvokableCall,
        > = __cordl_object.invoke("PrepareInvoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn RemoveListener(
        &mut self,
        targetObj: *mut crate::System::Object,
        method: *mut crate::System::Reflection::MethodInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveListener", (targetObj, method))?;
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
#[cfg(feature = "UnityEngine+Events+InvokableCallList")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Events::InvokableCallList {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
