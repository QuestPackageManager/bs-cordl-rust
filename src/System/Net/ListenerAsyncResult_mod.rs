#[cfg(feature = "System+Net+ListenerAsyncResult")]
#[repr(C)]
#[derive(Debug)]
pub struct ListenerAsyncResult {
    __cordl_parent: crate::System::Object,
    pub handle: *mut crate::System::Threading::ManualResetEvent,
    pub synch: bool,
    pub completed: bool,
    pub cb: *mut crate::System::AsyncCallback,
    pub state: *mut crate::System::Object,
    pub exception: *mut crate::System::Exception,
    pub context: *mut crate::System::Net::HttpListenerContext,
    pub locker: *mut crate::System::Object,
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
    type Target = crate::System::Object;
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
    pub fn GetContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::HttpListenerContext> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::HttpListenerContext = __cordl_object
            .invoke("GetContext", ())?;
        Ok(__cordl_ret)
    }
    pub fn Complete_Exception0(
        &mut self,
        exc: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Complete", (exc))?;
        Ok(__cordl_ret)
    }
    pub fn Complete_HttpListenerContext1(
        &mut self,
        context: *mut crate::System::Net::HttpListenerContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Complete", (context))?;
        Ok(__cordl_ret)
    }
    pub fn Complete_HttpListenerContext__cordl_bool2(
        &mut self,
        context: *mut crate::System::Net::HttpListenerContext,
        synch: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Complete", (context, synch))?;
        Ok(__cordl_ret)
    }
    pub fn get_AsyncWaitHandle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::WaitHandle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::WaitHandle = __cordl_object
            .invoke("get_AsyncWaitHandle", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AsyncState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_AsyncState", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CompletedSynchronously(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CompletedSynchronously", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        cb: *mut crate::System::AsyncCallback,
        state: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cb, state))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsCompleted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsCompleted", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        cb: *mut crate::System::AsyncCallback,
        state: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cb, state))?;
        Ok(__cordl_object)
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
