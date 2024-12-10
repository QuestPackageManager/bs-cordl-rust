#[cfg(feature = "System+Net+ListenerAsyncResult")]
#[repr(C)]
#[derive(Debug)]
pub struct ListenerAsyncResult {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub handle: *mut crate::System::Threading::ManualResetEvent,
    pub synch: bool,
    pub completed: bool,
    pub cb: *mut crate::System::AsyncCallback,
    pub state: *mut quest_hook::libil2cpp::Il2CppObject,
    pub exception: *mut crate::System::Exception,
    pub context: *mut crate::System::Net::HttpListenerContext,
    pub locker: *mut quest_hook::libil2cpp::Il2CppObject,
    pub forward: *mut crate::System::Net::ListenerAsyncResult,
    pub EndCalled: bool,
    pub InGet: bool,
}
#[cfg(feature = "System+Net+ListenerAsyncResult")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::ListenerAsyncResult => "System.Net"
    ."ListenerAsyncResult"
);
#[cfg(feature = "System+Net+ListenerAsyncResult")]
impl std::ops::Deref for crate::System::Net::ListenerAsyncResult {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ListenerAsyncResult")]
impl std::ops::DerefMut for crate::System::Net::ListenerAsyncResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ListenerAsyncResult")]
impl crate::System::Net::ListenerAsyncResult {
    pub fn Complete_Exception0(
        &mut self,
        exc: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Complete", (exc))?;
        Ok(__cordl_ret.into())
    }
    pub fn Complete_HttpListenerContext1(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListenerContext>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Complete", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn Complete_HttpListenerContext__cordl_bool2(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListenerContext>,
        synch: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Complete", (context, synch))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::HttpListenerContext>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::HttpListenerContext,
        > = __cordl_object.invoke("GetContext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        cb: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cb, state))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        cb: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cb, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AsyncState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_AsyncState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AsyncWaitHandle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::WaitHandle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::WaitHandle,
        > = __cordl_object.invoke("get_AsyncWaitHandle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CompletedSynchronously(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CompletedSynchronously", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCompleted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsCompleted", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+ListenerAsyncResult")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::ListenerAsyncResult {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
