#[cfg(feature = "System+Threading+WaitHandle")]
#[repr(C)]
#[derive(Debug)]
pub struct WaitHandle {
    __cordl_parent: crate::System::MarshalByRefObject,
    pub waitHandle: crate::System::IntPtr,
    pub safeWaitHandle: *mut crate::Microsoft::Win32::SafeHandles::SafeWaitHandle,
    pub hasThreadAffinity: bool,
}
#[cfg(feature = "System+Threading+WaitHandle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::WaitHandle =>
    "System.Threading"."WaitHandle"
);
#[cfg(feature = "System+Threading+WaitHandle")]
impl std::ops::Deref for crate::System::Threading::WaitHandle {
    type Target = crate::System::MarshalByRefObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+WaitHandle")]
impl std::ops::DerefMut for crate::System::Threading::WaitHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+WaitHandle")]
impl crate::System::Threading::WaitHandle {
    pub const ERROR_NOT_OWNED_BY_CALLER: i32 = 299i32;
    pub const ERROR_TOO_MANY_POSTS: i32 = 298i32;
    pub const MAX_WAITHANDLES: i32 = 64i32;
    pub const MaxWaitHandles: i32 = 64i32;
    pub const WAIT_ABANDONED: i32 = 128i32;
    pub const WAIT_FAILED: i32 = 2147483647i32;
    pub const WAIT_OBJECT_0: i32 = 0i32;
    pub const WaitTimeout: i32 = 258i32;
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose__cordl_bool0(
        &mut self,
        explicitDisposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (explicitDisposing))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetHandleInternal(
        &mut self,
        handle: *mut crate::Microsoft::Win32::SafeHandles::SafeWaitHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetHandleInternal", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn WaitOne_2(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("WaitOne", ())?;
        Ok(__cordl_ret)
    }
    pub fn WaitOne_TimeSpan4(
        &mut self,
        timeout: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("WaitOne", (timeout))?;
        Ok(__cordl_ret)
    }
    pub fn WaitOne_TimeSpan__cordl_bool1(
        &mut self,
        timeout: crate::System::TimeSpan,
        exitContext: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("WaitOne", (timeout, exitContext))?;
        Ok(__cordl_ret)
    }
    pub fn WaitOne_i32_3(
        &mut self,
        millisecondsTimeout: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("WaitOne", (millisecondsTimeout))?;
        Ok(__cordl_ret)
    }
    pub fn WaitOne_i32__cordl_bool0(
        &mut self,
        millisecondsTimeout: i32,
        exitContext: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("WaitOne", (millisecondsTimeout, exitContext))?;
        Ok(__cordl_ret)
    }
    pub fn WaitOne_i64__cordl_bool5(
        &mut self,
        timeout: i64,
        exitContext: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("WaitOne", (timeout, exitContext))?;
        Ok(__cordl_ret)
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
    pub fn get_SafeWaitHandle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Microsoft::Win32::SafeHandles::SafeWaitHandle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Microsoft::Win32::SafeHandles::SafeWaitHandle = __cordl_object
            .invoke("get_SafeWaitHandle", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Handle(
        &mut self,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Handle", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Threading+WaitHandle")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::WaitHandle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
