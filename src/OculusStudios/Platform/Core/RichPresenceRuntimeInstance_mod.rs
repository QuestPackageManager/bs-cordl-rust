#[cfg(feature = "cordl_class_OculusStudios+Platform+Core+RichPresenceRuntimeInstance")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RichPresenceRuntimeInstance {
    pub _definition_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::OculusStudios::Platform::Core::IRichPresenceDefinition,
    >,
    pub _state_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _partyInfo_k__BackingField: crate::System::Nullable_1<
        crate::OculusStudios::Platform::Core::PartyInfo,
    >,
    pub _timestamp_k__BackingField: crate::System::Nullable_1<
        crate::OculusStudios::Platform::Core::RichPresenceTimestamp,
    >,
}
#[cfg(feature = "cordl_class_OculusStudios+Platform+Core+RichPresenceRuntimeInstance")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OculusStudios::Platform::Core::RichPresenceRuntimeInstance {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OculusStudios.Platform.Core";
    const CLASS_NAME: &'static str = "RichPresenceRuntimeInstance";
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
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_OculusStudios+Platform+Core+RichPresenceRuntimeInstance")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::OculusStudios::Platform::Core::RichPresenceRuntimeInstance {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OculusStudios+Platform+Core+RichPresenceRuntimeInstance")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::OculusStudios::Platform::Core::RichPresenceRuntimeInstance {
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
#[cfg(feature = "cordl_class_OculusStudios+Platform+Core+RichPresenceRuntimeInstance")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::OculusStudios::Platform::Core::RichPresenceRuntimeInstance {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_OculusStudios+Platform+Core+RichPresenceRuntimeInstance")]
unsafe impl quest_hook::libil2cpp::Return
for crate::OculusStudios::Platform::Core::RichPresenceRuntimeInstance {
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
#[cfg(feature = "cordl_class_OculusStudios+Platform+Core+RichPresenceRuntimeInstance")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OculusStudios::Platform::Core::RichPresenceRuntimeInstance {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OculusStudios+Platform+Core+RichPresenceRuntimeInstance")]
impl crate::OculusStudios::Platform::Core::RichPresenceRuntimeInstance {
    pub fn _ctor(
        &mut self,
        definition: quest_hook::libil2cpp::Gc<
            crate::OculusStudios::Platform::Core::IRichPresenceDefinition,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        partyInfo: crate::System::Nullable_1<
            crate::OculusStudios::Platform::Core::PartyInfo,
        >,
        timestamp: crate::System::Nullable_1<
            crate::OculusStudios::Platform::Core::RichPresenceTimestamp,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::OculusStudios::Platform::Core::IRichPresenceDefinition,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::System::Nullable_1<
                                crate::OculusStudios::Platform::Core::PartyInfo,
                            >,
                            crate::System::Nullable_1<
                                crate::OculusStudios::Platform::Core::RichPresenceTimestamp,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (definition, state, partyInfo, timestamp))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_definition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::OculusStudios::Platform::Core::IRichPresenceDefinition,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::OculusStudios::Platform::Core::IRichPresenceDefinition,
                        >,
                        0usize,
                    >("get_definition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_definition", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OculusStudios::Platform::Core::IRichPresenceDefinition,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_partyInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::OculusStudios::Platform::Core::PartyInfo>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::System::Nullable_1<
                            crate::OculusStudios::Platform::Core::PartyInfo,
                        >,
                        0usize,
                    >("get_partyInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_partyInfo", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<
            crate::OculusStudios::Platform::Core::PartyInfo,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_state(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_state")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_state", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_timestamp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<
            crate::OculusStudios::Platform::Core::RichPresenceTimestamp,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::System::Nullable_1<
                            crate::OculusStudios::Platform::Core::RichPresenceTimestamp,
                        >,
                        0usize,
                    >("get_timestamp")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_timestamp", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<
            crate::OculusStudios::Platform::Core::RichPresenceTimestamp,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
