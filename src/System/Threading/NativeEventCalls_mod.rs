#[cfg(feature = "System+Threading+NativeEventCalls")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeEventCalls {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Threading+NativeEventCalls")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Threading::NativeEventCalls {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "NativeEventCalls";
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
#[cfg(feature = "System+Threading+NativeEventCalls")]
impl std::ops::Deref for crate::System::Threading::NativeEventCalls {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::NativeEventCalls as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CloseEvent_internal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::NativeEventCalls as
                    quest_hook::libil2cpp::Type > ::class(), "CloseEvent_internal",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateEvent_icall(
        manual: bool,
        initial: bool,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        name_length: i32,
        errorCode: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::NativeEventCalls as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    bool,
                    bool,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                crate::System::IntPtr,
                5usize,
            >("CreateEvent_icall")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::NativeEventCalls as
                    quest_hook::libil2cpp::Type > ::class(), "CreateEvent_icall", 5usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (manual, initial, name, name_length, errorCode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateEvent_internal(
        manual: bool,
        initial: bool,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        errorCode: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::NativeEventCalls as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    bool,
                    bool,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                crate::System::IntPtr,
                4usize,
            >("CreateEvent_internal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::NativeEventCalls as
                    quest_hook::libil2cpp::Type > ::class(), "CreateEvent_internal",
                    4usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (manual, initial, name, errorCode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResetEvent(
        handle: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeWaitHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::NativeEventCalls as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Microsoft::Win32::SafeHandles::SafeWaitHandle,
                >),
                bool,
                1usize,
            >("ResetEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::NativeEventCalls as
                    quest_hook::libil2cpp::Type > ::class(), "ResetEvent", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (handle))? };
        Ok(__cordl_ret.into())
    }
    pub fn ResetEvent_internal(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::NativeEventCalls as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ResetEvent_internal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::NativeEventCalls as
                    quest_hook::libil2cpp::Type > ::class(), "ResetEvent_internal",
                    1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (handle))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetEvent(
        handle: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeWaitHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::NativeEventCalls as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Microsoft::Win32::SafeHandles::SafeWaitHandle,
                >),
                bool,
                1usize,
            >("SetEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::NativeEventCalls as
                    quest_hook::libil2cpp::Type > ::class(), "SetEvent", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (handle))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetEvent_internal(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::NativeEventCalls as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("SetEvent_internal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::NativeEventCalls as
                    quest_hook::libil2cpp::Type > ::class(), "SetEvent_internal", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (handle))? };
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
