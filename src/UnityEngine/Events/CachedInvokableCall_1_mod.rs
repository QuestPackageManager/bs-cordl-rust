#[cfg(feature = "UnityEngine+Events+CachedInvokableCall_1")]
#[repr(C)]
#[derive(Debug)]
pub struct CachedInvokableCall_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::UnityEngine::Events::InvokableCall_1<T>,
    pub m_Arg1: T,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+Events+CachedInvokableCall_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Events::CachedInvokableCall_1 < T >
    => "UnityEngine.Events"."CachedInvokableCall`1" < T >
);
#[cfg(feature = "UnityEngine+Events+CachedInvokableCall_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::Events::CachedInvokableCall_1<T> {
    type Target = crate::UnityEngine::Events::InvokableCall_1<T>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Events+CachedInvokableCall_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::Events::CachedInvokableCall_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Events+CachedInvokableCall_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::UnityEngine::Events::CachedInvokableCall_1<T> {
    pub fn Invoke_Il2CppArray0(
        &mut self,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (args))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke_T1(
        &mut self,
        arg0: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (arg0))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        target: *mut crate::UnityEngine::Object,
        theFunction: *mut crate::System::Reflection::MethodInfo,
        argument: T,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (target, theFunction, argument))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        target: *mut crate::UnityEngine::Object,
        theFunction: *mut crate::System::Reflection::MethodInfo,
        argument: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (target, theFunction, argument))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Events+CachedInvokableCall_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Events::CachedInvokableCall_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
