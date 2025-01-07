#[cfg(feature = "System+Threading+WaitHandle")]
#[repr(C)]
#[derive(Debug)]
pub struct WaitHandle {
    __cordl_parent: crate::System::MarshalByRefObject,
    pub waitHandle: crate::System::IntPtr,
    pub safeWaitHandle: quest_hook::libil2cpp::Gc<
        crate::Microsoft::Win32::SafeHandles::SafeWaitHandle,
    >,
    pub hasThreadAffinity: bool,
}
#[cfg(feature = "System+Threading+WaitHandle")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Threading::WaitHandle {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "WaitHandle";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
        Ok(__cordl_ret.into())
    }
    pub fn Dispose_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalWaitOne(
        waitableSafeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::InteropServices::SafeHandle,
        >,
        millisecondsTimeout: i64,
        hasThreadAffinity: bool,
        exitContext: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InternalWaitOne",
                (waitableSafeHandle, millisecondsTimeout, hasThreadAffinity, exitContext),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetHandleInternal(
        &mut self,
        handle: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeWaitHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetHandleInternal", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowAbandonedMutexException_0() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowAbandonedMutexException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowAbandonedMutexException_i32_WaitHandle1(
        location: i32,
        handle: quest_hook::libil2cpp::Gc<crate::System::Threading::WaitHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowAbandonedMutexException", (location, handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitAny_TimeSpan1(
        waitHandles: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Threading::WaitHandle>,
            >,
        >,
        timeout: crate::System::TimeSpan,
        exitContext: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WaitAny", (waitHandles, timeout, exitContext))?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitAny_i32_0(
        waitHandles: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Threading::WaitHandle>,
            >,
        >,
        millisecondsTimeout: i32,
        exitContext: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WaitAny", (waitHandles, millisecondsTimeout, exitContext))?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitMultiple(
        waitHandles: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Threading::WaitHandle>,
            >,
        >,
        millisecondsTimeout: i32,
        exitContext: bool,
        WaitAll: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "WaitMultiple",
                (waitHandles, millisecondsTimeout, exitContext, WaitAll),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitOneNative(
        waitableSafeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::InteropServices::SafeHandle,
        >,
        millisecondsTimeout: u32,
        hasThreadAffinity: bool,
        exitContext: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "WaitOneNative",
                (waitableSafeHandle, millisecondsTimeout, hasThreadAffinity, exitContext),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitOne_2(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("WaitOne", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitOne_TimeSpan4(
        &mut self,
        timeout: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("WaitOne", (timeout))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn WaitOne_i32_3(
        &mut self,
        millisecondsTimeout: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("WaitOne", (millisecondsTimeout))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn Wait_internal(
        handles: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        numHandles: i32,
        waitAll: bool,
        ms: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Wait_internal", (handles, numHandles, waitAll, ms))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SafeWaitHandle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Microsoft::Win32::SafeHandles::SafeWaitHandle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeWaitHandle,
        > = __cordl_object.invoke("get_SafeWaitHandle", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+Threading+WaitHandle")]
impl AsRef<crate::System::IDisposable> for crate::System::Threading::WaitHandle {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Threading+WaitHandle")]
impl AsMut<crate::System::IDisposable> for crate::System::Threading::WaitHandle {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
