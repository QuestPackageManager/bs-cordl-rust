#[cfg(feature = "System+Net+ContextAwareResult")]
#[repr(C)]
#[derive(Debug)]
pub struct ContextAwareResult {
    __cordl_parent: crate::System::Net::LazyAsyncResult,
    pub _context: *mut crate::System::Threading::ExecutionContext,
    pub _lock: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _flags: crate::System::Net::ContextAwareResult_StateFlags,
}
#[cfg(feature = "System+Net+ContextAwareResult")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::ContextAwareResult => "System.Net"
    ."ContextAwareResult"
);
#[cfg(feature = "System+Net+ContextAwareResult")]
impl std::ops::Deref for crate::System::Net::ContextAwareResult {
    type Target = crate::System::Net::LazyAsyncResult;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ContextAwareResult")]
impl std::ops::DerefMut for crate::System::Net::ContextAwareResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ContextAwareResult")]
impl crate::System::Net::ContextAwareResult {
    #[cfg(feature = "System+Net+ContextAwareResult+StateFlags")]
    pub type StateFlags = crate::System::Net::ContextAwareResult_StateFlags;
    pub fn CaptureOrComplete(
        &mut self,
        cachedContext: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Threading::ExecutionContext,
        >,
        returnContext: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CaptureOrComplete", (cachedContext, returnContext))?;
        Ok(__cordl_ret.into())
    }
    pub fn Cleanup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Cleanup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CleanupInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanupInternal", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn CompleteCallback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompleteCallback", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FinishPostingAsyncOp(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("FinishPostingAsyncOp", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppObject_Il2CppObject_AsyncCallback0(
        myObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        myState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        myCallBack: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (myObject, myState, myCallBack))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool__cordl_bool_Il2CppObject_Il2CppObject_AsyncCallback1(
        captureIdentity: bool,
        forceCaptureContext: bool,
        myObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        myState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        myCallBack: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (captureIdentity, forceCaptureContext, myObject, myState, myCallBack),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool__cordl_bool__cordl_bool_Il2CppObject_Il2CppObject_AsyncCallback2(
        captureIdentity: bool,
        forceCaptureContext: bool,
        threadSafeContextCopy: bool,
        myObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        myState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        myCallBack: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    captureIdentity,
                    forceCaptureContext,
                    threadSafeContextCopy,
                    myObject,
                    myState,
                    myCallBack,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn SafeCaptureIdentity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SafeCaptureIdentity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StartPostingAsyncOp_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("StartPostingAsyncOp", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StartPostingAsyncOp__cordl_bool1(
        &mut self,
        lockCapture: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("StartPostingAsyncOp", (lockCapture))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppObject_Il2CppObject_AsyncCallback0(
        &mut self,
        myObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        myState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        myCallBack: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (myObject, myState, myCallBack))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool__cordl_bool_Il2CppObject_Il2CppObject_AsyncCallback1(
        &mut self,
        captureIdentity: bool,
        forceCaptureContext: bool,
        myObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        myState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        myCallBack: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (captureIdentity, forceCaptureContext, myObject, myState, myCallBack),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool__cordl_bool__cordl_bool_Il2CppObject_Il2CppObject_AsyncCallback2(
        &mut self,
        captureIdentity: bool,
        forceCaptureContext: bool,
        threadSafeContextCopy: bool,
        myObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        myState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        myCallBack: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    captureIdentity,
                    forceCaptureContext,
                    threadSafeContextCopy,
                    myObject,
                    myState,
                    myCallBack,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+ContextAwareResult")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::ContextAwareResult {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+ContextAwareResult+StateFlags")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextAwareResult_StateFlags {
    CaptureContext = 2u8,
    CaptureIdentity = 1u8,
    None = 0u8,
    PostBlockFinished = 16u8,
    PostBlockStarted = 8u8,
    ThreadSafeContextCopy = 4u8,
}
#[cfg(feature = "System+Net+ContextAwareResult+StateFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::ContextAwareResult_StateFlags =>
    "System.Net"."ContextAwareResult/StateFlags"
);
