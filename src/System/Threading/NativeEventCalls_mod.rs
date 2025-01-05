#[cfg(feature = "System+Threading+NativeEventCalls")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeEventCalls {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Threading+NativeEventCalls")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::NativeEventCalls =>
    "System.Threading"."NativeEventCalls"
);
#[cfg(feature = "System+Threading+NativeEventCalls")]
impl std::ops::Deref for crate::System::Threading::NativeEventCalls {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+NativeEventCalls")]
impl std::ops::DerefMut for crate::System::Threading::NativeEventCalls {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+NativeEventCalls")]
impl crate::System::Threading::NativeEventCalls {
    pub fn CloseEvent_internal(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CloseEvent_internal", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateEvent_icall(
        manual: bool,
        initial: bool,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        name_length: i32,
        errorCode: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateEvent_icall",
                (manual, initial, name, name_length, errorCode),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateEvent_internal(
        manual: bool,
        initial: bool,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        errorCode: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateEvent_internal", (manual, initial, name, errorCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetEvent(
        handle: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeWaitHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResetEvent", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetEvent_internal(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResetEvent_internal", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetEvent(
        handle: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeWaitHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetEvent", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetEvent_internal(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetEvent_internal", (handle))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+NativeEventCalls")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::NativeEventCalls {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
