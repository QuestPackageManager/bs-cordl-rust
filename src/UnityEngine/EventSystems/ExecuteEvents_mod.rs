#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents")]
#[repr(C)]
#[derive(Debug)]
pub struct ExecuteEvents {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::EventSystems::ExecuteEvents =>
    "UnityEngine.EventSystems"."ExecuteEvents"
);
#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents")]
impl std::ops::Deref for crate::UnityEngine::EventSystems::ExecuteEvents {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents")]
impl std::ops::DerefMut for crate::UnityEngine::EventSystems::ExecuteEvents {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents")]
impl crate::UnityEngine::EventSystems::ExecuteEvents {
    #[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents+EventFunction_1")]
    pub type EventFunction_1<T1: quest_hook::libil2cpp::Type> = crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
        T1,
    >;
}
#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::ExecuteEvents {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents+EventFunction_1")]
#[repr(C)]
#[derive(Debug)]
pub struct ExecuteEvents_EventFunction_1<T1: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::MulticastDelegate,
    __cordl_phantom_T1: std::marker::PhantomData<T1>,
}
#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents+EventFunction_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1 < T1 > =>
    "UnityEngine.EventSystems"."ExecuteEvents/EventFunction`1" < T1 >
);
#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents+EventFunction_1")]
impl<T1: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<T1> {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents+EventFunction_1")]
impl<T1: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<T1> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents+EventFunction_1")]
impl<
    T1: quest_hook::libil2cpp::Type,
> crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<T1> {
    pub fn BeginInvoke(
        &mut self,
        handler: T1,
        eventData: *mut crate::UnityEngine::EventSystems::BaseEventData,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (handler, eventData, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        handler: T1,
        eventData: *mut crate::UnityEngine::EventSystems::BaseEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (handler, eventData))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
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
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents+EventFunction_1")]
impl<T1: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<T1> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
