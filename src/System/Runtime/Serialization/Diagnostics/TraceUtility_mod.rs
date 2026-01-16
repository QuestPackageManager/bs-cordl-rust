#[cfg(feature = "cordl_class_System+Runtime+Serialization+Diagnostics+TraceUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct TraceUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_System+Runtime+Serialization+Diagnostics+TraceUtility")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Serialization::Diagnostics::TraceUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Serialization.Diagnostics";
    const CLASS_NAME: &'static str = "TraceUtility";
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
#[cfg(feature = "System+Runtime+Serialization+Diagnostics+TraceUtility")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Diagnostics::TraceUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Diagnostics+TraceUtility")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Diagnostics::TraceUtility {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Diagnostics+TraceUtility")]
impl crate::System::Runtime::Serialization::Diagnostics::TraceUtility {
    pub fn Trace_TraceEventType_i32_Il2CppString0(
        severity: crate::System::Diagnostics::TraceEventType,
        traceCode: i32,
        traceDescription: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::Diagnostics::TraceEventType,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("Trace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Trace",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (severity, traceCode, traceDescription))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Trace_TraceRecord1(
        severity: crate::System::Diagnostics::TraceEventType,
        traceCode: i32,
        traceDescription: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        record: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::TraceRecord,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::Diagnostics::TraceEventType,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Diagnostics::TraceRecord,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Trace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Trace",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (severity, traceCode, traceDescription, record))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Trace_TraceRecord_Exception2(
        severity: crate::System::Diagnostics::TraceEventType,
        traceCode: i32,
        traceDescription: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        record: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::TraceRecord,
        >,
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::Diagnostics::TraceEventType,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Diagnostics::TraceRecord,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Exception>,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("Trace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Trace",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (severity, traceCode, traceDescription, record, exception),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Runtime+Serialization+Diagnostics+TraceUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Diagnostics::TraceUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
