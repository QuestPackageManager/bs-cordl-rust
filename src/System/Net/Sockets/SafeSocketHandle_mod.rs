#[cfg(feature = "System+Net+Sockets+SafeSocketHandle")]
#[repr(C)]
#[derive(Debug)]
pub struct SafeSocketHandle {
    __cordl_parent: crate::Microsoft::Win32::SafeHandles::SafeHandleMinusOneIsInvalid,
    pub blocking_threads: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::System::Threading::Thread>,
        >,
    >,
    pub threads_stacktraces: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::System::Threading::Thread>,
            quest_hook::libil2cpp::Gc<crate::System::Diagnostics::StackTrace>,
        >,
    >,
    pub in_cleanup: bool,
}
#[cfg(feature = "System+Net+Sockets+SafeSocketHandle")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::Sockets::SafeSocketHandle {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net.Sockets";
    const CLASS_NAME: &'static str = "SafeSocketHandle";
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
#[cfg(feature = "System+Net+Sockets+SafeSocketHandle")]
impl std::ops::Deref for crate::System::Net::Sockets::SafeSocketHandle {
    type Target = crate::Microsoft::Win32::SafeHandles::SafeHandleMinusOneIsInvalid;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Sockets+SafeSocketHandle")]
impl std::ops::DerefMut for crate::System::Net::Sockets::SafeSocketHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Sockets+SafeSocketHandle")]
impl crate::System::Net::Sockets::SafeSocketHandle {
    pub const ABORT_RETRIES: i32 = 10i32;
    pub const SOCKET_CLOSED: i32 = 10004i32;
    pub fn New(
        preexistingHandle: crate::System::IntPtr,
        ownsHandle: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (preexistingHandle, ownsHandle))?;
        Ok(__cordl_object.into())
    }
    pub fn RegisterForBlockingSyscall(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("RegisterForBlockingSyscall")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterForBlockingSyscall", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseHandle(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("ReleaseHandle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReleaseHandle", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn UnRegisterForBlockingSyscall(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("UnRegisterForBlockingSyscall")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UnRegisterForBlockingSyscall", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        preexistingHandle: crate::System::IntPtr,
        ownsHandle: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::IntPtr, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (preexistingHandle, ownsHandle))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Sockets+SafeSocketHandle")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Sockets::SafeSocketHandle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
