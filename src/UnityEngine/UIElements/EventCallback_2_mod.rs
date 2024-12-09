#[cfg(feature = "UnityEngine+UIElements+EventCallback_2")]
#[repr(C)]
#[derive(Debug)]
pub struct EventCallback_2<
    TEventType: quest_hook::libil2cpp::Type,
    TCallbackArgs: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::MulticastDelegate,
    __cordl_phantom_TEventType: std::marker::PhantomData<TEventType>,
    __cordl_phantom_TCallbackArgs: std::marker::PhantomData<TCallbackArgs>,
}
#[cfg(feature = "UnityEngine+UIElements+EventCallback_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::EventCallback_2 <
    TEventType, TCallbackArgs > => "UnityEngine.UIElements"."EventCallback`2" <
    TEventType, TCallbackArgs >
);
#[cfg(feature = "UnityEngine+UIElements+EventCallback_2")]
impl<
    TEventType: quest_hook::libil2cpp::Type,
    TCallbackArgs: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::UnityEngine::UIElements::EventCallback_2<TEventType, TCallbackArgs> {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventCallback_2")]
impl<
    TEventType: quest_hook::libil2cpp::Type,
    TCallbackArgs: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::UnityEngine::UIElements::EventCallback_2<TEventType, TCallbackArgs> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventCallback_2")]
impl<
    TEventType: quest_hook::libil2cpp::Type,
    TCallbackArgs: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::EventCallback_2<TEventType, TCallbackArgs> {
    pub fn Invoke(
        &mut self,
        evt: TEventType,
        userArgs: TCallbackArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TEventType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TCallbackArgs: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (evt, userArgs))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        TEventType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TCallbackArgs: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TEventType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TCallbackArgs: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventCallback_2")]
impl<
    TEventType: quest_hook::libil2cpp::Type,
    TCallbackArgs: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::EventCallback_2<TEventType, TCallbackArgs> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
