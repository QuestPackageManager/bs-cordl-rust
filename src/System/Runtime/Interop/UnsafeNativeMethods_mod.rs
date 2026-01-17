#[cfg(feature = "cordl_class_System+Runtime+Interop+UnsafeNativeMethods")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct UnsafeNativeMethods {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_System+Runtime+Interop+UnsafeNativeMethods")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Runtime::Interop::UnsafeNativeMethods {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Interop";
    const CLASS_NAME: &'static str = "UnsafeNativeMethods";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "System+Runtime+Interop+UnsafeNativeMethods")]
impl std::ops::Deref for crate::System::Runtime::Interop::UnsafeNativeMethods {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Interop+UnsafeNativeMethods")]
impl std::ops::DerefMut for crate::System::Runtime::Interop::UnsafeNativeMethods {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Interop+UnsafeNativeMethods")]
impl crate::System::Runtime::Interop::UnsafeNativeMethods {
    #[cfg(feature = "System+Runtime+Interop+UnsafeNativeMethods+EtwEnableCallback")]
    pub type EtwEnableCallback =
        crate::System::Runtime::Interop::UnsafeNativeMethods_EtwEnableCallback;
    #[cfg(feature = "System+Runtime+Interop+UnsafeNativeMethods+EventData")]
    pub type EventData = crate::System::Runtime::Interop::UnsafeNativeMethods_EventData;
    pub fn EventActivityIdControl(
        ControlCode: quest_hook::libil2cpp::ByRef<i32>,
        ActivityId: quest_hook::libil2cpp::ByRefMut<crate::System::Guid>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRef<i32>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::Guid>,
                    ), u32, 2usize>("EventActivityIdControl")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EventActivityIdControl",
                            2usize
                        )
                    })
            });
        let __cordl_ret: u32 =
            unsafe { cordl_method_info.invoke_unchecked((), (ControlCode, ActivityId))? };
        Ok(__cordl_ret.into())
    }
    pub fn EventEnabled(
        registrationHandle: quest_hook::libil2cpp::ByRef<i64>,
        eventDescriptor: quest_hook::libil2cpp::ByRefMut<
            crate::System::Runtime::Diagnostics::EventDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRef<i64>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::System::Runtime::Diagnostics::EventDescriptor,
                        >,
                    ), bool, 2usize>("EventEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EventEnabled",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (registrationHandle, eventDescriptor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EventRegister(
        providerId: quest_hook::libil2cpp::ByRefMut<crate::System::Guid>,
        enableCallback: quest_hook::libil2cpp::ByRef<
            quest_hook::libil2cpp::Gc<
                crate::System::Runtime::Interop::UnsafeNativeMethods_EtwEnableCallback,
            >,
        >,
        callbackContext: quest_hook::libil2cpp::ByRef<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        registrationHandle: quest_hook::libil2cpp::ByRefMut<i64>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<crate::System::Guid>,
                            quest_hook::libil2cpp::ByRef<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Runtime::Interop::UnsafeNativeMethods_EtwEnableCallback,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRef<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i64>,
                        ),
                        u32,
                        4usize,
                    >("EventRegister")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EventRegister", 4usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    providerId,
                    enableCallback,
                    callbackContext,
                    registrationHandle,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EventUnregister(
        registrationHandle: quest_hook::libil2cpp::ByRef<i64>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::ByRef<i64>), u32, 1usize>(
                        "EventUnregister",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EventUnregister",
                            1usize
                        )
                    })
            });
        let __cordl_ret: u32 =
            unsafe { cordl_method_info.invoke_unchecked((), (registrationHandle))? };
        Ok(__cordl_ret.into())
    }
    pub fn EventWrite(
        registrationHandle: quest_hook::libil2cpp::ByRef<i64>,
        eventDescriptor: quest_hook::libil2cpp::ByRefMut<
            crate::System::Runtime::Diagnostics::EventDescriptor,
        >,
        userDataCount: quest_hook::libil2cpp::ByRef<u32>,
        userData: quest_hook::libil2cpp::ByRef<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRef<i64>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::System::Runtime::Diagnostics::EventDescriptor,
                        >,
                        quest_hook::libil2cpp::ByRef<u32>,
                        quest_hook::libil2cpp::ByRef<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        >,
                    ), u32, 4usize>("EventWrite")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EventWrite",
                            4usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (registrationHandle, eventDescriptor, userDataCount, userData),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterEventSource(
        uncServerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sourceName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Interop::SafeEventLogWriteHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Interop::SafeEventLogWriteHandle,
                    >, 2usize>("RegisterEventSource")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RegisterEventSource",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Interop::SafeEventLogWriteHandle,
        > = unsafe { cordl_method_info.invoke_unchecked((), (uncServerName, sourceName))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReportEvent(
        hEventLog: quest_hook::libil2cpp::Gc<crate::System::Runtime::InteropServices::SafeHandle>,
        _cordl_type: u16,
        category: u16,
        eventID: u32,
        userSID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        numStrings: u16,
        dataLen: u32,
        strings: crate::System::Runtime::InteropServices::HandleRef,
        rawData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::InteropServices::SafeHandle,
                        >,
                        u16,
                        u16,
                        u32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                        u16,
                        u32,
                        crate::System::Runtime::InteropServices::HandleRef,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    ), bool, 9usize>("ReportEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReportEvent",
                            9usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    hEventLog,
                    _cordl_type,
                    category,
                    eventID,
                    userSID,
                    numStrings,
                    dataLen,
                    strings,
                    rawData,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Runtime+Interop+UnsafeNativeMethods")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Runtime::Interop::UnsafeNativeMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_System+Runtime+Interop+UnsafeNativeMethods+EtwEnableCallback")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct UnsafeNativeMethods_EtwEnableCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "cordl_class_System+Runtime+Interop+UnsafeNativeMethods+EtwEnableCallback")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::System::Runtime::Interop::UnsafeNativeMethods_EtwEnableCallback
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Interop";
    const CLASS_NAME: &'static str = "UnsafeNativeMethods/EtwEnableCallback";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "System+Runtime+Interop+UnsafeNativeMethods+EtwEnableCallback")]
impl std::ops::Deref for crate::System::Runtime::Interop::UnsafeNativeMethods_EtwEnableCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Interop+UnsafeNativeMethods+EtwEnableCallback")]
impl std::ops::DerefMut for crate::System::Runtime::Interop::UnsafeNativeMethods_EtwEnableCallback {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Interop+UnsafeNativeMethods+EtwEnableCallback")]
impl crate::System::Runtime::Interop::UnsafeNativeMethods_EtwEnableCallback {
    pub fn Invoke(
        &mut self,
        sourceId: quest_hook::libil2cpp::ByRefMut<crate::System::Guid>,
        isEnabled: quest_hook::libil2cpp::ByRef<i32>,
        level: quest_hook::libil2cpp::ByRef<u8>,
        matchAnyKeywords: quest_hook::libil2cpp::ByRef<i64>,
        matchAllKeywords: quest_hook::libil2cpp::ByRef<i64>,
        filterData: quest_hook::libil2cpp::ByRef<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        callbackContext: quest_hook::libil2cpp::ByRef<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::Guid>,
                        quest_hook::libil2cpp::ByRef<i32>,
                        quest_hook::libil2cpp::ByRef<u8>,
                        quest_hook::libil2cpp::ByRef<i64>,
                        quest_hook::libil2cpp::ByRef<i64>,
                        quest_hook::libil2cpp::ByRef<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        >,
                        quest_hook::libil2cpp::ByRef<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        >,
                    ), quest_hook::libil2cpp::Void, 7usize>("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Invoke",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    sourceId,
                    isEnabled,
                    level,
                    matchAnyKeywords,
                    matchAllKeywords,
                    filterData,
                    callbackContext,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (object, method))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Runtime+Interop+UnsafeNativeMethods+EtwEnableCallback")]
impl quest_hook::libil2cpp::ObjectType
    for crate::System::Runtime::Interop::UnsafeNativeMethods_EtwEnableCallback
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_System+Runtime+Interop+UnsafeNativeMethods+EventData")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct UnsafeNativeMethods_EventData {
    padding: quest_hook::libil2cpp::ValueTypePadding<16usize>,
}
#[cfg(feature = "cordl_class_System+Runtime+Interop+UnsafeNativeMethods+EventData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::System::Runtime::Interop::UnsafeNativeMethods_EventData
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Runtime.Interop";
    const CLASS_NAME: &'static str = "UnsafeNativeMethods/EventData";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_System+Runtime+Interop+UnsafeNativeMethods+EventData")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::System::Runtime::Interop::UnsafeNativeMethods_EventData
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+Runtime+Interop+UnsafeNativeMethods+EventData")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::System::Runtime::Interop::UnsafeNativeMethods_EventData
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_System+Runtime+Interop+UnsafeNativeMethods+EventData")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::System::Runtime::Interop::UnsafeNativeMethods_EventData
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_System+Runtime+Interop+UnsafeNativeMethods+EventData")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::System::Runtime::Interop::UnsafeNativeMethods_EventData
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_System+Runtime+Interop+UnsafeNativeMethods+EventData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::System::Runtime::Interop::UnsafeNativeMethods_EventData
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Runtime+Interop+UnsafeNativeMethods+EventData")]
impl crate::System::Runtime::Interop::UnsafeNativeMethods_EventData {}
