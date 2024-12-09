#[cfg(feature = "UnityEngine+Events+InvokableCall_1")]
#[repr(C)]
#[derive(Debug)]
pub struct InvokableCall_1<T1: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::UnityEngine::Events::BaseInvokableCall,
    pub Delegate: *mut crate::UnityEngine::Events::UnityAction_1<T1>,
    __cordl_phantom_T1: std::marker::PhantomData<T1>,
}
#[cfg(feature = "UnityEngine+Events+InvokableCall_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Events::InvokableCall_1 < T1 > =>
    "UnityEngine.Events"."InvokableCall`1" < T1 >
);
#[cfg(feature = "UnityEngine+Events+InvokableCall_1")]
impl<T1: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::Events::InvokableCall_1<T1> {
    type Target = crate::UnityEngine::Events::BaseInvokableCall;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Events+InvokableCall_1")]
impl<T1: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::Events::InvokableCall_1<T1> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Events+InvokableCall_1")]
impl<T1: quest_hook::libil2cpp::Type> crate::UnityEngine::Events::InvokableCall_1<T1> {
    pub fn Find(
        &mut self,
        targetObj: *mut crate::System::Object,
        method: *mut crate::System::Reflection::MethodInfo,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Find", (targetObj, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke_Il2CppArray0(
        &mut self,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (args))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke_T1_1(
        &mut self,
        args0: T1,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (args0))?;
        Ok(__cordl_ret)
    }
    pub fn New_Object_MethodInfo0(
        target: *mut crate::System::Object,
        theFunction: *mut crate::System::Reflection::MethodInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (target, theFunction))?;
        Ok(__cordl_object)
    }
    pub fn New_UnityAction_1_1(
        action: *mut crate::UnityEngine::Events::UnityAction_1<T1>,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (action))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_Object_MethodInfo0(
        &mut self,
        target: *mut crate::System::Object,
        theFunction: *mut crate::System::Reflection::MethodInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (target, theFunction))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_UnityAction_1_1(
        &mut self,
        action: *mut crate::UnityEngine::Events::UnityAction_1<T1>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (action))?;
        Ok(__cordl_ret)
    }
    pub fn add_Delegate(
        &mut self,
        value: *mut crate::UnityEngine::Events::UnityAction_1<T1>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_Delegate", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_Delegate(
        &mut self,
        value: *mut crate::UnityEngine::Events::UnityAction_1<T1>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_Delegate", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Events+InvokableCall_1")]
impl<T1: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Events::InvokableCall_1<T1> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
