#[cfg(feature = "UnityEngine+Events+InvokableCall")]
#[repr(C)]
#[derive(Debug)]
pub struct InvokableCall {
    __cordl_parent: crate::UnityEngine::Events::BaseInvokableCall,
    pub Delegate: *mut crate::UnityEngine::Events::UnityAction,
}
#[cfg(feature = "UnityEngine+Events+InvokableCall")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Events::InvokableCall =>
    "UnityEngine.Events"."InvokableCall"
);
#[cfg(feature = "UnityEngine+Events+InvokableCall")]
impl std::ops::Deref for crate::UnityEngine::Events::InvokableCall {
    type Target = crate::UnityEngine::Events::BaseInvokableCall;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Events+InvokableCall")]
impl std::ops::DerefMut for crate::UnityEngine::Events::InvokableCall {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Events+InvokableCall")]
impl crate::UnityEngine::Events::InvokableCall {
    pub fn Find(
        &mut self,
        targetObj: *mut quest_hook::libil2cpp::Il2CppObject,
        method: *mut crate::System::Reflection::MethodInfo,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Find", (targetObj, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn Invoke_Il2CppArray0(
        &mut self,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (args))?;
        Ok(__cordl_ret)
    }
    pub fn New_Il2CppObject_MethodInfo0(
        target: *mut quest_hook::libil2cpp::Il2CppObject,
        theFunction: *mut crate::System::Reflection::MethodInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (target, theFunction))?;
        Ok(__cordl_object)
    }
    pub fn New_UnityAction1(
        action: *mut crate::UnityEngine::Events::UnityAction,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (action))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_Il2CppObject_MethodInfo0(
        &mut self,
        target: *mut quest_hook::libil2cpp::Il2CppObject,
        theFunction: *mut crate::System::Reflection::MethodInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (target, theFunction))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_UnityAction1(
        &mut self,
        action: *mut crate::UnityEngine::Events::UnityAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (action))?;
        Ok(__cordl_ret)
    }
    pub fn add_Delegate(
        &mut self,
        value: *mut crate::UnityEngine::Events::UnityAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_Delegate", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_Delegate(
        &mut self,
        value: *mut crate::UnityEngine::Events::UnityAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_Delegate", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Events+InvokableCall")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Events::InvokableCall {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
