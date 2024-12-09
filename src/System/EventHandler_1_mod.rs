#[cfg(feature = "System+EventHandler_1")]
#[repr(C)]
#[derive(Debug)]
pub struct EventHandler_1<TEventArgs: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::MulticastDelegate,
    __cordl_phantom_TEventArgs: std::marker::PhantomData<TEventArgs>,
}
#[cfg(feature = "System+EventHandler_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::EventHandler_1 < TEventArgs > =>
    "System"."EventHandler`1" < TEventArgs >
);
#[cfg(feature = "System+EventHandler_1")]
impl<TEventArgs: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::EventHandler_1<TEventArgs> {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+EventHandler_1")]
impl<TEventArgs: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::EventHandler_1<TEventArgs> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+EventHandler_1")]
impl<TEventArgs: quest_hook::libil2cpp::Type> crate::System::EventHandler_1<TEventArgs> {
    pub fn Invoke(
        &mut self,
        sender: *mut crate::System::Object,
        e: TEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TEventArgs: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (sender, e))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        TEventArgs: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
        TEventArgs: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
#[cfg(feature = "System+EventHandler_1")]
impl<TEventArgs: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::EventHandler_1<TEventArgs> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
