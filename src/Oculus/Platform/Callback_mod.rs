#[cfg(feature = "Oculus+Platform+Callback")]
#[repr(C)]
#[derive(Debug)]
pub struct Callback {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+Callback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Callback => "Oculus.Platform"
    ."Callback"
);
#[cfg(feature = "Oculus+Platform+Callback")]
impl std::ops::Deref for crate::Oculus::Platform::Callback {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Callback")]
impl std::ops::DerefMut for crate::Oculus::Platform::Callback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Callback")]
impl crate::Oculus::Platform::Callback {
    #[cfg(feature = "Oculus+Platform+Callback+RequestCallback")]
    pub type RequestCallback = crate::Oculus::Platform::Callback_RequestCallback;
    #[cfg(feature = "Oculus+Platform+Callback+RequestCallback_1")]
    pub type RequestCallback_1<T: quest_hook::libil2cpp::Type> = crate::Oculus::Platform::Callback_RequestCallback_1<
        T,
    >;
    pub fn AddRequest(
        request: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddRequest", (request))?;
        Ok(__cordl_ret.into())
    }
    pub fn FlushJoinIntentNotificationQueue() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FlushJoinIntentNotificationQueue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMessage(
        msg: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HandleMessage", (msg))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnApplicationQuit() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnApplicationQuit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RunCallbacks() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RunCallbacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RunLimitedCallbacks(
        limit: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RunLimitedCallbacks", (limit))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNotificationCallback_Message_1_Callback0<T>(
        _cordl_type: crate::Oculus::Platform::Message_MessageType,
        callback: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1_Callback<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetNotificationCallback", (_cordl_type, callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNotificationCallback_Message_Callback1(
        _cordl_type: crate::Oculus::Platform::Message_MessageType,
        callback: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message_Callback>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetNotificationCallback", (_cordl_type, callback))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+Callback")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Callback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Oculus+Platform+Callback+RequestCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct Callback_RequestCallback {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub messageCallback: quest_hook::libil2cpp::Gc<
        crate::Oculus::Platform::Message_Callback,
    >,
}
#[cfg(feature = "Oculus+Platform+Callback+RequestCallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Callback_RequestCallback =>
    "Oculus.Platform"."Callback/RequestCallback"
);
#[cfg(feature = "Oculus+Platform+Callback+RequestCallback")]
impl std::ops::Deref for crate::Oculus::Platform::Callback_RequestCallback {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Callback+RequestCallback")]
impl std::ops::DerefMut for crate::Oculus::Platform::Callback_RequestCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Callback+RequestCallback")]
impl crate::Oculus::Platform::Callback_RequestCallback {
    pub fn HandleMessage(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMessage", (msg))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Message_Callback1(
        callback: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message_Callback>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (callback))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Message_Callback1(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message_Callback>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (callback))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+Callback+RequestCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::Callback_RequestCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Oculus+Platform+Callback+RequestCallback_1")]
#[repr(C)]
#[derive(Debug)]
pub struct Callback_RequestCallback_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::Oculus::Platform::Callback_RequestCallback,
    pub callback: quest_hook::libil2cpp::Gc<
        crate::Oculus::Platform::Message_1_Callback<T>,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Oculus+Platform+Callback+RequestCallback_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Callback_RequestCallback_1 < T
    > => "Oculus.Platform"."Callback/RequestCallback`1" < T >
);
#[cfg(feature = "Oculus+Platform+Callback+RequestCallback_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Oculus::Platform::Callback_RequestCallback_1<T> {
    type Target = crate::Oculus::Platform::Callback_RequestCallback;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Callback+RequestCallback_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Oculus::Platform::Callback_RequestCallback_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Callback+RequestCallback_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::Oculus::Platform::Callback_RequestCallback_1<T> {
    pub fn HandleMessage(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMessage", (msg))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        callback: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1_Callback<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (callback))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1_Callback<T>,
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
            .invoke(".ctor", (callback))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+Callback+RequestCallback_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::Callback_RequestCallback_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
