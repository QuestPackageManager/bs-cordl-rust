#[cfg(feature = "System+Diagnostics+TraceFilter")]
#[repr(C)]
#[derive(Debug)]
pub struct TraceFilter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Diagnostics+TraceFilter")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Diagnostics::TraceFilter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Diagnostics";
    const CLASS_NAME: &'static str = "TraceFilter";
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
#[cfg(feature = "System+Diagnostics+TraceFilter")]
impl std::ops::Deref for crate::System::Diagnostics::TraceFilter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+TraceFilter")]
impl std::ops::DerefMut for crate::System::Diagnostics::TraceFilter {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+TraceFilter")]
impl crate::System::Diagnostics::TraceFilter {
    pub fn ShouldTrace_Il2CppArray_Il2CppObject_Il2CppArray0(
        &mut self,
        cache: quest_hook::libil2cpp::Gc<crate::System::Diagnostics::TraceEventCache>,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        eventType: crate::System::Diagnostics::TraceEventType,
        id: i32,
        formatOrMessage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        data1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        data: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Diagnostics::TraceEventCache,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::System::Diagnostics::TraceEventType,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppObject,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppObject,
                                    >,
                                >,
                            >,
                        ),
                        bool,
                        8usize,
                    >("ShouldTrace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ShouldTrace", 8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (cache, source, eventType, id, formatOrMessage, args, data1, data),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldTrace_TraceEventCache_Il2CppString_TraceEventType_i32_Il2CppString1(
        &mut self,
        cache: quest_hook::libil2cpp::Gc<crate::System::Diagnostics::TraceEventCache>,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        eventType: crate::System::Diagnostics::TraceEventType,
        id: i32,
        formatOrMessage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Diagnostics::TraceEventCache,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::System::Diagnostics::TraceEventType,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        bool,
                        5usize,
                    >("ShouldTrace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ShouldTrace", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(self, (cache, source, eventType, id, formatOrMessage))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Diagnostics+TraceFilter")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Diagnostics::TraceFilter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
