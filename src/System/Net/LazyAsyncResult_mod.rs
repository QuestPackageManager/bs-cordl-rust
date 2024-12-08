#[cfg(feature = "System+Net+LazyAsyncResult")]
#[repr(C)]
#[derive(Debug)]
pub struct LazyAsyncResult {
    __cordl_parent: crate::System::Object,
    pub m_AsyncObject: *mut crate::System::Object,
    pub m_AsyncState: *mut crate::System::Object,
    pub m_AsyncCallback: *mut crate::System::AsyncCallback,
    pub m_Result: *mut crate::System::Object,
    pub m_IntCompleted: i32,
    pub m_EndCalled: bool,
    pub m_UserEvent: bool,
    pub m_Event: *mut crate::System::Object,
}
#[cfg(feature = "System+Net+LazyAsyncResult")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::LazyAsyncResult => "System.Net"
    ."LazyAsyncResult"
);
#[cfg(feature = "System+Net+LazyAsyncResult")]
impl std::ops::Deref for crate::System::Net::LazyAsyncResult {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+LazyAsyncResult")]
impl std::ops::DerefMut for crate::System::Net::LazyAsyncResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+LazyAsyncResult")]
impl crate::System::Net::LazyAsyncResult {
    #[cfg(feature = "System+Net+LazyAsyncResult+ThreadContext")]
    pub type ThreadContext = crate::System::Net::LazyAsyncResult_ThreadContext;
    pub fn Cleanup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Cleanup", ())?;
        Ok(__cordl_ret)
    }
    pub fn Complete(
        &mut self,
        userToken: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Complete", (userToken))?;
        Ok(__cordl_ret)
    }
    pub fn InternalWaitForCompletion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("InternalWaitForCompletion", ())?;
        Ok(__cordl_ret)
    }
    pub fn InvokeCallback_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeCallback", ())?;
        Ok(__cordl_ret)
    }
    pub fn InvokeCallback_Object0(
        &mut self,
        result: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeCallback", (result))?;
        Ok(__cordl_ret)
    }
    pub fn LazilyCreateEvent(
        &mut self,
        waitHandle: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Threading::ManualResetEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("LazilyCreateEvent", (waitHandle))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        myObject: *mut crate::System::Object,
        myState: *mut crate::System::Object,
        myCallBack: *mut crate::System::AsyncCallback,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (myObject, myState, myCallBack))?;
        Ok(__cordl_object)
    }
    pub fn ProtectedInvokeCallback(
        &mut self,
        result: *mut crate::System::Object,
        userToken: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProtectedInvokeCallback", (result, userToken))?;
        Ok(__cordl_ret)
    }
    pub fn WaitForCompletion(
        &mut self,
        snap: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("WaitForCompletion", (snap))?;
        Ok(__cordl_ret)
    }
    pub fn WorkerThreadComplete(
        &mut self,
        state: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WorkerThreadComplete", (state))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        myObject: *mut crate::System::Object,
        myState: *mut crate::System::Object,
        myCallBack: *mut crate::System::AsyncCallback,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (myObject, myState, myCallBack))?;
        Ok(__cordl_ret)
    }
    pub fn get_AsyncCallback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::AsyncCallback> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::AsyncCallback = __cordl_object
            .invoke("get_AsyncCallback", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AsyncObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_AsyncObject", ())?;
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
    pub fn get_CompletedSynchronously(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CompletedSynchronously", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EndCalled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_EndCalled", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_InternalPeekCompleted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_InternalPeekCompleted", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsCompleted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsCompleted", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_EndCalled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_EndCalled", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+LazyAsyncResult")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::LazyAsyncResult {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+LazyAsyncResult+ThreadContext")]
#[repr(C)]
#[derive(Debug)]
pub struct LazyAsyncResult_ThreadContext {
    __cordl_parent: crate::System::Object,
    pub m_NestedIOCount: i32,
}
#[cfg(feature = "System+Net+LazyAsyncResult+ThreadContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::LazyAsyncResult_ThreadContext =>
    "System.Net"."LazyAsyncResult/ThreadContext"
);
#[cfg(feature = "System+Net+LazyAsyncResult+ThreadContext")]
impl std::ops::Deref for crate::System::Net::LazyAsyncResult_ThreadContext {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+LazyAsyncResult+ThreadContext")]
impl std::ops::DerefMut for crate::System::Net::LazyAsyncResult_ThreadContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+LazyAsyncResult+ThreadContext")]
impl crate::System::Net::LazyAsyncResult_ThreadContext {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "System+Net+LazyAsyncResult+ThreadContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::LazyAsyncResult_ThreadContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
