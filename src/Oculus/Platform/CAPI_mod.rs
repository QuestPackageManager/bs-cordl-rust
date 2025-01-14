#[cfg(feature = "Oculus+Platform+CAPI")]
#[repr(C)]
#[derive(Debug)]
pub struct CAPI {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+CAPI")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Platform::CAPI {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "CAPI";
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
#[cfg(feature = "Oculus+Platform+CAPI")]
impl std::ops::Deref for crate::Oculus::Platform::CAPI {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+CAPI")]
impl std::ops::DerefMut for crate::Oculus::Platform::CAPI {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+CAPI")]
impl crate::Oculus::Platform::CAPI {
    pub const DLL_NAME: &'static str = "ovrplatformloader";
    pub const VoipFilterBufferSize: i32 = 480i32;
    #[cfg(feature = "Oculus+Platform+CAPI+FilterCallback")]
    pub type FilterCallback = crate::Oculus::Platform::CAPI_FilterCallback;
    #[cfg(feature = "Oculus+Platform+CAPI+OculusInitParams")]
    pub type OculusInitParams = crate::Oculus::Platform::CAPI_OculusInitParams;
    #[cfg(feature = "Oculus+Platform+CAPI+ovrKeyValuePair")]
    pub type ovrKeyValuePair = crate::Oculus::Platform::CAPI_ovrKeyValuePair;
    #[cfg(feature = "Oculus+Platform+CAPI+ovrNetSyncVec3")]
    pub type ovrNetSyncVec3 = crate::Oculus::Platform::CAPI_ovrNetSyncVec3;
    pub fn ArrayOfStructsToIntPtr(
        ar: quest_hook::libil2cpp::Gc<crate::System::Array>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Array>),
                crate::System::IntPtr,
                1usize,
            >("ArrayOfStructsToIntPtr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ArrayOfStructsToIntPtr", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (ar))
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlobFromNative(
        _cordl_size: u32,
        pointer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u32, crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                2usize,
            >("BlobFromNative")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "BlobFromNative", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked((), (_cordl_size, pointer)) };
        Ok(__cordl_ret.into())
    }
    pub fn DataStoreFromNative(
        pointer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::Dictionary_2<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
                1usize,
            >("DataStoreFromNative")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DataStoreFromNative", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = unsafe { method.invoke_unchecked((), (pointer)) };
        Ok(__cordl_ret.into())
    }
    pub fn DateTimeFromNative(
        seconds_since_the_one_true_epoch: u64,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                crate::System::DateTime,
                1usize,
            >("DateTimeFromNative")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DateTimeFromNative", 1usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), (seconds_since_the_one_true_epoch))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DateTimeToNative(
        dt: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::DateTime),
                u64,
                1usize,
            >("DateTimeToNative")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DateTimeToNative", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (dt)) };
        Ok(__cordl_ret.into())
    }
    pub fn DictionaryToOVRKeyValuePairs_Dictionary_2_0(
        dict: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::Oculus::Platform::InitConfigOptions,
                bool,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::Oculus::Platform::CAPI_ovrKeyValuePair,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::Dictionary_2<
                        crate::Oculus::Platform::InitConfigOptions,
                        bool,
                    >,
                >),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        crate::Oculus::Platform::CAPI_ovrKeyValuePair,
                    >,
                >,
                1usize,
            >("DictionaryToOVRKeyValuePairs")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DictionaryToOVRKeyValuePairs", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::Oculus::Platform::CAPI_ovrKeyValuePair,
            >,
        > = unsafe { method.invoke_unchecked((), (dict)) };
        Ok(__cordl_ret.into())
    }
    pub fn DictionaryToOVRKeyValuePairs_Dictionary_2_1(
        dict: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::Oculus::Platform::CAPI_ovrKeyValuePair,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::Dictionary_2<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    >,
                >),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        crate::Oculus::Platform::CAPI_ovrKeyValuePair,
                    >,
                >,
                1usize,
            >("DictionaryToOVRKeyValuePairs")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DictionaryToOVRKeyValuePairs", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::Oculus::Platform::CAPI_ovrKeyValuePair,
            >,
        > = unsafe { method.invoke_unchecked((), (dict)) };
        Ok(__cordl_ret.into())
    }
    pub fn FiledataFromNative(
        _cordl_size: u32,
        pointer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u32, crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                2usize,
            >("FiledataFromNative")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FiledataFromNative", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked((), (_cordl_size, pointer)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetNativeStringLengthNotIncludingNullTerminator(
        pointer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                i32,
                1usize,
            >("GetNativeStringLengthNotIncludingNullTerminator")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetNativeStringLengthNotIncludingNullTerminator", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (pointer)) };
        Ok(__cordl_ret.into())
    }
    pub fn IntPtrToByteArray(
        data: crate::System::IntPtr,
        _cordl_size: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, u64),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                2usize,
            >("IntPtrToByteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IntPtrToByteArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked((), (data, _cordl_size)) };
        Ok(__cordl_ret.into())
    }
    pub fn LogNewEvent(
        eventName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::Dictionary_2<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("LogNewEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LogNewEvent", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (eventName, values))
        };
        Ok(__cordl_ret.into())
    }
    pub fn LogNewUnifiedEvent(
        eventName: crate::Oculus::Platform::LogEventName,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Oculus::Platform::LogEventName,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::Dictionary_2<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("LogNewUnifiedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LogNewUnifiedEvent", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (eventName, values))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn StringFromNative(
        pointer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("StringFromNative")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "StringFromNative", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (pointer)) };
        Ok(__cordl_ret.into())
    }
    pub fn StringToNative(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                crate::System::IntPtr,
                1usize,
            >("StringToNative")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "StringToNative", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (s))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AbuseReportOptions_Create() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::System::IntPtr,
                0usize,
            >("ovr_AbuseReportOptions_Create")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AbuseReportOptions_Create", 0usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AbuseReportOptions_Destroy(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_AbuseReportOptions_Destroy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AbuseReportOptions_Destroy", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AbuseReportOptions_SetPreventPeopleChooser(
        handle: crate::System::IntPtr,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_AbuseReportOptions_SetPreventPeopleChooser")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AbuseReportOptions_SetPreventPeopleChooser", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AbuseReportOptions_SetReportType(
        handle: crate::System::IntPtr,
        value: crate::Oculus::Platform::AbuseReportType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::Oculus::Platform::AbuseReportType),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_AbuseReportOptions_SetReportType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AbuseReportOptions_SetReportType", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AbuseReportRecording_GetRecordingUuid(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_AbuseReportRecording_GetRecordingUuid")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AbuseReportRecording_GetRecordingUuid", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AbuseReportRecording_GetRecordingUuid_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_AbuseReportRecording_GetRecordingUuid_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AbuseReportRecording_GetRecordingUuid_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AbuseReport_LaunchAdvancedReportFlow(
        content_id: u64,
        abuse_report_options: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64, crate::System::IntPtr),
                u64,
                2usize,
            >("ovr_AbuseReport_LaunchAdvancedReportFlow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AbuseReport_LaunchAdvancedReportFlow", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (content_id, abuse_report_options))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AbuseReport_ReportRequestHandled(
        response: crate::Oculus::Platform::ReportRequestResponse,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Oculus::Platform::ReportRequestResponse),
                u64,
                1usize,
            >("ovr_AbuseReport_ReportRequestHandled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AbuseReport_ReportRequestHandled", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (response)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementDefinitionArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::UIntPtr),
                crate::System::IntPtr,
                2usize,
            >("ovr_AchievementDefinitionArray_GetElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AchievementDefinitionArray_GetElement", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementDefinitionArray_GetNextUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_AchievementDefinitionArray_GetNextUrl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AchievementDefinitionArray_GetNextUrl", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementDefinitionArray_GetNextUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_AchievementDefinitionArray_GetNextUrl_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AchievementDefinitionArray_GetNextUrl_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementDefinitionArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::UIntPtr,
                1usize,
            >("ovr_AchievementDefinitionArray_GetSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AchievementDefinitionArray_GetSize", 1usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementDefinitionArray_HasNextPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_AchievementDefinitionArray_HasNextPage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AchievementDefinitionArray_HasNextPage", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementDefinition_GetBitfieldLength(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u32,
                1usize,
            >("ovr_AchievementDefinition_GetBitfieldLength")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AchievementDefinition_GetBitfieldLength", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementDefinition_GetName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_AchievementDefinition_GetName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AchievementDefinition_GetName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementDefinition_GetName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_AchievementDefinition_GetName_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AchievementDefinition_GetName_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementDefinition_GetTarget(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_AchievementDefinition_GetTarget")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AchievementDefinition_GetTarget", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementDefinition_GetType(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::AchievementType> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::Oculus::Platform::AchievementType,
                1usize,
            >("ovr_AchievementDefinition_GetType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AchievementDefinition_GetType", 1usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::AchievementType = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementProgressArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::UIntPtr),
                crate::System::IntPtr,
                2usize,
            >("ovr_AchievementProgressArray_GetElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AchievementProgressArray_GetElement", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementProgressArray_GetNextUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_AchievementProgressArray_GetNextUrl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AchievementProgressArray_GetNextUrl", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementProgressArray_GetNextUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_AchievementProgressArray_GetNextUrl_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AchievementProgressArray_GetNextUrl_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementProgressArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::UIntPtr,
                1usize,
            >("ovr_AchievementProgressArray_GetSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AchievementProgressArray_GetSize", 1usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementProgressArray_HasNextPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_AchievementProgressArray_HasNextPage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AchievementProgressArray_HasNextPage", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementProgress_GetBitfield(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_AchievementProgress_GetBitfield")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AchievementProgress_GetBitfield", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementProgress_GetBitfield_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_AchievementProgress_GetBitfield_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AchievementProgress_GetBitfield_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementProgress_GetCount(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_AchievementProgress_GetCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AchievementProgress_GetCount", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementProgress_GetIsUnlocked(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_AchievementProgress_GetIsUnlocked")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AchievementProgress_GetIsUnlocked", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementProgress_GetName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_AchievementProgress_GetName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AchievementProgress_GetName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementProgress_GetName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_AchievementProgress_GetName_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AchievementProgress_GetName_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementProgress_GetUnlockTime(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::DateTime,
                1usize,
            >("ovr_AchievementProgress_GetUnlockTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AchievementProgress_GetUnlockTime", 1usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementProgress_GetUnlockTime_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_AchievementProgress_GetUnlockTime_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AchievementProgress_GetUnlockTime_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementUpdate_GetJustUnlocked(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_AchievementUpdate_GetJustUnlocked")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AchievementUpdate_GetJustUnlocked", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementUpdate_GetName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_AchievementUpdate_GetName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AchievementUpdate_GetName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementUpdate_GetName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_AchievementUpdate_GetName_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AchievementUpdate_GetName_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Achievements_AddCount(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        count: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, u64),
                u64,
                2usize,
            >("ovr_Achievements_AddCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Achievements_AddCount", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (name, count)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Achievements_AddCount_Native(
        name: crate::System::IntPtr,
        count: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, u64),
                u64,
                2usize,
            >("ovr_Achievements_AddCount_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Achievements_AddCount_Native", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (name, count)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Achievements_AddFields(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fields: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                u64,
                2usize,
            >("ovr_Achievements_AddFields")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Achievements_AddFields", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (name, fields)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Achievements_AddFields_Native(
        name: crate::System::IntPtr,
        fields: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                u64,
                2usize,
            >("ovr_Achievements_AddFields_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Achievements_AddFields_Native", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (name, fields)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Achievements_GetAllDefinitions() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_Achievements_GetAllDefinitions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Achievements_GetAllDefinitions", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Achievements_GetAllProgress() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_Achievements_GetAllProgress")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Achievements_GetAllProgress", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Achievements_GetDefinitionsByName(
        names: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        >,
                    >,
                    i32,
                ),
                u64,
                2usize,
            >("ovr_Achievements_GetDefinitionsByName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Achievements_GetDefinitionsByName", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (names, count)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Achievements_GetProgressByName(
        names: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        >,
                    >,
                    i32,
                ),
                u64,
                2usize,
            >("ovr_Achievements_GetProgressByName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Achievements_GetProgressByName", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (names, count)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Achievements_Unlock(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ovr_Achievements_Unlock")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Achievements_Unlock", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (name)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Achievements_Unlock_Native(
        name: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_Achievements_Unlock_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Achievements_Unlock_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (name)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AdvancedAbuseReportOptions_AddSuggestedUser(
        handle: crate::System::IntPtr,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, u64),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_AdvancedAbuseReportOptions_AddSuggestedUser")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AdvancedAbuseReportOptions_AddSuggestedUser", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AdvancedAbuseReportOptions_ClearDeveloperDefinedContext(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_AdvancedAbuseReportOptions_ClearDeveloperDefinedContext")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AdvancedAbuseReportOptions_ClearDeveloperDefinedContext",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AdvancedAbuseReportOptions_ClearSuggestedUsers(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_AdvancedAbuseReportOptions_ClearSuggestedUsers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AdvancedAbuseReportOptions_ClearSuggestedUsers", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AdvancedAbuseReportOptions_Create() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::System::IntPtr,
                0usize,
            >("ovr_AdvancedAbuseReportOptions_Create")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AdvancedAbuseReportOptions_Create", 0usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AdvancedAbuseReportOptions_Destroy(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_AdvancedAbuseReportOptions_Destroy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AdvancedAbuseReportOptions_Destroy", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AdvancedAbuseReportOptions_SetDeveloperDefinedContextString(
        handle: crate::System::IntPtr,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("ovr_AdvancedAbuseReportOptions_SetDeveloperDefinedContextString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self,
                    "ovr_AdvancedAbuseReportOptions_SetDeveloperDefinedContextString",
                    3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, key, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AdvancedAbuseReportOptions_SetDeveloperDefinedContextString_Native(
        handle: crate::System::IntPtr,
        key: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                3usize,
            >("ovr_AdvancedAbuseReportOptions_SetDeveloperDefinedContextString_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self,
                    "ovr_AdvancedAbuseReportOptions_SetDeveloperDefinedContextString_Native",
                    3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, key, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AdvancedAbuseReportOptions_SetObjectType(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_AdvancedAbuseReportOptions_SetObjectType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AdvancedAbuseReportOptions_SetObjectType", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AdvancedAbuseReportOptions_SetObjectType_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_AdvancedAbuseReportOptions_SetObjectType_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AdvancedAbuseReportOptions_SetObjectType_Native", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AdvancedAbuseReportOptions_SetReportType(
        handle: crate::System::IntPtr,
        value: crate::Oculus::Platform::AbuseReportType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::Oculus::Platform::AbuseReportType),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_AdvancedAbuseReportOptions_SetReportType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AdvancedAbuseReportOptions_SetReportType", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AdvancedAbuseReportOptions_SetVideoMode(
        handle: crate::System::IntPtr,
        value: crate::Oculus::Platform::AbuseReportVideoMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::Oculus::Platform::AbuseReportVideoMode),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_AdvancedAbuseReportOptions_SetVideoMode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AdvancedAbuseReportOptions_SetVideoMode", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AppDownloadProgressResult_GetDownloadBytes(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                i64,
                1usize,
            >("ovr_AppDownloadProgressResult_GetDownloadBytes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AppDownloadProgressResult_GetDownloadBytes", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AppDownloadProgressResult_GetDownloadedBytes(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                i64,
                1usize,
            >("ovr_AppDownloadProgressResult_GetDownloadedBytes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AppDownloadProgressResult_GetDownloadedBytes", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AppDownloadProgressResult_GetStatusCode(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::AppStatus> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::Oculus::Platform::AppStatus,
                1usize,
            >("ovr_AppDownloadProgressResult_GetStatusCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AppDownloadProgressResult_GetStatusCode", 1usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::AppStatus = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AppDownloadResult_GetTimestamp(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                i64,
                1usize,
            >("ovr_AppDownloadResult_GetTimestamp")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AppDownloadResult_GetTimestamp", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationInviteArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::UIntPtr),
                crate::System::IntPtr,
                2usize,
            >("ovr_ApplicationInviteArray_GetElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationInviteArray_GetElement", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationInviteArray_GetNextUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_ApplicationInviteArray_GetNextUrl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationInviteArray_GetNextUrl", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationInviteArray_GetNextUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_ApplicationInviteArray_GetNextUrl_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationInviteArray_GetNextUrl_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationInviteArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::UIntPtr,
                1usize,
            >("ovr_ApplicationInviteArray_GetSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationInviteArray_GetSize", 1usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationInviteArray_HasNextPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_ApplicationInviteArray_HasNextPage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationInviteArray_HasNextPage", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationInvite_GetDestination(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_ApplicationInvite_GetDestination")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationInvite_GetDestination", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationInvite_GetID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_ApplicationInvite_GetID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationInvite_GetID", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationInvite_GetIsActive(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_ApplicationInvite_GetIsActive")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationInvite_GetIsActive", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationInvite_GetLobbySessionId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_ApplicationInvite_GetLobbySessionId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationInvite_GetLobbySessionId", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationInvite_GetLobbySessionId_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_ApplicationInvite_GetLobbySessionId_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationInvite_GetLobbySessionId_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationInvite_GetMatchSessionId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_ApplicationInvite_GetMatchSessionId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationInvite_GetMatchSessionId", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationInvite_GetMatchSessionId_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_ApplicationInvite_GetMatchSessionId_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationInvite_GetMatchSessionId_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationInvite_GetRecipient(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_ApplicationInvite_GetRecipient")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationInvite_GetRecipient", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationLifecycle_GetLaunchDetails() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::System::IntPtr,
                0usize,
            >("ovr_ApplicationLifecycle_GetLaunchDetails")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationLifecycle_GetLaunchDetails", 0usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationLifecycle_GetRegisteredPIDs() -> quest_hook::libil2cpp::Result<
        u64,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                u64,
                0usize,
            >("ovr_ApplicationLifecycle_GetRegisteredPIDs")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationLifecycle_GetRegisteredPIDs", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationLifecycle_GetSessionKey() -> quest_hook::libil2cpp::Result<
        u64,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                u64,
                0usize,
            >("ovr_ApplicationLifecycle_GetSessionKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationLifecycle_GetSessionKey", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationLifecycle_LogDeeplinkResult(
        trackingID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: crate::Oculus::Platform::LaunchResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::Oculus::Platform::LaunchResult,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_ApplicationLifecycle_LogDeeplinkResult")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationLifecycle_LogDeeplinkResult", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (trackingID, result))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationLifecycle_LogDeeplinkResult_Native(
        trackingID: crate::System::IntPtr,
        result: crate::Oculus::Platform::LaunchResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::Oculus::Platform::LaunchResult),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_ApplicationLifecycle_LogDeeplinkResult_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationLifecycle_LogDeeplinkResult_Native", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (trackingID, result))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationLifecycle_RegisterSessionKey(
        sessionKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ovr_ApplicationLifecycle_RegisterSessionKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationLifecycle_RegisterSessionKey", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (sessionKey)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationLifecycle_RegisterSessionKey_Native(
        sessionKey: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_ApplicationLifecycle_RegisterSessionKey_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationLifecycle_RegisterSessionKey_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (sessionKey)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationOptions_Create() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::System::IntPtr,
                0usize,
            >("ovr_ApplicationOptions_Create")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationOptions_Create", 0usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationOptions_Destroy(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_ApplicationOptions_Destroy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationOptions_Destroy", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationOptions_SetDeeplinkMessage(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_ApplicationOptions_SetDeeplinkMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationOptions_SetDeeplinkMessage", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationOptions_SetDeeplinkMessage_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_ApplicationOptions_SetDeeplinkMessage_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationOptions_SetDeeplinkMessage_Native", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationOptions_SetDestinationApiName(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_ApplicationOptions_SetDestinationApiName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationOptions_SetDestinationApiName", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationOptions_SetDestinationApiName_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_ApplicationOptions_SetDestinationApiName_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationOptions_SetDestinationApiName_Native", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationOptions_SetLobbySessionId(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_ApplicationOptions_SetLobbySessionId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationOptions_SetLobbySessionId", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationOptions_SetLobbySessionId_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_ApplicationOptions_SetLobbySessionId_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationOptions_SetLobbySessionId_Native", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationOptions_SetMatchSessionId(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_ApplicationOptions_SetMatchSessionId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationOptions_SetMatchSessionId", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationOptions_SetMatchSessionId_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_ApplicationOptions_SetMatchSessionId_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationOptions_SetMatchSessionId_Native", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationOptions_SetRoomId(
        handle: crate::System::IntPtr,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, u64),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_ApplicationOptions_SetRoomId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationOptions_SetRoomId", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationVersion_GetCurrentCode(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                i32,
                1usize,
            >("ovr_ApplicationVersion_GetCurrentCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationVersion_GetCurrentCode", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationVersion_GetCurrentName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_ApplicationVersion_GetCurrentName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationVersion_GetCurrentName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationVersion_GetCurrentName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_ApplicationVersion_GetCurrentName_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationVersion_GetCurrentName_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationVersion_GetLatestCode(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                i32,
                1usize,
            >("ovr_ApplicationVersion_GetLatestCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationVersion_GetLatestCode", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationVersion_GetLatestName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_ApplicationVersion_GetLatestName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationVersion_GetLatestName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationVersion_GetLatestName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_ApplicationVersion_GetLatestName_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationVersion_GetLatestName_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationVersion_GetReleaseDate(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                i64,
                1usize,
            >("ovr_ApplicationVersion_GetReleaseDate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationVersion_GetReleaseDate", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationVersion_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_ApplicationVersion_GetSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationVersion_GetSize", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationVersion_GetSize_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_ApplicationVersion_GetSize_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ApplicationVersion_GetSize_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Application_CancelAppDownload() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_Application_CancelAppDownload")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Application_CancelAppDownload", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Application_CheckAppDownloadProgress() -> quest_hook::libil2cpp::Result<
        u64,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                u64,
                0usize,
            >("ovr_Application_CheckAppDownloadProgress")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Application_CheckAppDownloadProgress", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Application_GetID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_Application_GetID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Application_GetID", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Application_GetInstalledApplications() -> quest_hook::libil2cpp::Result<
        u64,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                u64,
                0usize,
            >("ovr_Application_GetInstalledApplications")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Application_GetInstalledApplications", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Application_GetVersion() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_Application_GetVersion")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Application_GetVersion", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Application_InstallAppUpdateAndRelaunch(
        deeplink_options: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_Application_InstallAppUpdateAndRelaunch")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Application_InstallAppUpdateAndRelaunch", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (deeplink_options))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Application_LaunchOtherApp(
        appID: u64,
        deeplink_options: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64, crate::System::IntPtr),
                u64,
                2usize,
            >("ovr_Application_LaunchOtherApp")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Application_LaunchOtherApp", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (appID, deeplink_options))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Application_StartAppDownload() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_Application_StartAppDownload")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Application_StartAppDownload", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetDetailsArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::UIntPtr),
                crate::System::IntPtr,
                2usize,
            >("ovr_AssetDetailsArray_GetElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetDetailsArray_GetElement", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetDetailsArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::UIntPtr,
                1usize,
            >("ovr_AssetDetailsArray_GetSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetDetailsArray_GetSize", 1usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetDetails_GetAssetId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_AssetDetails_GetAssetId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetDetails_GetAssetId", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetDetails_GetAssetType(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_AssetDetails_GetAssetType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetDetails_GetAssetType", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetDetails_GetAssetType_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_AssetDetails_GetAssetType_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetDetails_GetAssetType_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetDetails_GetDownloadStatus(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_AssetDetails_GetDownloadStatus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetDetails_GetDownloadStatus", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetDetails_GetDownloadStatus_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_AssetDetails_GetDownloadStatus_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetDetails_GetDownloadStatus_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetDetails_GetFilepath(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_AssetDetails_GetFilepath")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetDetails_GetFilepath", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetDetails_GetFilepath_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_AssetDetails_GetFilepath_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetDetails_GetFilepath_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetDetails_GetIapStatus(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_AssetDetails_GetIapStatus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetDetails_GetIapStatus", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetDetails_GetIapStatus_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_AssetDetails_GetIapStatus_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetDetails_GetIapStatus_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetDetails_GetLanguage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_AssetDetails_GetLanguage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetDetails_GetLanguage", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetDetails_GetMetadata(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_AssetDetails_GetMetadata")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetDetails_GetMetadata", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetDetails_GetMetadata_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_AssetDetails_GetMetadata_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetDetails_GetMetadata_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDeleteResult_GetAssetFileId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_AssetFileDeleteResult_GetAssetFileId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFileDeleteResult_GetAssetFileId", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDeleteResult_GetAssetId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_AssetFileDeleteResult_GetAssetId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFileDeleteResult_GetAssetId", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDeleteResult_GetFilepath(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_AssetFileDeleteResult_GetFilepath")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFileDeleteResult_GetFilepath", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDeleteResult_GetFilepath_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_AssetFileDeleteResult_GetFilepath_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFileDeleteResult_GetFilepath_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDeleteResult_GetSuccess(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_AssetFileDeleteResult_GetSuccess")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFileDeleteResult_GetSuccess", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadCancelResult_GetAssetFileId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_AssetFileDownloadCancelResult_GetAssetFileId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFileDownloadCancelResult_GetAssetFileId", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadCancelResult_GetAssetId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_AssetFileDownloadCancelResult_GetAssetId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFileDownloadCancelResult_GetAssetId", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadCancelResult_GetFilepath(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_AssetFileDownloadCancelResult_GetFilepath")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFileDownloadCancelResult_GetFilepath", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadCancelResult_GetFilepath_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_AssetFileDownloadCancelResult_GetFilepath_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFileDownloadCancelResult_GetFilepath_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadCancelResult_GetSuccess(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_AssetFileDownloadCancelResult_GetSuccess")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFileDownloadCancelResult_GetSuccess", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadResult_GetAssetId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_AssetFileDownloadResult_GetAssetId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFileDownloadResult_GetAssetId", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadResult_GetFilepath(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_AssetFileDownloadResult_GetFilepath")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFileDownloadResult_GetFilepath", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadResult_GetFilepath_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_AssetFileDownloadResult_GetFilepath_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFileDownloadResult_GetFilepath_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadUpdate_GetAssetFileId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_AssetFileDownloadUpdate_GetAssetFileId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFileDownloadUpdate_GetAssetFileId", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadUpdate_GetAssetId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_AssetFileDownloadUpdate_GetAssetId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFileDownloadUpdate_GetAssetId", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadUpdate_GetBytesTotal(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u32,
                1usize,
            >("ovr_AssetFileDownloadUpdate_GetBytesTotal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFileDownloadUpdate_GetBytesTotal", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadUpdate_GetBytesTotalLong(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_AssetFileDownloadUpdate_GetBytesTotalLong")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFileDownloadUpdate_GetBytesTotalLong", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadUpdate_GetBytesTransferred(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                i32,
                1usize,
            >("ovr_AssetFileDownloadUpdate_GetBytesTransferred")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFileDownloadUpdate_GetBytesTransferred", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadUpdate_GetBytesTransferredLong(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                i64,
                1usize,
            >("ovr_AssetFileDownloadUpdate_GetBytesTransferredLong")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFileDownloadUpdate_GetBytesTransferredLong", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadUpdate_GetCompleted(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_AssetFileDownloadUpdate_GetCompleted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFileDownloadUpdate_GetCompleted", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_Delete(assetFileID: u64) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("ovr_AssetFile_Delete")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFile_Delete", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (assetFileID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_DeleteById(
        assetFileID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("ovr_AssetFile_DeleteById")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFile_DeleteById", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (assetFileID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_DeleteByName(
        assetFileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ovr_AssetFile_DeleteByName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFile_DeleteByName", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (assetFileName)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_DeleteByName_Native(
        assetFileName: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_AssetFile_DeleteByName_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFile_DeleteByName_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (assetFileName)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_Download(
        assetFileID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("ovr_AssetFile_Download")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFile_Download", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (assetFileID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_DownloadById(
        assetFileID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("ovr_AssetFile_DownloadById")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFile_DownloadById", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (assetFileID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_DownloadByName(
        assetFileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ovr_AssetFile_DownloadByName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFile_DownloadByName", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (assetFileName)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_DownloadByName_Native(
        assetFileName: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_AssetFile_DownloadByName_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFile_DownloadByName_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (assetFileName)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_DownloadCancel(
        assetFileID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("ovr_AssetFile_DownloadCancel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFile_DownloadCancel", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (assetFileID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_DownloadCancelById(
        assetFileID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("ovr_AssetFile_DownloadCancelById")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFile_DownloadCancelById", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (assetFileID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_DownloadCancelByName(
        assetFileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ovr_AssetFile_DownloadCancelByName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFile_DownloadCancelByName", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (assetFileName)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_DownloadCancelByName_Native(
        assetFileName: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_AssetFile_DownloadCancelByName_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFile_DownloadCancelByName_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (assetFileName)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_GetList() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_AssetFile_GetList")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFile_GetList", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_Status(assetFileID: u64) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("ovr_AssetFile_Status")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFile_Status", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (assetFileID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_StatusById(
        assetFileID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("ovr_AssetFile_StatusById")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFile_StatusById", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (assetFileID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_StatusByName(
        assetFileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ovr_AssetFile_StatusByName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFile_StatusByName", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (assetFileName)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_StatusByName_Native(
        assetFileName: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_AssetFile_StatusByName_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AssetFile_StatusByName_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (assetFileName)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AvatarEditorOptions_Create() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::System::IntPtr,
                0usize,
            >("ovr_AvatarEditorOptions_Create")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AvatarEditorOptions_Create", 0usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AvatarEditorOptions_Destroy(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_AvatarEditorOptions_Destroy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AvatarEditorOptions_Destroy", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AvatarEditorOptions_SetSourceOverride(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_AvatarEditorOptions_SetSourceOverride")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AvatarEditorOptions_SetSourceOverride", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AvatarEditorOptions_SetSourceOverride_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_AvatarEditorOptions_SetSourceOverride_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AvatarEditorOptions_SetSourceOverride_Native", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AvatarEditorResult_GetRequestSent(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_AvatarEditorResult_GetRequestSent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_AvatarEditorResult_GetRequestSent", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Avatar_LaunchAvatarEditor(
        options: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_Avatar_LaunchAvatarEditor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Avatar_LaunchAvatarEditor", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (options)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Avatar_UpdateMetaData(
        avatarMetaData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        imageFilePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                u64,
                2usize,
            >("ovr_Avatar_UpdateMetaData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Avatar_UpdateMetaData", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (avatarMetaData, imageFilePath))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Avatar_UpdateMetaData_Native(
        avatarMetaData: crate::System::IntPtr,
        imageFilePath: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                u64,
                2usize,
            >("ovr_Avatar_UpdateMetaData_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Avatar_UpdateMetaData_Native", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (avatarMetaData, imageFilePath))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_BlockedUserArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::UIntPtr),
                crate::System::IntPtr,
                2usize,
            >("ovr_BlockedUserArray_GetElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_BlockedUserArray_GetElement", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_BlockedUserArray_GetNextUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_BlockedUserArray_GetNextUrl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_BlockedUserArray_GetNextUrl", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_BlockedUserArray_GetNextUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_BlockedUserArray_GetNextUrl_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_BlockedUserArray_GetNextUrl_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_BlockedUserArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::UIntPtr,
                1usize,
            >("ovr_BlockedUserArray_GetSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_BlockedUserArray_GetSize", 1usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_BlockedUserArray_HasNextPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_BlockedUserArray_HasNextPage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_BlockedUserArray_HasNextPage", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_BlockedUser_GetId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_BlockedUser_GetId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_BlockedUser_GetId", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::UIntPtr),
                crate::System::IntPtr,
                2usize,
            >("ovr_ChallengeArray_GetElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeArray_GetElement", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeArray_GetNextUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_ChallengeArray_GetNextUrl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeArray_GetNextUrl", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeArray_GetNextUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_ChallengeArray_GetNextUrl_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeArray_GetNextUrl_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeArray_GetPreviousUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_ChallengeArray_GetPreviousUrl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeArray_GetPreviousUrl", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeArray_GetPreviousUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_ChallengeArray_GetPreviousUrl_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeArray_GetPreviousUrl_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::UIntPtr,
                1usize,
            >("ovr_ChallengeArray_GetSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeArray_GetSize", 1usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeArray_GetTotalCount(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_ChallengeArray_GetTotalCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeArray_GetTotalCount", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeArray_HasNextPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_ChallengeArray_HasNextPage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeArray_HasNextPage", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeArray_HasPreviousPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_ChallengeArray_HasPreviousPage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeArray_HasPreviousPage", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntryArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::UIntPtr),
                crate::System::IntPtr,
                2usize,
            >("ovr_ChallengeEntryArray_GetElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeEntryArray_GetElement", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntryArray_GetNextUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_ChallengeEntryArray_GetNextUrl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeEntryArray_GetNextUrl", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntryArray_GetNextUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_ChallengeEntryArray_GetNextUrl_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeEntryArray_GetNextUrl_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntryArray_GetPreviousUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_ChallengeEntryArray_GetPreviousUrl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeEntryArray_GetPreviousUrl", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntryArray_GetPreviousUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_ChallengeEntryArray_GetPreviousUrl_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeEntryArray_GetPreviousUrl_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntryArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::UIntPtr,
                1usize,
            >("ovr_ChallengeEntryArray_GetSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeEntryArray_GetSize", 1usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntryArray_GetTotalCount(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_ChallengeEntryArray_GetTotalCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeEntryArray_GetTotalCount", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntryArray_HasNextPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_ChallengeEntryArray_HasNextPage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeEntryArray_HasNextPage", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntryArray_HasPreviousPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_ChallengeEntryArray_HasPreviousPage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeEntryArray_HasPreviousPage", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntry_GetDisplayScore(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_ChallengeEntry_GetDisplayScore")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeEntry_GetDisplayScore", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntry_GetDisplayScore_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_ChallengeEntry_GetDisplayScore_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeEntry_GetDisplayScore_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntry_GetExtraData(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                1usize,
            >("ovr_ChallengeEntry_GetExtraData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeEntry_GetExtraData", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntry_GetExtraDataLength(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u32,
                1usize,
            >("ovr_ChallengeEntry_GetExtraDataLength")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeEntry_GetExtraDataLength", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntry_GetExtraData_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_ChallengeEntry_GetExtraData_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeEntry_GetExtraData_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntry_GetID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_ChallengeEntry_GetID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeEntry_GetID", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntry_GetRank(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                i32,
                1usize,
            >("ovr_ChallengeEntry_GetRank")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeEntry_GetRank", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntry_GetScore(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                i64,
                1usize,
            >("ovr_ChallengeEntry_GetScore")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeEntry_GetScore", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntry_GetTimestamp(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::DateTime,
                1usize,
            >("ovr_ChallengeEntry_GetTimestamp")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeEntry_GetTimestamp", 1usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntry_GetTimestamp_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_ChallengeEntry_GetTimestamp_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeEntry_GetTimestamp_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntry_GetUser(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_ChallengeEntry_GetUser")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeEntry_GetUser", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_Create() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::System::IntPtr,
                0usize,
            >("ovr_ChallengeOptions_Create")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeOptions_Create", 0usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_Destroy(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_ChallengeOptions_Destroy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeOptions_Destroy", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetDescription(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_ChallengeOptions_SetDescription")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeOptions_SetDescription", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetDescription_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_ChallengeOptions_SetDescription_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeOptions_SetDescription_Native", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetEndDate(
        handle: crate::System::IntPtr,
        value: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::DateTime),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_ChallengeOptions_SetEndDate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeOptions_SetEndDate", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetEndDate_Native(
        handle: crate::System::IntPtr,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, u64),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_ChallengeOptions_SetEndDate_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeOptions_SetEndDate_Native", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetIncludeActiveChallenges(
        handle: crate::System::IntPtr,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_ChallengeOptions_SetIncludeActiveChallenges")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeOptions_SetIncludeActiveChallenges", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetIncludeFutureChallenges(
        handle: crate::System::IntPtr,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_ChallengeOptions_SetIncludeFutureChallenges")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeOptions_SetIncludeFutureChallenges", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetIncludePastChallenges(
        handle: crate::System::IntPtr,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_ChallengeOptions_SetIncludePastChallenges")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeOptions_SetIncludePastChallenges", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetLeaderboardName(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_ChallengeOptions_SetLeaderboardName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeOptions_SetLeaderboardName", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetLeaderboardName_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_ChallengeOptions_SetLeaderboardName_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeOptions_SetLeaderboardName_Native", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetStartDate(
        handle: crate::System::IntPtr,
        value: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::DateTime),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_ChallengeOptions_SetStartDate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeOptions_SetStartDate", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetStartDate_Native(
        handle: crate::System::IntPtr,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, u64),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_ChallengeOptions_SetStartDate_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeOptions_SetStartDate_Native", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetTitle(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_ChallengeOptions_SetTitle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeOptions_SetTitle", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetTitle_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_ChallengeOptions_SetTitle_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeOptions_SetTitle_Native", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetViewerFilter(
        handle: crate::System::IntPtr,
        value: crate::Oculus::Platform::ChallengeViewerFilter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::Oculus::Platform::ChallengeViewerFilter),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_ChallengeOptions_SetViewerFilter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeOptions_SetViewerFilter", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetVisibility(
        handle: crate::System::IntPtr,
        value: crate::Oculus::Platform::ChallengeVisibility,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::Oculus::Platform::ChallengeVisibility),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_ChallengeOptions_SetVisibility")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ChallengeOptions_SetVisibility", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenge_GetCreationType(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::ChallengeCreationType> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::Oculus::Platform::ChallengeCreationType,
                1usize,
            >("ovr_Challenge_GetCreationType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenge_GetCreationType", 1usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::ChallengeCreationType = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenge_GetDescription(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_Challenge_GetDescription")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenge_GetDescription", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenge_GetDescription_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Challenge_GetDescription_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenge_GetDescription_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenge_GetEndDate(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::DateTime,
                1usize,
            >("ovr_Challenge_GetEndDate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenge_GetEndDate", 1usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenge_GetEndDate_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_Challenge_GetEndDate_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenge_GetEndDate_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenge_GetID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_Challenge_GetID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenge_GetID", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenge_GetInvitedUsers(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Challenge_GetInvitedUsers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenge_GetInvitedUsers", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenge_GetLeaderboard(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Challenge_GetLeaderboard")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenge_GetLeaderboard", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenge_GetParticipants(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Challenge_GetParticipants")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenge_GetParticipants", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenge_GetStartDate(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::DateTime,
                1usize,
            >("ovr_Challenge_GetStartDate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenge_GetStartDate", 1usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenge_GetStartDate_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_Challenge_GetStartDate_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenge_GetStartDate_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenge_GetTitle(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_Challenge_GetTitle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenge_GetTitle", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenge_GetTitle_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Challenge_GetTitle_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenge_GetTitle_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenge_GetVisibility(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::ChallengeVisibility> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::Oculus::Platform::ChallengeVisibility,
                1usize,
            >("ovr_Challenge_GetVisibility")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenge_GetVisibility", 1usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::ChallengeVisibility = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_Create(
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        challengeOptions: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::System::IntPtr,
                ),
                u64,
                2usize,
            >("ovr_Challenges_Create")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenges_Create", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (leaderboardName, challengeOptions))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_Create_Native(
        leaderboardName: crate::System::IntPtr,
        challengeOptions: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                u64,
                2usize,
            >("ovr_Challenges_Create_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenges_Create_Native", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (leaderboardName, challengeOptions))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_DeclineInvite(
        challengeID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("ovr_Challenges_DeclineInvite")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenges_DeclineInvite", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (challengeID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_Delete(
        challengeID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("ovr_Challenges_Delete")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenges_Delete", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (challengeID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_Get(challengeID: u64) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("ovr_Challenges_Get")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenges_Get", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (challengeID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_GetEntries(
        challengeID: u64,
        limit: i32,
        filter: crate::Oculus::Platform::LeaderboardFilterType,
        startAt: crate::Oculus::Platform::LeaderboardStartAt,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    u64,
                    i32,
                    crate::Oculus::Platform::LeaderboardFilterType,
                    crate::Oculus::Platform::LeaderboardStartAt,
                ),
                u64,
                4usize,
            >("ovr_Challenges_GetEntries")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenges_GetEntries", 4usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (challengeID, limit, filter, startAt))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_GetEntriesAfterRank(
        challengeID: u64,
        limit: i32,
        afterRank: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64, i32, u64),
                u64,
                3usize,
            >("ovr_Challenges_GetEntriesAfterRank")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenges_GetEntriesAfterRank", 3usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (challengeID, limit, afterRank))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_GetEntriesByIds(
        challengeID: u64,
        limit: i32,
        startAt: crate::Oculus::Platform::LeaderboardStartAt,
        userIDs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        userIDLength: u32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    u64,
                    i32,
                    crate::Oculus::Platform::LeaderboardStartAt,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
                    u32,
                ),
                u64,
                5usize,
            >("ovr_Challenges_GetEntriesByIds")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenges_GetEntriesByIds", 5usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (challengeID, limit, startAt, userIDs, userIDLength),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_GetList(
        challengeOptions: crate::System::IntPtr,
        limit: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, i32),
                u64,
                2usize,
            >("ovr_Challenges_GetList")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenges_GetList", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (challengeOptions, limit))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_GetNextChallenges(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_Challenges_GetNextChallenges")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenges_GetNextChallenges", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (handle)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_GetNextEntries(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_Challenges_GetNextEntries")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenges_GetNextEntries", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (handle)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_GetPreviousChallenges(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_Challenges_GetPreviousChallenges")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenges_GetPreviousChallenges", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (handle)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_GetPreviousEntries(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_Challenges_GetPreviousEntries")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenges_GetPreviousEntries", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (handle)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_Join(challengeID: u64) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("ovr_Challenges_Join")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenges_Join", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (challengeID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_Leave(challengeID: u64) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("ovr_Challenges_Leave")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenges_Leave", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (challengeID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_UpdateInfo(
        challengeID: u64,
        challengeOptions: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64, crate::System::IntPtr),
                u64,
                2usize,
            >("ovr_Challenges_UpdateInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Challenges_UpdateInfo", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (challengeID, challengeOptions))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Colocation_GetCurrentMapUuid() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_Colocation_GetCurrentMapUuid")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Colocation_GetCurrentMapUuid", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Colocation_RequestMap(
        uuid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ovr_Colocation_RequestMap")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Colocation_RequestMap", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (uuid)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Colocation_RequestMap_Native(
        uuid: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_Colocation_RequestMap_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Colocation_RequestMap_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (uuid)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Colocation_ShareMap(
        uuid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ovr_Colocation_ShareMap")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Colocation_ShareMap", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (uuid)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Colocation_ShareMap_Native(
        uuid: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_Colocation_ShareMap_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Colocation_ShareMap_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (uuid)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_CrashApplication() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ovr_CrashApplication")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_CrashApplication", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DataStore_Contains(
        obj: crate::System::IntPtr,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                u32,
                2usize,
            >("ovr_DataStore_Contains")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_DataStore_Contains", 2usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (obj, key)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DataStore_Contains_Native(
        obj: crate::System::IntPtr,
        key: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                u32,
                2usize,
            >("ovr_DataStore_Contains_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_DataStore_Contains_Native", 2usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (obj, key)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DataStore_GetKey(
        obj: crate::System::IntPtr,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, i32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("ovr_DataStore_GetKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_DataStore_GetKey", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj, index)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DataStore_GetKey_Native(
        obj: crate::System::IntPtr,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, i32),
                crate::System::IntPtr,
                2usize,
            >("ovr_DataStore_GetKey_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_DataStore_GetKey_Native", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DataStore_GetNumKeys(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::UIntPtr,
                1usize,
            >("ovr_DataStore_GetNumKeys")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_DataStore_GetNumKeys", 1usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DataStore_GetValue(
        obj: crate::System::IntPtr,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("ovr_DataStore_GetValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_DataStore_GetValue", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj, key)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DataStore_GetValue_Native(
        obj: crate::System::IntPtr,
        key: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                crate::System::IntPtr,
                2usize,
            >("ovr_DataStore_GetValue_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_DataStore_GetValue_Native", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj, key))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DestinationArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::UIntPtr),
                crate::System::IntPtr,
                2usize,
            >("ovr_DestinationArray_GetElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_DestinationArray_GetElement", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DestinationArray_GetNextUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_DestinationArray_GetNextUrl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_DestinationArray_GetNextUrl", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DestinationArray_GetNextUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_DestinationArray_GetNextUrl_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_DestinationArray_GetNextUrl_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DestinationArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::UIntPtr,
                1usize,
            >("ovr_DestinationArray_GetSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_DestinationArray_GetSize", 1usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DestinationArray_HasNextPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_DestinationArray_HasNextPage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_DestinationArray_HasNextPage", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Destination_GetApiName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_Destination_GetApiName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Destination_GetApiName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Destination_GetApiName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Destination_GetApiName_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Destination_GetApiName_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Destination_GetDeeplinkMessage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_Destination_GetDeeplinkMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Destination_GetDeeplinkMessage", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Destination_GetDeeplinkMessage_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Destination_GetDeeplinkMessage_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Destination_GetDeeplinkMessage_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Destination_GetDisplayName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_Destination_GetDisplayName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Destination_GetDisplayName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Destination_GetDisplayName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Destination_GetDisplayName_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Destination_GetDisplayName_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DeviceApplicationIntegrity_GetAttestationToken(
        challenge_nonce: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ovr_DeviceApplicationIntegrity_GetAttestationToken")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_DeviceApplicationIntegrity_GetAttestationToken", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (challenge_nonce)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DeviceApplicationIntegrity_GetAttestationToken_Native(
        challenge_nonce: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_DeviceApplicationIntegrity_GetAttestationToken_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_DeviceApplicationIntegrity_GetAttestationToken_Native",
                    1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (challenge_nonce)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DeviceApplicationIntegrity_GetIntegrityToken(
        challenge_nonce: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ovr_DeviceApplicationIntegrity_GetIntegrityToken")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_DeviceApplicationIntegrity_GetIntegrityToken", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (challenge_nonce)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DeviceApplicationIntegrity_GetIntegrityToken_Native(
        challenge_nonce: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_DeviceApplicationIntegrity_GetIntegrityToken_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_DeviceApplicationIntegrity_GetIntegrityToken_Native",
                    1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (challenge_nonce)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Entitlement_GetIsViewerEntitled() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_Entitlement_GetIsViewerEntitled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Entitlement_GetIsViewerEntitled", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Error_GetCode(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                i32,
                1usize,
            >("ovr_Error_GetCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Error_GetCode", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Error_GetDisplayableMessage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_Error_GetDisplayableMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Error_GetDisplayableMessage", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Error_GetDisplayableMessage_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Error_GetDisplayableMessage_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Error_GetDisplayableMessage_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Error_GetHttpCode(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                i32,
                1usize,
            >("ovr_Error_GetHttpCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Error_GetHttpCode", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Error_GetMessage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_Error_GetMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Error_GetMessage", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Error_GetMessage_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Error_GetMessage_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Error_GetMessage_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_FreeMessage(
        message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_FreeMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_FreeMessage", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (message))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GetLoggedInUserLocale() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("ovr_GetLoggedInUserLocale")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GetLoggedInUserLocale", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GetLoggedInUserLocale_Native() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::System::IntPtr,
                0usize,
            >("ovr_GetLoggedInUserLocale_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GetLoggedInUserLocale_Native", 0usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GraphAPI_Get(
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ovr_GraphAPI_Get")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GraphAPI_Get", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (url)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GraphAPI_Get_Native(
        url: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_GraphAPI_Get_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GraphAPI_Get_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (url)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GraphAPI_Post(
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ovr_GraphAPI_Post")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GraphAPI_Post", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (url)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GraphAPI_Post_Native(
        url: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_GraphAPI_Post_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GraphAPI_Post_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (url)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceJoinIntent_GetDeeplinkMessage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_GroupPresenceJoinIntent_GetDeeplinkMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresenceJoinIntent_GetDeeplinkMessage", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceJoinIntent_GetDeeplinkMessage_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_GroupPresenceJoinIntent_GetDeeplinkMessage_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresenceJoinIntent_GetDeeplinkMessage_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceJoinIntent_GetDestinationApiName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_GroupPresenceJoinIntent_GetDestinationApiName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresenceJoinIntent_GetDestinationApiName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceJoinIntent_GetDestinationApiName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_GroupPresenceJoinIntent_GetDestinationApiName_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresenceJoinIntent_GetDestinationApiName_Native",
                    1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceJoinIntent_GetLobbySessionId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_GroupPresenceJoinIntent_GetLobbySessionId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresenceJoinIntent_GetLobbySessionId", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceJoinIntent_GetLobbySessionId_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_GroupPresenceJoinIntent_GetLobbySessionId_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresenceJoinIntent_GetLobbySessionId_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceJoinIntent_GetMatchSessionId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_GroupPresenceJoinIntent_GetMatchSessionId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresenceJoinIntent_GetMatchSessionId", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceJoinIntent_GetMatchSessionId_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_GroupPresenceJoinIntent_GetMatchSessionId_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresenceJoinIntent_GetMatchSessionId_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceLeaveIntent_GetDestinationApiName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_GroupPresenceLeaveIntent_GetDestinationApiName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresenceLeaveIntent_GetDestinationApiName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceLeaveIntent_GetDestinationApiName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_GroupPresenceLeaveIntent_GetDestinationApiName_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresenceLeaveIntent_GetDestinationApiName_Native",
                    1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceLeaveIntent_GetLobbySessionId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_GroupPresenceLeaveIntent_GetLobbySessionId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresenceLeaveIntent_GetLobbySessionId", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceLeaveIntent_GetLobbySessionId_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_GroupPresenceLeaveIntent_GetLobbySessionId_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresenceLeaveIntent_GetLobbySessionId_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceLeaveIntent_GetMatchSessionId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_GroupPresenceLeaveIntent_GetMatchSessionId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresenceLeaveIntent_GetMatchSessionId", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceLeaveIntent_GetMatchSessionId_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_GroupPresenceLeaveIntent_GetMatchSessionId_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresenceLeaveIntent_GetMatchSessionId_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceOptions_Create() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::System::IntPtr,
                0usize,
            >("ovr_GroupPresenceOptions_Create")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresenceOptions_Create", 0usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceOptions_Destroy(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_GroupPresenceOptions_Destroy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresenceOptions_Destroy", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceOptions_SetDeeplinkMessageOverride(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_GroupPresenceOptions_SetDeeplinkMessageOverride")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresenceOptions_SetDeeplinkMessageOverride", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceOptions_SetDeeplinkMessageOverride_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_GroupPresenceOptions_SetDeeplinkMessageOverride_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresenceOptions_SetDeeplinkMessageOverride_Native",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceOptions_SetDestinationApiName(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_GroupPresenceOptions_SetDestinationApiName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresenceOptions_SetDestinationApiName", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceOptions_SetDestinationApiName_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_GroupPresenceOptions_SetDestinationApiName_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresenceOptions_SetDestinationApiName_Native", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceOptions_SetIsJoinable(
        handle: crate::System::IntPtr,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_GroupPresenceOptions_SetIsJoinable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresenceOptions_SetIsJoinable", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceOptions_SetLobbySessionId(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_GroupPresenceOptions_SetLobbySessionId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresenceOptions_SetLobbySessionId", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceOptions_SetLobbySessionId_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_GroupPresenceOptions_SetLobbySessionId_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresenceOptions_SetLobbySessionId_Native", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceOptions_SetMatchSessionId(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_GroupPresenceOptions_SetMatchSessionId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresenceOptions_SetMatchSessionId", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceOptions_SetMatchSessionId_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_GroupPresenceOptions_SetMatchSessionId_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresenceOptions_SetMatchSessionId_Native", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_Clear() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_GroupPresence_Clear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresence_Clear", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_GetInvitableUsers(
        options: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_GroupPresence_GetInvitableUsers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresence_GetInvitableUsers", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (options)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_GetSentInvites() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_GroupPresence_GetSentInvites")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresence_GetSentInvites", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_LaunchInvitePanel(
        options: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_GroupPresence_LaunchInvitePanel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresence_LaunchInvitePanel", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (options)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_LaunchMultiplayerErrorDialog(
        options: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_GroupPresence_LaunchMultiplayerErrorDialog")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresence_LaunchMultiplayerErrorDialog", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (options)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_LaunchRejoinDialog(
        lobby_session_id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        match_session_id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        destination_api_name: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                u64,
                3usize,
            >("ovr_GroupPresence_LaunchRejoinDialog")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresence_LaunchRejoinDialog", 3usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (lobby_session_id, match_session_id, destination_api_name),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_LaunchRejoinDialog_Native(
        lobby_session_id: crate::System::IntPtr,
        match_session_id: crate::System::IntPtr,
        destination_api_name: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr, crate::System::IntPtr),
                u64,
                3usize,
            >("ovr_GroupPresence_LaunchRejoinDialog_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresence_LaunchRejoinDialog_Native", 3usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (lobby_session_id, match_session_id, destination_api_name),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_LaunchRosterPanel(
        options: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_GroupPresence_LaunchRosterPanel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresence_LaunchRosterPanel", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (options)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_SendInvites(
        userIDs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        userIDLength: u32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
                    u32,
                ),
                u64,
                2usize,
            >("ovr_GroupPresence_SendInvites")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresence_SendInvites", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (userIDs, userIDLength))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_Set(
        groupPresenceOptions: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_GroupPresence_Set")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresence_Set", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (groupPresenceOptions))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_SetDeeplinkMessageOverride(
        deeplink_message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ovr_GroupPresence_SetDeeplinkMessageOverride")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresence_SetDeeplinkMessageOverride", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (deeplink_message))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_SetDeeplinkMessageOverride_Native(
        deeplink_message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_GroupPresence_SetDeeplinkMessageOverride_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresence_SetDeeplinkMessageOverride_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (deeplink_message))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_SetDestination(
        api_name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ovr_GroupPresence_SetDestination")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresence_SetDestination", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (api_name)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_SetDestination_Native(
        api_name: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_GroupPresence_SetDestination_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresence_SetDestination_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (api_name)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_SetIsJoinable(
        is_joinable: bool,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(bool), u64, 1usize>("ovr_GroupPresence_SetIsJoinable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresence_SetIsJoinable", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (is_joinable)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_SetLobbySession(
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ovr_GroupPresence_SetLobbySession")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresence_SetLobbySession", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (id)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_SetLobbySession_Native(
        id: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_GroupPresence_SetLobbySession_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresence_SetLobbySession_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (id)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_SetMatchSession(
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ovr_GroupPresence_SetMatchSession")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresence_SetMatchSession", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (id)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_SetMatchSession_Native(
        id: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_GroupPresence_SetMatchSession_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_GroupPresence_SetMatchSession_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (id)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HTTP_Get(
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ovr_HTTP_Get")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_HTTP_Get", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (url)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HTTP_GetToFile(
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        diskFile: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                u64,
                2usize,
            >("ovr_HTTP_GetToFile")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_HTTP_GetToFile", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (url, diskFile)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HTTP_GetToFile_Native(
        url: crate::System::IntPtr,
        diskFile: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                u64,
                2usize,
            >("ovr_HTTP_GetToFile_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_HTTP_GetToFile_Native", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (url, diskFile)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HTTP_GetWithMessageType(
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        messageType: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, i32),
                u64,
                2usize,
            >("ovr_HTTP_GetWithMessageType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_HTTP_GetWithMessageType", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (url, messageType))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HTTP_Get_Native(
        url: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_HTTP_Get_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_HTTP_Get_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (url)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HTTP_MultiPartPost(
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        filepath_param_name: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        filepath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        access_token: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        post_params: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::Oculus::Platform::CAPI_ovrKeyValuePair,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::Oculus::Platform::CAPI_ovrKeyValuePair,
                        >,
                    >,
                ),
                u64,
                5usize,
            >("ovr_HTTP_MultiPartPost")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_HTTP_MultiPartPost", 5usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (url, filepath_param_name, filepath, access_token, post_params),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HTTP_MultiPartPost_Native(
        url: crate::System::IntPtr,
        filepath_param_name: crate::System::IntPtr,
        filepath: crate::System::IntPtr,
        access_token: crate::System::IntPtr,
        post_params: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::Oculus::Platform::CAPI_ovrKeyValuePair,
            >,
        >,
        numItems: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::Oculus::Platform::CAPI_ovrKeyValuePair,
                        >,
                    >,
                    crate::System::UIntPtr,
                ),
                u64,
                6usize,
            >("ovr_HTTP_MultiPartPost_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_HTTP_MultiPartPost_Native", 6usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        url,
                        filepath_param_name,
                        filepath,
                        access_token,
                        post_params,
                        numItems,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HTTP_Post(
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ovr_HTTP_Post")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_HTTP_Post", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (url)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HTTP_Post_Native(
        url: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_HTTP_Post_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_HTTP_Post_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (url)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HTTP_StartTransfer(
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        headers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::Oculus::Platform::CAPI_ovrKeyValuePair,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::Oculus::Platform::CAPI_ovrKeyValuePair,
                        >,
                    >,
                ),
                u64,
                2usize,
            >("ovr_HTTP_StartTransfer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_HTTP_StartTransfer", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (url, headers)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HTTP_StartTransfer_Native(
        url: crate::System::IntPtr,
        headers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::Oculus::Platform::CAPI_ovrKeyValuePair,
            >,
        >,
        numItems: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::Oculus::Platform::CAPI_ovrKeyValuePair,
                        >,
                    >,
                    crate::System::UIntPtr,
                ),
                u64,
                3usize,
            >("ovr_HTTP_StartTransfer_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_HTTP_StartTransfer_Native", 3usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (url, headers, numItems))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HTTP_Write(
        transferId: u64,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        length: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    u64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    crate::System::UIntPtr,
                ),
                bool,
                3usize,
            >("ovr_HTTP_Write")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_HTTP_Write", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (transferId, bytes, length))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HTTP_WriteEOM(
        transferId: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_HTTP_WriteEOM")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_HTTP_WriteEOM", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (transferId))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HttpTransferUpdate_GetBytes(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_HttpTransferUpdate_GetBytes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_HttpTransferUpdate_GetBytes", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HttpTransferUpdate_GetID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_HttpTransferUpdate_GetID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_HttpTransferUpdate_GetID", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HttpTransferUpdate_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::UIntPtr,
                1usize,
            >("ovr_HttpTransferUpdate_GetSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_HttpTransferUpdate_GetSize", 1usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HttpTransferUpdate_IsCompleted(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_HttpTransferUpdate_IsCompleted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_HttpTransferUpdate_IsCompleted", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_IAP_ConsumePurchase(
        sku: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ovr_IAP_ConsumePurchase")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_IAP_ConsumePurchase", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (sku)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_IAP_ConsumePurchase_Native(
        sku: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_IAP_ConsumePurchase_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_IAP_ConsumePurchase_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (sku)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_IAP_GetProductsBySKU(
        skus: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        >,
                    >,
                    i32,
                ),
                u64,
                2usize,
            >("ovr_IAP_GetProductsBySKU")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_IAP_GetProductsBySKU", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (skus, count)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_IAP_GetViewerPurchases() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_IAP_GetViewerPurchases")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_IAP_GetViewerPurchases", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_IAP_GetViewerPurchasesDurableCache() -> quest_hook::libil2cpp::Result<
        u64,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                u64,
                0usize,
            >("ovr_IAP_GetViewerPurchasesDurableCache")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_IAP_GetViewerPurchasesDurableCache", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_IAP_LaunchCheckoutFlow(
        sku: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ovr_IAP_LaunchCheckoutFlow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_IAP_LaunchCheckoutFlow", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (sku)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_IAP_LaunchCheckoutFlow_Native(
        sku: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_IAP_LaunchCheckoutFlow_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_IAP_LaunchCheckoutFlow_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (sku)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InstalledApplicationArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::UIntPtr),
                crate::System::IntPtr,
                2usize,
            >("ovr_InstalledApplicationArray_GetElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_InstalledApplicationArray_GetElement", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InstalledApplicationArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::UIntPtr,
                1usize,
            >("ovr_InstalledApplicationArray_GetSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_InstalledApplicationArray_GetSize", 1usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InstalledApplication_GetApplicationId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_InstalledApplication_GetApplicationId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_InstalledApplication_GetApplicationId", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InstalledApplication_GetApplicationId_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_InstalledApplication_GetApplicationId_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_InstalledApplication_GetApplicationId_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InstalledApplication_GetPackageName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_InstalledApplication_GetPackageName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_InstalledApplication_GetPackageName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InstalledApplication_GetPackageName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_InstalledApplication_GetPackageName_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_InstalledApplication_GetPackageName_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InstalledApplication_GetStatus(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_InstalledApplication_GetStatus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_InstalledApplication_GetStatus", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InstalledApplication_GetStatus_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_InstalledApplication_GetStatus_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_InstalledApplication_GetStatus_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InstalledApplication_GetVersionCode(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                i32,
                1usize,
            >("ovr_InstalledApplication_GetVersionCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_InstalledApplication_GetVersionCode", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InstalledApplication_GetVersionName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_InstalledApplication_GetVersionName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_InstalledApplication_GetVersionName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InstalledApplication_GetVersionName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_InstalledApplication_GetVersionName_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_InstalledApplication_GetVersionName_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InviteOptions_AddSuggestedUser(
        handle: crate::System::IntPtr,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, u64),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_InviteOptions_AddSuggestedUser")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_InviteOptions_AddSuggestedUser", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InviteOptions_ClearSuggestedUsers(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_InviteOptions_ClearSuggestedUsers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_InviteOptions_ClearSuggestedUsers", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InviteOptions_Create() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::System::IntPtr,
                0usize,
            >("ovr_InviteOptions_Create")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_InviteOptions_Create", 0usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InviteOptions_Destroy(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_InviteOptions_Destroy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_InviteOptions_Destroy", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InvitePanelResultInfo_GetInvitesSent(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_InvitePanelResultInfo_GetInvitesSent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_InvitePanelResultInfo_GetInvitesSent", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LanguagePackInfo_GetEnglishName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_LanguagePackInfo_GetEnglishName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LanguagePackInfo_GetEnglishName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LanguagePackInfo_GetEnglishName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_LanguagePackInfo_GetEnglishName_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LanguagePackInfo_GetEnglishName_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LanguagePackInfo_GetNativeName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_LanguagePackInfo_GetNativeName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LanguagePackInfo_GetNativeName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LanguagePackInfo_GetNativeName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_LanguagePackInfo_GetNativeName_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LanguagePackInfo_GetNativeName_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LanguagePackInfo_GetTag(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_LanguagePackInfo_GetTag")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LanguagePackInfo_GetTag", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LanguagePackInfo_GetTag_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_LanguagePackInfo_GetTag_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LanguagePackInfo_GetTag_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LanguagePack_GetCurrent() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_LanguagePack_GetCurrent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LanguagePack_GetCurrent", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LanguagePack_SetCurrent(
        tag: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ovr_LanguagePack_SetCurrent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LanguagePack_SetCurrent", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (tag)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LanguagePack_SetCurrent_Native(
        tag: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_LanguagePack_SetCurrent_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LanguagePack_SetCurrent_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (tag)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchBlockFlowResult_GetDidBlock(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_LaunchBlockFlowResult_GetDidBlock")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LaunchBlockFlowResult_GetDidBlock", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchBlockFlowResult_GetDidCancel(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_LaunchBlockFlowResult_GetDidCancel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LaunchBlockFlowResult_GetDidCancel", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchDetails_GetDeeplinkMessage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_LaunchDetails_GetDeeplinkMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LaunchDetails_GetDeeplinkMessage", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchDetails_GetDeeplinkMessage_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_LaunchDetails_GetDeeplinkMessage_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LaunchDetails_GetDeeplinkMessage_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchDetails_GetDestinationApiName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_LaunchDetails_GetDestinationApiName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LaunchDetails_GetDestinationApiName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchDetails_GetDestinationApiName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_LaunchDetails_GetDestinationApiName_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LaunchDetails_GetDestinationApiName_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchDetails_GetLaunchSource(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_LaunchDetails_GetLaunchSource")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LaunchDetails_GetLaunchSource", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchDetails_GetLaunchSource_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_LaunchDetails_GetLaunchSource_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LaunchDetails_GetLaunchSource_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchDetails_GetLaunchType(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::LaunchType> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::Oculus::Platform::LaunchType,
                1usize,
            >("ovr_LaunchDetails_GetLaunchType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LaunchDetails_GetLaunchType", 1usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::LaunchType = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchDetails_GetTrackingID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_LaunchDetails_GetTrackingID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LaunchDetails_GetTrackingID", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchDetails_GetTrackingID_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_LaunchDetails_GetTrackingID_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LaunchDetails_GetTrackingID_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchDetails_GetUsers(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_LaunchDetails_GetUsers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LaunchDetails_GetUsers", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchFriendRequestFlowResult_GetDidCancel(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_LaunchFriendRequestFlowResult_GetDidCancel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LaunchFriendRequestFlowResult_GetDidCancel", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchFriendRequestFlowResult_GetDidSendRequest(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_LaunchFriendRequestFlowResult_GetDidSendRequest")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LaunchFriendRequestFlowResult_GetDidSendRequest", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchInvitePanelFlowResult_GetInvitedUsers(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_LaunchInvitePanelFlowResult_GetInvitedUsers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LaunchInvitePanelFlowResult_GetInvitedUsers", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchReportFlowResult_GetDidCancel(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_LaunchReportFlowResult_GetDidCancel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LaunchReportFlowResult_GetDidCancel", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchReportFlowResult_GetUserReportId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_LaunchReportFlowResult_GetUserReportId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LaunchReportFlowResult_GetUserReportId", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchUnblockFlowResult_GetDidCancel(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_LaunchUnblockFlowResult_GetDidCancel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LaunchUnblockFlowResult_GetDidCancel", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchUnblockFlowResult_GetDidUnblock(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_LaunchUnblockFlowResult_GetDidUnblock")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LaunchUnblockFlowResult_GetDidUnblock", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::UIntPtr),
                crate::System::IntPtr,
                2usize,
            >("ovr_LeaderboardArray_GetElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardArray_GetElement", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardArray_GetNextUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_LeaderboardArray_GetNextUrl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardArray_GetNextUrl", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardArray_GetNextUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_LeaderboardArray_GetNextUrl_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardArray_GetNextUrl_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::UIntPtr,
                1usize,
            >("ovr_LeaderboardArray_GetSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardArray_GetSize", 1usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardArray_HasNextPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_LeaderboardArray_HasNextPage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardArray_HasNextPage", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntryArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::UIntPtr),
                crate::System::IntPtr,
                2usize,
            >("ovr_LeaderboardEntryArray_GetElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardEntryArray_GetElement", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntryArray_GetNextUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_LeaderboardEntryArray_GetNextUrl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardEntryArray_GetNextUrl", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntryArray_GetNextUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_LeaderboardEntryArray_GetNextUrl_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardEntryArray_GetNextUrl_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntryArray_GetPreviousUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_LeaderboardEntryArray_GetPreviousUrl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardEntryArray_GetPreviousUrl", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntryArray_GetPreviousUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_LeaderboardEntryArray_GetPreviousUrl_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardEntryArray_GetPreviousUrl_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntryArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::UIntPtr,
                1usize,
            >("ovr_LeaderboardEntryArray_GetSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardEntryArray_GetSize", 1usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntryArray_GetTotalCount(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_LeaderboardEntryArray_GetTotalCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardEntryArray_GetTotalCount", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntryArray_HasNextPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_LeaderboardEntryArray_HasNextPage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardEntryArray_HasNextPage", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntryArray_HasPreviousPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_LeaderboardEntryArray_HasPreviousPage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardEntryArray_HasPreviousPage", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntry_GetDisplayScore(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_LeaderboardEntry_GetDisplayScore")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardEntry_GetDisplayScore", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntry_GetDisplayScore_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_LeaderboardEntry_GetDisplayScore_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardEntry_GetDisplayScore_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntry_GetExtraData(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                1usize,
            >("ovr_LeaderboardEntry_GetExtraData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardEntry_GetExtraData", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntry_GetExtraDataLength(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u32,
                1usize,
            >("ovr_LeaderboardEntry_GetExtraDataLength")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardEntry_GetExtraDataLength", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntry_GetExtraData_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_LeaderboardEntry_GetExtraData_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardEntry_GetExtraData_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntry_GetID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_LeaderboardEntry_GetID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardEntry_GetID", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntry_GetRank(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                i32,
                1usize,
            >("ovr_LeaderboardEntry_GetRank")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardEntry_GetRank", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntry_GetScore(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                i64,
                1usize,
            >("ovr_LeaderboardEntry_GetScore")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardEntry_GetScore", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntry_GetSupplementaryMetric(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_LeaderboardEntry_GetSupplementaryMetric")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardEntry_GetSupplementaryMetric", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntry_GetTimestamp(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::DateTime,
                1usize,
            >("ovr_LeaderboardEntry_GetTimestamp")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardEntry_GetTimestamp", 1usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntry_GetTimestamp_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_LeaderboardEntry_GetTimestamp_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardEntry_GetTimestamp_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntry_GetUser(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_LeaderboardEntry_GetUser")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardEntry_GetUser", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardUpdateStatus_GetDidUpdate(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_LeaderboardUpdateStatus_GetDidUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardUpdateStatus_GetDidUpdate", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardUpdateStatus_GetUpdatedChallengeId(
        obj: crate::System::IntPtr,
        index: u32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, u32),
                u64,
                2usize,
            >("ovr_LeaderboardUpdateStatus_GetUpdatedChallengeId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardUpdateStatus_GetUpdatedChallengeId", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj, index)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardUpdateStatus_GetUpdatedChallengeIdsSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u32,
                1usize,
            >("ovr_LeaderboardUpdateStatus_GetUpdatedChallengeIdsSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LeaderboardUpdateStatus_GetUpdatedChallengeIdsSize",
                    1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_Get(
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ovr_Leaderboard_Get")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Leaderboard_Get", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (leaderboardName)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_GetApiName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_Leaderboard_GetApiName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Leaderboard_GetApiName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_GetApiName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Leaderboard_GetApiName_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Leaderboard_GetApiName_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_GetDestination(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Leaderboard_GetDestination")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Leaderboard_GetDestination", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_GetEntries(
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        limit: i32,
        filter: crate::Oculus::Platform::LeaderboardFilterType,
        startAt: crate::Oculus::Platform::LeaderboardStartAt,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i32,
                    crate::Oculus::Platform::LeaderboardFilterType,
                    crate::Oculus::Platform::LeaderboardStartAt,
                ),
                u64,
                4usize,
            >("ovr_Leaderboard_GetEntries")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Leaderboard_GetEntries", 4usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (leaderboardName, limit, filter, startAt))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_GetEntriesAfterRank(
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        limit: i32,
        afterRank: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i32,
                    u64,
                ),
                u64,
                3usize,
            >("ovr_Leaderboard_GetEntriesAfterRank")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Leaderboard_GetEntriesAfterRank", 3usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (leaderboardName, limit, afterRank))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_GetEntriesAfterRank_Native(
        leaderboardName: crate::System::IntPtr,
        limit: i32,
        afterRank: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, i32, u64),
                u64,
                3usize,
            >("ovr_Leaderboard_GetEntriesAfterRank_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Leaderboard_GetEntriesAfterRank_Native", 3usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (leaderboardName, limit, afterRank))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_GetEntriesByIds(
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        limit: i32,
        startAt: crate::Oculus::Platform::LeaderboardStartAt,
        userIDs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        userIDLength: u32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i32,
                    crate::Oculus::Platform::LeaderboardStartAt,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
                    u32,
                ),
                u64,
                5usize,
            >("ovr_Leaderboard_GetEntriesByIds")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Leaderboard_GetEntriesByIds", 5usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (leaderboardName, limit, startAt, userIDs, userIDLength),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_GetEntriesByIds_Native(
        leaderboardName: crate::System::IntPtr,
        limit: i32,
        startAt: crate::Oculus::Platform::LeaderboardStartAt,
        userIDs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        userIDLength: u32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    i32,
                    crate::Oculus::Platform::LeaderboardStartAt,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
                    u32,
                ),
                u64,
                5usize,
            >("ovr_Leaderboard_GetEntriesByIds_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Leaderboard_GetEntriesByIds_Native", 5usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (leaderboardName, limit, startAt, userIDs, userIDLength),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_GetEntries_Native(
        leaderboardName: crate::System::IntPtr,
        limit: i32,
        filter: crate::Oculus::Platform::LeaderboardFilterType,
        startAt: crate::Oculus::Platform::LeaderboardStartAt,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    i32,
                    crate::Oculus::Platform::LeaderboardFilterType,
                    crate::Oculus::Platform::LeaderboardStartAt,
                ),
                u64,
                4usize,
            >("ovr_Leaderboard_GetEntries_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Leaderboard_GetEntries_Native", 4usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (leaderboardName, limit, filter, startAt))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_GetID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_Leaderboard_GetID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Leaderboard_GetID", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_GetNextEntries(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_Leaderboard_GetNextEntries")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Leaderboard_GetNextEntries", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (handle)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_GetPreviousEntries(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_Leaderboard_GetPreviousEntries")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Leaderboard_GetPreviousEntries", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (handle)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_Get_Native(
        leaderboardName: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_Leaderboard_Get_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Leaderboard_Get_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (leaderboardName)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_WriteEntry(
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        score: i64,
        extraData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        extraDataLength: u32,
        forceUpdate: bool,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    u32,
                    bool,
                ),
                u64,
                5usize,
            >("ovr_Leaderboard_WriteEntry")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Leaderboard_WriteEntry", 5usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (leaderboardName, score, extraData, extraDataLength, forceUpdate),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_WriteEntryWithSupplementaryMetric(
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        score: i64,
        supplementaryMetric: i64,
        extraData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        extraDataLength: u32,
        forceUpdate: bool,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i64,
                    i64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    u32,
                    bool,
                ),
                u64,
                6usize,
            >("ovr_Leaderboard_WriteEntryWithSupplementaryMetric")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Leaderboard_WriteEntryWithSupplementaryMetric", 6usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        leaderboardName,
                        score,
                        supplementaryMetric,
                        extraData,
                        extraDataLength,
                        forceUpdate,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_WriteEntryWithSupplementaryMetric_Native(
        leaderboardName: crate::System::IntPtr,
        score: i64,
        supplementaryMetric: i64,
        extraData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        extraDataLength: u32,
        forceUpdate: bool,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    i64,
                    i64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    u32,
                    bool,
                ),
                u64,
                6usize,
            >("ovr_Leaderboard_WriteEntryWithSupplementaryMetric_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Leaderboard_WriteEntryWithSupplementaryMetric_Native",
                    6usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        leaderboardName,
                        score,
                        supplementaryMetric,
                        extraData,
                        extraDataLength,
                        forceUpdate,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_WriteEntry_Native(
        leaderboardName: crate::System::IntPtr,
        score: i64,
        extraData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        extraDataLength: u32,
        forceUpdate: bool,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    i64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    u32,
                    bool,
                ),
                u64,
                5usize,
            >("ovr_Leaderboard_WriteEntry_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Leaderboard_WriteEntry_Native", 5usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (leaderboardName, score, extraData, extraDataLength, forceUpdate),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LinkedAccountArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::UIntPtr),
                crate::System::IntPtr,
                2usize,
            >("ovr_LinkedAccountArray_GetElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LinkedAccountArray_GetElement", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LinkedAccountArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::UIntPtr,
                1usize,
            >("ovr_LinkedAccountArray_GetSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LinkedAccountArray_GetSize", 1usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LinkedAccount_GetAccessToken(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_LinkedAccount_GetAccessToken")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LinkedAccount_GetAccessToken", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LinkedAccount_GetAccessToken_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_LinkedAccount_GetAccessToken_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LinkedAccount_GetAccessToken_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LinkedAccount_GetServiceProvider(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::ServiceProvider> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::Oculus::Platform::ServiceProvider,
                1usize,
            >("ovr_LinkedAccount_GetServiceProvider")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LinkedAccount_GetServiceProvider", 1usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::ServiceProvider = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LinkedAccount_GetUserId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_LinkedAccount_GetUserId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LinkedAccount_GetUserId", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LinkedAccount_GetUserId_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_LinkedAccount_GetUserId_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LinkedAccount_GetUserId_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LivestreamingApplicationStatus_GetStreamingEnabled(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_LivestreamingApplicationStatus_GetStreamingEnabled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LivestreamingApplicationStatus_GetStreamingEnabled",
                    1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LivestreamingStartResult_GetStreamingResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        crate::Oculus::Platform::LivestreamingStartStatus,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::Oculus::Platform::LivestreamingStartStatus,
                1usize,
            >("ovr_LivestreamingStartResult_GetStreamingResult")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LivestreamingStartResult_GetStreamingResult", 1usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::LivestreamingStartStatus = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LivestreamingStatus_GetCommentsVisible(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_LivestreamingStatus_GetCommentsVisible")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LivestreamingStatus_GetCommentsVisible", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LivestreamingStatus_GetIsPaused(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_LivestreamingStatus_GetIsPaused")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LivestreamingStatus_GetIsPaused", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LivestreamingStatus_GetLivestreamingEnabled(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_LivestreamingStatus_GetLivestreamingEnabled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LivestreamingStatus_GetLivestreamingEnabled", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LivestreamingStatus_GetLivestreamingType(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                i32,
                1usize,
            >("ovr_LivestreamingStatus_GetLivestreamingType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LivestreamingStatus_GetLivestreamingType", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LivestreamingStatus_GetMicEnabled(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_LivestreamingStatus_GetMicEnabled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LivestreamingStatus_GetMicEnabled", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LivestreamingVideoStats_GetCommentCount(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                i32,
                1usize,
            >("ovr_LivestreamingVideoStats_GetCommentCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LivestreamingVideoStats_GetCommentCount", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LivestreamingVideoStats_GetReactionCount(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                i32,
                1usize,
            >("ovr_LivestreamingVideoStats_GetReactionCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LivestreamingVideoStats_GetReactionCount", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LivestreamingVideoStats_GetTotalViews(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_LivestreamingVideoStats_GetTotalViews")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LivestreamingVideoStats_GetTotalViews", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LivestreamingVideoStats_GetTotalViews_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_LivestreamingVideoStats_GetTotalViews_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_LivestreamingVideoStats_GetTotalViews_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Livestreaming_IsAllowedForApplication(
        packageName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ovr_Livestreaming_IsAllowedForApplication")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Livestreaming_IsAllowedForApplication", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (packageName)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Livestreaming_IsAllowedForApplication_Native(
        packageName: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_Livestreaming_IsAllowedForApplication_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Livestreaming_IsAllowedForApplication_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (packageName)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Livestreaming_StartPartyStream() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_Livestreaming_StartPartyStream")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Livestreaming_StartPartyStream", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Livestreaming_StartStream(
        audience: crate::Oculus::Platform::LivestreamingAudience,
        micStatus: crate::Oculus::Platform::LivestreamingMicrophoneStatus,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Oculus::Platform::LivestreamingAudience,
                    crate::Oculus::Platform::LivestreamingMicrophoneStatus,
                ),
                u64,
                2usize,
            >("ovr_Livestreaming_StartStream")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Livestreaming_StartStream", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (audience, micStatus))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Livestreaming_StopPartyStream() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_Livestreaming_StopPartyStream")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Livestreaming_StopPartyStream", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Livestreaming_StopStream() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_Livestreaming_StopStream")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Livestreaming_StopStream", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Livestreaming_UpdateMicStatus(
        micStatus: crate::Oculus::Platform::LivestreamingMicrophoneStatus,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Oculus::Platform::LivestreamingMicrophoneStatus),
                u64,
                1usize,
            >("ovr_Livestreaming_UpdateMicStatus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Livestreaming_UpdateMicStatus", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (micStatus)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Log_NewEvent(
        eventName: crate::System::IntPtr,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::IntPtr>,
        >,
        length: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::System::IntPtr>,
                    >,
                    crate::System::UIntPtr,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("ovr_Log_NewEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Log_NewEvent", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (eventName, values, length))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Media_ShareToFacebook(
        postTextSuggestion: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        contentType: crate::Oculus::Platform::MediaContentType,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::Oculus::Platform::MediaContentType,
                ),
                u64,
                3usize,
            >("ovr_Media_ShareToFacebook")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Media_ShareToFacebook", 3usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (postTextSuggestion, filePath, contentType))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Media_ShareToFacebook_Native(
        postTextSuggestion: crate::System::IntPtr,
        filePath: crate::System::IntPtr,
        contentType: crate::Oculus::Platform::MediaContentType,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::System::IntPtr,
                    crate::Oculus::Platform::MediaContentType,
                ),
                u64,
                3usize,
            >("ovr_Media_ShareToFacebook_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Media_ShareToFacebook_Native", 3usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (postTextSuggestion, filePath, contentType))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetAbuseReportRecording(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetAbuseReportRecording")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetAbuseReportRecording", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetAchievementDefinitionArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetAchievementDefinitionArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetAchievementDefinitionArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetAchievementProgressArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetAchievementProgressArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetAchievementProgressArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetAchievementUpdate(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetAchievementUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetAchievementUpdate", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetAppDownloadProgressResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetAppDownloadProgressResult")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetAppDownloadProgressResult", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetAppDownloadResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetAppDownloadResult")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetAppDownloadResult", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetApplicationInviteArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetApplicationInviteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetApplicationInviteArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetApplicationVersion(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetApplicationVersion")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetApplicationVersion", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetAssetDetails(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetAssetDetails")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetAssetDetails", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetAssetDetailsArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetAssetDetailsArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetAssetDetailsArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetAssetFileDeleteResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetAssetFileDeleteResult")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetAssetFileDeleteResult", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetAssetFileDownloadCancelResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetAssetFileDownloadCancelResult")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetAssetFileDownloadCancelResult", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetAssetFileDownloadResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetAssetFileDownloadResult")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetAssetFileDownloadResult", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetAssetFileDownloadUpdate(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetAssetFileDownloadUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetAssetFileDownloadUpdate", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetAvatarEditorResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetAvatarEditorResult")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetAvatarEditorResult", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetBlockedUserArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetBlockedUserArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetBlockedUserArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetChallenge(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetChallenge")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetChallenge", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetChallengeArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetChallengeArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetChallengeArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetChallengeEntryArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetChallengeEntryArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetChallengeEntryArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetDataStore(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetDataStore")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetDataStore", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetDestinationArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetDestinationArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetDestinationArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetError(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetError")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetError", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetGroupPresenceJoinIntent(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetGroupPresenceJoinIntent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetGroupPresenceJoinIntent", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetGroupPresenceLeaveIntent(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetGroupPresenceLeaveIntent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetGroupPresenceLeaveIntent", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetHttpTransferUpdate(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetHttpTransferUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetHttpTransferUpdate", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetInstalledApplicationArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetInstalledApplicationArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetInstalledApplicationArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetInvitePanelResultInfo(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetInvitePanelResultInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetInvitePanelResultInfo", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetLaunchBlockFlowResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetLaunchBlockFlowResult")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetLaunchBlockFlowResult", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetLaunchFriendRequestFlowResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetLaunchFriendRequestFlowResult")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetLaunchFriendRequestFlowResult", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetLaunchInvitePanelFlowResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetLaunchInvitePanelFlowResult")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetLaunchInvitePanelFlowResult", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetLaunchReportFlowResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetLaunchReportFlowResult")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetLaunchReportFlowResult", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetLaunchUnblockFlowResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetLaunchUnblockFlowResult")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetLaunchUnblockFlowResult", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetLeaderboardArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetLeaderboardArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetLeaderboardArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetLeaderboardEntryArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetLeaderboardEntryArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetLeaderboardEntryArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetLeaderboardUpdateStatus(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetLeaderboardUpdateStatus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetLeaderboardUpdateStatus", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetLinkedAccountArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetLinkedAccountArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetLinkedAccountArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetLivestreamingApplicationStatus(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetLivestreamingApplicationStatus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetLivestreamingApplicationStatus", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetLivestreamingStartResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetLivestreamingStartResult")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetLivestreamingStartResult", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetLivestreamingStatus(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetLivestreamingStatus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetLivestreamingStatus", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetLivestreamingVideoStats(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetLivestreamingVideoStats")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetLivestreamingVideoStats", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetMicrophoneAvailabilityState(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetMicrophoneAvailabilityState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetMicrophoneAvailabilityState", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetNativeMessage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetNativeMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetNativeMessage", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetNetSyncConnection(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetNetSyncConnection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetNetSyncConnection", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetNetSyncSessionArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetNetSyncSessionArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetNetSyncSessionArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetNetSyncSessionsChangedNotification(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetNetSyncSessionsChangedNotification")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetNetSyncSessionsChangedNotification", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetNetSyncSetSessionPropertyResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetNetSyncSetSessionPropertyResult")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetNetSyncSetSessionPropertyResult", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetNetSyncVoipAttenuationValueArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetNetSyncVoipAttenuationValueArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetNetSyncVoipAttenuationValueArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetOrgScopedID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetOrgScopedID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetOrgScopedID", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetParty(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetParty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetParty", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetPartyID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetPartyID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetPartyID", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetPartyUpdateNotification(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetPartyUpdateNotification")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetPartyUpdateNotification", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetPidArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetPidArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetPidArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetPlatformInitialize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetPlatformInitialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetPlatformInitialize", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetProductArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetProductArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetProductArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetPurchase(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetPurchase")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetPurchase", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetPurchaseArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetPurchaseArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetPurchaseArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetRejoinDialogResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetRejoinDialogResult")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetRejoinDialogResult", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetRequestID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_Message_GetRequestID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetRequestID", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetSdkAccountArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetSdkAccountArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetSdkAccountArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetSendInvitesResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetSendInvitesResult")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetSendInvitesResult", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetShareMediaResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetShareMediaResult")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetShareMediaResult", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetString(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_Message_GetString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetStringForJavascript(
        message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_Message_GetStringForJavascript")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetStringForJavascript", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (message)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetStringForJavascript_Native(
        message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetStringForJavascript_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetStringForJavascript_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (message))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetString_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetString_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetString_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetSystemVoipState(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetSystemVoipState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetSystemVoipState", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetType(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::Message_MessageType> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::Oculus::Platform::Message_MessageType,
                1usize,
            >("ovr_Message_GetType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetType", 1usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::Message_MessageType = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetUser(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetUser")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetUser", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetUserAccountAgeCategory(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetUserAccountAgeCategory")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetUserAccountAgeCategory", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetUserArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetUserArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetUserArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetUserCapabilityArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetUserCapabilityArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetUserCapabilityArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetUserDataStoreUpdateResponse(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetUserDataStoreUpdateResponse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetUserDataStoreUpdateResponse", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetUserProof(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetUserProof")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetUserProof", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetUserReportID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Message_GetUserReportID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_GetUserReportID", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_IsError(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_Message_IsError")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Message_IsError", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_MicrophoneAvailabilityState_GetMicrophoneAvailable(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_MicrophoneAvailabilityState_GetMicrophoneAvailable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_MicrophoneAvailabilityState_GetMicrophoneAvailable",
                    1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Microphone_Create() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::System::IntPtr,
                0usize,
            >("ovr_Microphone_Create")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Microphone_Create", 0usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Microphone_Destroy(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_Microphone_Destroy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Microphone_Destroy", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Microphone_GetNumSamplesAvailable(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::UIntPtr,
                1usize,
            >("ovr_Microphone_GetNumSamplesAvailable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Microphone_GetNumSamplesAvailable", 1usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Microphone_GetOutputBufferMaxSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::UIntPtr,
                1usize,
            >("ovr_Microphone_GetOutputBufferMaxSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Microphone_GetOutputBufferMaxSize", 1usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Microphone_GetPCM(
        obj: crate::System::IntPtr,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
        outputBufferNumElements: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
                    crate::System::UIntPtr,
                ),
                crate::System::UIntPtr,
                3usize,
            >("ovr_Microphone_GetPCM")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Microphone_GetPCM", 3usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj, outputBuffer, outputBufferNumElements))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Microphone_GetPCMFloat(
        obj: crate::System::IntPtr,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        outputBufferNumElements: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    crate::System::UIntPtr,
                ),
                crate::System::UIntPtr,
                3usize,
            >("ovr_Microphone_GetPCMFloat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Microphone_GetPCMFloat", 3usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj, outputBuffer, outputBufferNumElements))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Microphone_ReadData(
        obj: crate::System::IntPtr,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        outputBufferSize: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    crate::System::UIntPtr,
                ),
                crate::System::UIntPtr,
                3usize,
            >("ovr_Microphone_ReadData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Microphone_ReadData", 3usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj, outputBuffer, outputBufferSize))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Microphone_SetAcceptableRecordingDelayHint(
        obj: crate::System::IntPtr,
        delayMs: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::UIntPtr),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_Microphone_SetAcceptableRecordingDelayHint")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Microphone_SetAcceptableRecordingDelayHint", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (obj, delayMs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Microphone_Start(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_Microphone_Start")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Microphone_Start", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Microphone_Stop(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_Microphone_Stop")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Microphone_Stop", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_MultiplayerErrorOptions_Create() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::System::IntPtr,
                0usize,
            >("ovr_MultiplayerErrorOptions_Create")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_MultiplayerErrorOptions_Create", 0usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_MultiplayerErrorOptions_Destroy(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_MultiplayerErrorOptions_Destroy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_MultiplayerErrorOptions_Destroy", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_MultiplayerErrorOptions_SetErrorKey(
        handle: crate::System::IntPtr,
        value: crate::Oculus::Platform::MultiplayerErrorErrorKey,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    crate::Oculus::Platform::MultiplayerErrorErrorKey,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_MultiplayerErrorOptions_SetErrorKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_MultiplayerErrorOptions_SetErrorKey", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncConnection_GetConnectionId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                i64,
                1usize,
            >("ovr_NetSyncConnection_GetConnectionId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSyncConnection_GetConnectionId", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncConnection_GetDisconnectReason(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        crate::Oculus::Platform::NetSyncDisconnectReason,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::Oculus::Platform::NetSyncDisconnectReason,
                1usize,
            >("ovr_NetSyncConnection_GetDisconnectReason")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSyncConnection_GetDisconnectReason", 1usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::NetSyncDisconnectReason = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncConnection_GetSessionId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_NetSyncConnection_GetSessionId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSyncConnection_GetSessionId", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncConnection_GetStatus(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        crate::Oculus::Platform::NetSyncConnectionStatus,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::Oculus::Platform::NetSyncConnectionStatus,
                1usize,
            >("ovr_NetSyncConnection_GetStatus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSyncConnection_GetStatus", 1usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::NetSyncConnectionStatus = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncConnection_GetZoneId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_NetSyncConnection_GetZoneId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSyncConnection_GetZoneId", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncConnection_GetZoneId_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_NetSyncConnection_GetZoneId_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSyncConnection_GetZoneId_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncOptions_Create() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::System::IntPtr,
                0usize,
            >("ovr_NetSyncOptions_Create")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSyncOptions_Create", 0usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncOptions_Destroy(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_NetSyncOptions_Destroy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSyncOptions_Destroy", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncOptions_SetVoipGroup(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_NetSyncOptions_SetVoipGroup")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSyncOptions_SetVoipGroup", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncOptions_SetVoipGroup_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_NetSyncOptions_SetVoipGroup_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSyncOptions_SetVoipGroup_Native", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncOptions_SetVoipStreamDefault(
        handle: crate::System::IntPtr,
        value: crate::Oculus::Platform::NetSyncVoipStreamMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::Oculus::Platform::NetSyncVoipStreamMode),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_NetSyncOptions_SetVoipStreamDefault")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSyncOptions_SetVoipStreamDefault", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncOptions_SetZoneId(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_NetSyncOptions_SetZoneId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSyncOptions_SetZoneId", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncOptions_SetZoneId_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_NetSyncOptions_SetZoneId_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSyncOptions_SetZoneId_Native", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncSessionArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::UIntPtr),
                crate::System::IntPtr,
                2usize,
            >("ovr_NetSyncSessionArray_GetElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSyncSessionArray_GetElement", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncSessionArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::UIntPtr,
                1usize,
            >("ovr_NetSyncSessionArray_GetSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSyncSessionArray_GetSize", 1usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncSession_GetConnectionId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                i64,
                1usize,
            >("ovr_NetSyncSession_GetConnectionId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSyncSession_GetConnectionId", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncSession_GetMuted(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_NetSyncSession_GetMuted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSyncSession_GetMuted", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncSession_GetSessionId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_NetSyncSession_GetSessionId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSyncSession_GetSessionId", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncSession_GetUserId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_NetSyncSession_GetUserId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSyncSession_GetUserId", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncSession_GetVoipGroup(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_NetSyncSession_GetVoipGroup")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSyncSession_GetVoipGroup", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncSession_GetVoipGroup_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_NetSyncSession_GetVoipGroup_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSyncSession_GetVoipGroup_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncSessionsChangedNotification_GetConnectionId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                i64,
                1usize,
            >("ovr_NetSyncSessionsChangedNotification_GetConnectionId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSyncSessionsChangedNotification_GetConnectionId",
                    1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncSessionsChangedNotification_GetSessions(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_NetSyncSessionsChangedNotification_GetSessions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSyncSessionsChangedNotification_GetSessions", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncSetSessionPropertyResult_GetSession(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_NetSyncSetSessionPropertyResult_GetSession")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSyncSetSessionPropertyResult_GetSession", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncVoipAttenuationValueArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::UIntPtr),
                crate::System::IntPtr,
                2usize,
            >("ovr_NetSyncVoipAttenuationValueArray_GetElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSyncVoipAttenuationValueArray_GetElement", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncVoipAttenuationValueArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::UIntPtr,
                1usize,
            >("ovr_NetSyncVoipAttenuationValueArray_GetSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSyncVoipAttenuationValueArray_GetSize", 1usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncVoipAttenuationValue_GetDecibels(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                f32,
                1usize,
            >("ovr_NetSyncVoipAttenuationValue_GetDecibels")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSyncVoipAttenuationValue_GetDecibels", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncVoipAttenuationValue_GetDistance(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                f32,
                1usize,
            >("ovr_NetSyncVoipAttenuationValue_GetDistance")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSyncVoipAttenuationValue_GetDistance", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_Connect(
        connect_options: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_NetSync_Connect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSync_Connect", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (connect_options)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_Disconnect(
        connection_id: i64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i64), u64, 1usize>("ovr_NetSync_Disconnect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSync_Disconnect", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (connection_id)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_GetAmbisonicFloatPCM(
        connection_id: i64,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        outputBufferNumElements: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    crate::System::UIntPtr,
                ),
                crate::System::UIntPtr,
                3usize,
            >("ovr_NetSync_GetAmbisonicFloatPCM")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSync_GetAmbisonicFloatPCM", 3usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (connection_id, outputBuffer, outputBufferNumElements),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_GetAmbisonicInt16PCM(
        connection_id: i64,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
        outputBufferNumElements: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
                    crate::System::UIntPtr,
                ),
                crate::System::UIntPtr,
                3usize,
            >("ovr_NetSync_GetAmbisonicInt16PCM")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSync_GetAmbisonicInt16PCM", 3usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (connection_id, outputBuffer, outputBufferNumElements),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_GetAmbisonicInterleavedFloatPCM(
        connection_id: i64,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        outputBufferNumElements: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    crate::System::UIntPtr,
                ),
                crate::System::UIntPtr,
                3usize,
            >("ovr_NetSync_GetAmbisonicInterleavedFloatPCM")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSync_GetAmbisonicInterleavedFloatPCM", 3usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (connection_id, outputBuffer, outputBufferNumElements),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_GetAmbisonicInterleavedInt16PCM(
        connection_id: i64,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
        outputBufferNumElements: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
                    crate::System::UIntPtr,
                ),
                crate::System::UIntPtr,
                3usize,
            >("ovr_NetSync_GetAmbisonicInterleavedInt16PCM")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSync_GetAmbisonicInterleavedInt16PCM", 3usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (connection_id, outputBuffer, outputBufferNumElements),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_GetListenerPosition(
        connection_id: i64,
        sessionId: u64,
        position: quest_hook::libil2cpp::ByRefMut<
            crate::Oculus::Platform::CAPI_ovrNetSyncVec3,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i64,
                    u64,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::Oculus::Platform::CAPI_ovrNetSyncVec3,
                    >,
                ),
                bool,
                3usize,
            >("ovr_NetSync_GetListenerPosition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSync_GetListenerPosition", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (connection_id, sessionId, position))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_GetMonostreamFloatPCM(
        connection_id: i64,
        sessionId: u64,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        outputBufferNumElements: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i64,
                    u64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    crate::System::UIntPtr,
                ),
                crate::System::UIntPtr,
                4usize,
            >("ovr_NetSync_GetMonostreamFloatPCM")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSync_GetMonostreamFloatPCM", 4usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (connection_id, sessionId, outputBuffer, outputBufferNumElements),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_GetMonostreamInt16PCM(
        connection_id: i64,
        session_id: u64,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
        outputBufferNumElements: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i64,
                    u64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
                    crate::System::UIntPtr,
                ),
                crate::System::UIntPtr,
                4usize,
            >("ovr_NetSync_GetMonostreamInt16PCM")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSync_GetMonostreamInt16PCM", 4usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (connection_id, session_id, outputBuffer, outputBufferNumElements),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_GetPcmBufferMaxSamples() -> quest_hook::libil2cpp::Result<
        crate::System::UIntPtr,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::System::UIntPtr,
                0usize,
            >("ovr_NetSync_GetPcmBufferMaxSamples")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSync_GetPcmBufferMaxSamples", 0usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_GetSessions(
        connection_id: i64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i64), u64, 1usize>("ovr_NetSync_GetSessions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSync_GetSessions", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (connection_id)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_GetVoipAmplitude(
        connection_id: i64,
        sessionId: u64,
        amplitude: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i64, u64, quest_hook::libil2cpp::ByRefMut<f32>),
                bool,
                3usize,
            >("ovr_NetSync_GetVoipAmplitude")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSync_GetVoipAmplitude", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (connection_id, sessionId, amplitude))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_GetVoipAttenuation(
        connection_id: i64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i64), u64, 1usize>("ovr_NetSync_GetVoipAttenuation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSync_GetVoipAttenuation", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (connection_id)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_GetVoipAttenuationDefault() -> quest_hook::libil2cpp::Result<
        u64,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                u64,
                0usize,
            >("ovr_NetSync_GetVoipAttenuationDefault")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSync_GetVoipAttenuationDefault", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_SetListenerPosition(
        connection_id: i64,
        position: quest_hook::libil2cpp::ByRefMut<
            crate::Oculus::Platform::CAPI_ovrNetSyncVec3,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i64,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::Oculus::Platform::CAPI_ovrNetSyncVec3,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_NetSync_SetListenerPosition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSync_SetListenerPosition", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (connection_id, position))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_SetVoipAttenuation(
        connection_id: i64,
        distances: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        decibels: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        count: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    crate::System::UIntPtr,
                ),
                u64,
                4usize,
            >("ovr_NetSync_SetVoipAttenuation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSync_SetVoipAttenuation", 4usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (connection_id, distances, decibels, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_SetVoipAttenuationModel(
        connection_id: i64,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        distances: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        decibels: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        count: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    crate::System::UIntPtr,
                ),
                u64,
                5usize,
            >("ovr_NetSync_SetVoipAttenuationModel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSync_SetVoipAttenuationModel", 5usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method
                .invoke_unchecked((), (connection_id, name, distances, decibels, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_SetVoipAttenuationModel_Native(
        connection_id: i64,
        name: crate::System::IntPtr,
        distances: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        decibels: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        count: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i64,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    crate::System::UIntPtr,
                ),
                u64,
                5usize,
            >("ovr_NetSync_SetVoipAttenuationModel_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSync_SetVoipAttenuationModel_Native", 5usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method
                .invoke_unchecked((), (connection_id, name, distances, decibels, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_SetVoipChannelCfg(
        connection_id: i64,
        channel_name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attnmodel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        disable_spatialization: bool,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    bool,
                ),
                u64,
                4usize,
            >("ovr_NetSync_SetVoipChannelCfg")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSync_SetVoipChannelCfg", 4usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (connection_id, channel_name, attnmodel, disable_spatialization),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_SetVoipChannelCfg_Native(
        connection_id: i64,
        channel_name: crate::System::IntPtr,
        attnmodel: crate::System::IntPtr,
        disable_spatialization: bool,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i64, crate::System::IntPtr, crate::System::IntPtr, bool),
                u64,
                4usize,
            >("ovr_NetSync_SetVoipChannelCfg_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSync_SetVoipChannelCfg_Native", 4usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (connection_id, channel_name, attnmodel, disable_spatialization),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_SetVoipGroup(
        connection_id: i64,
        group_id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i64, quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                2usize,
            >("ovr_NetSync_SetVoipGroup")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSync_SetVoipGroup", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (connection_id, group_id))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_SetVoipGroup_Native(
        connection_id: i64,
        group_id: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i64, crate::System::IntPtr),
                u64,
                2usize,
            >("ovr_NetSync_SetVoipGroup_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSync_SetVoipGroup_Native", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (connection_id, group_id))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_SetVoipListentoChannels(
        connection_id: i64,
        listento_channels: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        count: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i64,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        >,
                    >,
                    crate::System::UIntPtr,
                ),
                u64,
                3usize,
            >("ovr_NetSync_SetVoipListentoChannels")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSync_SetVoipListentoChannels", 3usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (connection_id, listento_channels, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_SetVoipMicSource(
        connection_id: i64,
        mic_source: crate::Oculus::Platform::NetSyncVoipMicSource,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i64, crate::Oculus::Platform::NetSyncVoipMicSource),
                u64,
                2usize,
            >("ovr_NetSync_SetVoipMicSource")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSync_SetVoipMicSource", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (connection_id, mic_source))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_SetVoipSessionMuted(
        connection_id: i64,
        session_id: u64,
        muted: bool,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i64, u64, bool),
                u64,
                3usize,
            >("ovr_NetSync_SetVoipSessionMuted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSync_SetVoipSessionMuted", 3usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (connection_id, session_id, muted))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_SetVoipSpeaktoChannels(
        connection_id: i64,
        speakto_channels: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        count: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i64,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        >,
                    >,
                    crate::System::UIntPtr,
                ),
                u64,
                3usize,
            >("ovr_NetSync_SetVoipSpeaktoChannels")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSync_SetVoipSpeaktoChannels", 3usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (connection_id, speakto_channels, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_SetVoipStreamMode(
        connection_id: i64,
        sessionId: u64,
        streamMode: crate::Oculus::Platform::NetSyncVoipStreamMode,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i64, u64, crate::Oculus::Platform::NetSyncVoipStreamMode),
                u64,
                3usize,
            >("ovr_NetSync_SetVoipStreamMode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_NetSync_SetVoipStreamMode", 3usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (connection_id, sessionId, streamMode))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Notification_MarkAsRead(
        notificationID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("ovr_Notification_MarkAsRead")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Notification_MarkAsRead", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (notificationID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_OrgScopedID_GetID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_OrgScopedID_GetID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_OrgScopedID_GetID", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Packet_Free(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_Packet_Free")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Packet_Free", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Packet_GetBytes(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Packet_GetBytes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Packet_GetBytes", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Packet_GetSenderID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_Packet_GetSenderID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Packet_GetSenderID", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Packet_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::UIntPtr,
                1usize,
            >("ovr_Packet_GetSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Packet_GetSize", 1usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PartyID_GetID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_PartyID_GetID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_PartyID_GetID", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PartyUpdateNotification_GetAction(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::PartyUpdateAction> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::Oculus::Platform::PartyUpdateAction,
                1usize,
            >("ovr_PartyUpdateNotification_GetAction")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_PartyUpdateNotification_GetAction", 1usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::PartyUpdateAction = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PartyUpdateNotification_GetPartyId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_PartyUpdateNotification_GetPartyId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_PartyUpdateNotification_GetPartyId", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PartyUpdateNotification_GetSenderId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_PartyUpdateNotification_GetSenderId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_PartyUpdateNotification_GetSenderId", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PartyUpdateNotification_GetUpdateTimestamp(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_PartyUpdateNotification_GetUpdateTimestamp")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_PartyUpdateNotification_GetUpdateTimestamp", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PartyUpdateNotification_GetUpdateTimestamp_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_PartyUpdateNotification_GetUpdateTimestamp_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_PartyUpdateNotification_GetUpdateTimestamp_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PartyUpdateNotification_GetUserAlias(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_PartyUpdateNotification_GetUserAlias")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_PartyUpdateNotification_GetUserAlias", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PartyUpdateNotification_GetUserAlias_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_PartyUpdateNotification_GetUserAlias_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_PartyUpdateNotification_GetUserAlias_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PartyUpdateNotification_GetUserId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_PartyUpdateNotification_GetUserId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_PartyUpdateNotification_GetUserId", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PartyUpdateNotification_GetUserName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_PartyUpdateNotification_GetUserName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_PartyUpdateNotification_GetUserName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PartyUpdateNotification_GetUserName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_PartyUpdateNotification_GetUserName_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_PartyUpdateNotification_GetUserName_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_Create() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_Party_Create")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Party_Create", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_GatherInApplication(
        partyID: u64,
        appID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64, u64),
                u64,
                2usize,
            >("ovr_Party_GatherInApplication")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Party_GatherInApplication", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (partyID, appID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_Get(partyID: u64) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("ovr_Party_Get")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Party_Get", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (partyID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_GetCurrent() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_Party_GetCurrent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Party_GetCurrent", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_GetCurrentForUser(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("ovr_Party_GetCurrentForUser")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Party_GetCurrentForUser", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (userID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_GetID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_Party_GetID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Party_GetID", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_GetInvitedUsers(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Party_GetInvitedUsers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Party_GetInvitedUsers", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_GetLeader(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Party_GetLeader")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Party_GetLeader", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_GetUsers(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Party_GetUsers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Party_GetUsers", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_Invite(
        partyID: u64,
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64, u64), u64, 2usize>("ovr_Party_Invite")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Party_Invite", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (partyID, userID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_Join(partyID: u64) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("ovr_Party_Join")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Party_Join", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (partyID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_Leave(partyID: u64) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("ovr_Party_Leave")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Party_Leave", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (partyID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_PluginGetSharedMemHandle() -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), i32, 0usize>("ovr_Party_PluginGetSharedMemHandle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Party_PluginGetSharedMemHandle", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_PluginGetVoipMicrophoneMuted() -> quest_hook::libil2cpp::Result<
        crate::Oculus::Platform::VoipMuteState,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::Oculus::Platform::VoipMuteState,
                0usize,
            >("ovr_Party_PluginGetVoipMicrophoneMuted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Party_PluginGetVoipMicrophoneMuted", 0usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::VoipMuteState = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_PluginGetVoipPassthrough() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("ovr_Party_PluginGetVoipPassthrough")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Party_PluginGetVoipPassthrough", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_PluginGetVoipStatus() -> quest_hook::libil2cpp::Result<
        crate::Oculus::Platform::SystemVoipStatus,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::Oculus::Platform::SystemVoipStatus,
                0usize,
            >("ovr_Party_PluginGetVoipStatus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Party_PluginGetVoipStatus", 0usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::SystemVoipStatus = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PidArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::UIntPtr),
                crate::System::IntPtr,
                2usize,
            >("ovr_PidArray_GetElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_PidArray_GetElement", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PidArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::UIntPtr,
                1usize,
            >("ovr_PidArray_GetSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_PidArray_GetSize", 1usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Pid_GetId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_Pid_GetId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Pid_GetId", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Pid_GetId_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Pid_GetId_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Pid_GetId_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PlatformInitializeWithAccessToken(
        appId: u64,
        accessToken: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64, quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                2usize,
            >("ovr_PlatformInitializeWithAccessToken")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_PlatformInitializeWithAccessToken", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (appId, accessToken))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PlatformInitializeWithAccessTokenAndOptions(
        appId: u64,
        accessToken: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        configOptions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::Oculus::Platform::CAPI_ovrKeyValuePair,
            >,
        >,
        numOptions: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    u64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::Oculus::Platform::CAPI_ovrKeyValuePair,
                        >,
                    >,
                    crate::System::UIntPtr,
                ),
                u64,
                4usize,
            >("ovr_PlatformInitializeWithAccessTokenAndOptions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_PlatformInitializeWithAccessTokenAndOptions", 4usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (appId, accessToken, configOptions, numOptions))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PlatformInitialize_GetResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        crate::Oculus::Platform::PlatformInitializeResult,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::Oculus::Platform::PlatformInitializeResult,
                1usize,
            >("ovr_PlatformInitialize_GetResult")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_PlatformInitialize_GetResult", 1usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::PlatformInitializeResult = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Platform_InitializeStandaloneOculus(
        init: quest_hook::libil2cpp::ByRefMut<
            crate::Oculus::Platform::CAPI_OculusInitParams,
        >,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    crate::Oculus::Platform::CAPI_OculusInitParams,
                >),
                u64,
                1usize,
            >("ovr_Platform_InitializeStandaloneOculus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Platform_InitializeStandaloneOculus", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (init)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PopMessage() -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), crate::System::IntPtr, 0usize>("ovr_PopMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_PopMessage", 0usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Price_GetAmountInHundredths(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u32,
                1usize,
            >("ovr_Price_GetAmountInHundredths")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Price_GetAmountInHundredths", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Price_GetCurrency(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_Price_GetCurrency")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Price_GetCurrency", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Price_GetCurrency_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Price_GetCurrency_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Price_GetCurrency_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Price_GetFormatted(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_Price_GetFormatted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Price_GetFormatted", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Price_GetFormatted_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Price_GetFormatted_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Price_GetFormatted_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ProductArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::UIntPtr),
                crate::System::IntPtr,
                2usize,
            >("ovr_ProductArray_GetElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ProductArray_GetElement", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ProductArray_GetNextUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_ProductArray_GetNextUrl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ProductArray_GetNextUrl", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ProductArray_GetNextUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_ProductArray_GetNextUrl_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ProductArray_GetNextUrl_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ProductArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::UIntPtr,
                1usize,
            >("ovr_ProductArray_GetSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ProductArray_GetSize", 1usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ProductArray_HasNextPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_ProductArray_HasNextPage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ProductArray_HasNextPage", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Product_GetDescription(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_Product_GetDescription")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Product_GetDescription", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Product_GetDescription_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Product_GetDescription_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Product_GetDescription_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Product_GetFormattedPrice(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_Product_GetFormattedPrice")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Product_GetFormattedPrice", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Product_GetFormattedPrice_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Product_GetFormattedPrice_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Product_GetFormattedPrice_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Product_GetName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_Product_GetName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Product_GetName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Product_GetName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Product_GetName_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Product_GetName_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Product_GetSKU(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_Product_GetSKU")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Product_GetSKU", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Product_GetSKU_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Product_GetSKU_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Product_GetSKU_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PurchaseArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::UIntPtr),
                crate::System::IntPtr,
                2usize,
            >("ovr_PurchaseArray_GetElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_PurchaseArray_GetElement", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PurchaseArray_GetNextUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_PurchaseArray_GetNextUrl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_PurchaseArray_GetNextUrl", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PurchaseArray_GetNextUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_PurchaseArray_GetNextUrl_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_PurchaseArray_GetNextUrl_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PurchaseArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::UIntPtr,
                1usize,
            >("ovr_PurchaseArray_GetSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_PurchaseArray_GetSize", 1usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PurchaseArray_HasNextPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_PurchaseArray_HasNextPage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_PurchaseArray_HasNextPage", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Purchase_GetDeveloperPayload(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_Purchase_GetDeveloperPayload")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Purchase_GetDeveloperPayload", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Purchase_GetDeveloperPayload_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Purchase_GetDeveloperPayload_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Purchase_GetDeveloperPayload_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Purchase_GetExpirationTime(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::DateTime,
                1usize,
            >("ovr_Purchase_GetExpirationTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Purchase_GetExpirationTime", 1usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Purchase_GetExpirationTime_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_Purchase_GetExpirationTime_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Purchase_GetExpirationTime_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Purchase_GetGrantTime(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::DateTime,
                1usize,
            >("ovr_Purchase_GetGrantTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Purchase_GetGrantTime", 1usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Purchase_GetGrantTime_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_Purchase_GetGrantTime_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Purchase_GetGrantTime_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Purchase_GetPurchaseID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_Purchase_GetPurchaseID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Purchase_GetPurchaseID", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Purchase_GetPurchaseStrID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_Purchase_GetPurchaseStrID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Purchase_GetPurchaseStrID", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Purchase_GetPurchaseStrID_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Purchase_GetPurchaseStrID_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Purchase_GetPurchaseStrID_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Purchase_GetReportingId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_Purchase_GetReportingId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Purchase_GetReportingId", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Purchase_GetReportingId_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Purchase_GetReportingId_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Purchase_GetReportingId_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Purchase_GetSKU(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_Purchase_GetSKU")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Purchase_GetSKU", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Purchase_GetSKU_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_Purchase_GetSKU_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Purchase_GetSKU_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RejoinDialogResult_GetRejoinSelected(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_RejoinDialogResult_GetRejoinSelected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_RejoinDialogResult_GetRejoinSelected", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresenceOptions_Create() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::System::IntPtr,
                0usize,
            >("ovr_RichPresenceOptions_Create")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_RichPresenceOptions_Create", 0usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresenceOptions_Destroy(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_RichPresenceOptions_Destroy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_RichPresenceOptions_Destroy", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresenceOptions_SetApiName(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_RichPresenceOptions_SetApiName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_RichPresenceOptions_SetApiName", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresenceOptions_SetApiName_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_RichPresenceOptions_SetApiName_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_RichPresenceOptions_SetApiName_Native", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresenceOptions_SetDeeplinkMessageOverride(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_RichPresenceOptions_SetDeeplinkMessageOverride")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_RichPresenceOptions_SetDeeplinkMessageOverride", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresenceOptions_SetDeeplinkMessageOverride_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_RichPresenceOptions_SetDeeplinkMessageOverride_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_RichPresenceOptions_SetDeeplinkMessageOverride_Native",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresenceOptions_SetIsJoinable(
        handle: crate::System::IntPtr,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_RichPresenceOptions_SetIsJoinable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_RichPresenceOptions_SetIsJoinable", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresence_Clear() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_RichPresence_Clear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_RichPresence_Clear", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresence_GetDestinations() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_RichPresence_GetDestinations")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_RichPresence_GetDestinations", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresence_Set(
        richPresenceOptions: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_RichPresence_Set")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_RichPresence_Set", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (richPresenceOptions))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresence_SetDestination(
        api_name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ovr_RichPresence_SetDestination")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_RichPresence_SetDestination", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (api_name)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresence_SetDestination_Native(
        api_name: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_RichPresence_SetDestination_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_RichPresence_SetDestination_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (api_name)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresence_SetIsJoinable(
        is_joinable: bool,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(bool), u64, 1usize>("ovr_RichPresence_SetIsJoinable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_RichPresence_SetIsJoinable", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (is_joinable)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresence_SetLobbySession(
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ovr_RichPresence_SetLobbySession")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_RichPresence_SetLobbySession", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (id)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresence_SetLobbySession_Native(
        id: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_RichPresence_SetLobbySession_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_RichPresence_SetLobbySession_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (id)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresence_SetMatchSession(
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ovr_RichPresence_SetMatchSession")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_RichPresence_SetMatchSession", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (id)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresence_SetMatchSession_Native(
        id: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_RichPresence_SetMatchSession_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_RichPresence_SetMatchSession_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (id)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RosterOptions_AddSuggestedUser(
        handle: crate::System::IntPtr,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, u64),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_RosterOptions_AddSuggestedUser")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_RosterOptions_AddSuggestedUser", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RosterOptions_ClearSuggestedUsers(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_RosterOptions_ClearSuggestedUsers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_RosterOptions_ClearSuggestedUsers", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RosterOptions_Create() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::System::IntPtr,
                0usize,
            >("ovr_RosterOptions_Create")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_RosterOptions_Create", 0usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RosterOptions_Destroy(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_RosterOptions_Destroy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_RosterOptions_Destroy", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_SdkAccountArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::UIntPtr),
                crate::System::IntPtr,
                2usize,
            >("ovr_SdkAccountArray_GetElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_SdkAccountArray_GetElement", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_SdkAccountArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::UIntPtr,
                1usize,
            >("ovr_SdkAccountArray_GetSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_SdkAccountArray_GetSize", 1usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_SdkAccount_GetAccountType(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::SdkAccountType> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::Oculus::Platform::SdkAccountType,
                1usize,
            >("ovr_SdkAccount_GetAccountType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_SdkAccount_GetAccountType", 1usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::SdkAccountType = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_SdkAccount_GetUserId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_SdkAccount_GetUserId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_SdkAccount_GetUserId", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_SendInvitesResult_GetInvites(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_SendInvitesResult_GetInvites")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_SendInvitesResult_GetInvites", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_SetDeveloperAccessToken(
        accessToken: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("ovr_SetDeveloperAccessToken")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_SetDeveloperAccessToken", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (accessToken)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ShareMediaResult_GetStatus(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::ShareMediaStatus> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::Oculus::Platform::ShareMediaStatus,
                1usize,
            >("ovr_ShareMediaResult_GetStatus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_ShareMediaResult_GetStatus", 1usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::ShareMediaStatus = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_SupplementaryMetric_GetID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_SupplementaryMetric_GetID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_SupplementaryMetric_GetID", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_SupplementaryMetric_GetMetric(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                i64,
                1usize,
            >("ovr_SupplementaryMetric_GetMetric")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_SupplementaryMetric_GetMetric", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_SystemVoipState_GetMicrophoneMuted(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::VoipMuteState> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::Oculus::Platform::VoipMuteState,
                1usize,
            >("ovr_SystemVoipState_GetMicrophoneMuted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_SystemVoipState_GetMicrophoneMuted", 1usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::VoipMuteState = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_SystemVoipState_GetStatus(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::SystemVoipStatus> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::Oculus::Platform::SystemVoipStatus,
                1usize,
            >("ovr_SystemVoipState_GetStatus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_SystemVoipState_GetStatus", 1usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::SystemVoipStatus = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUserAppAccessArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::UIntPtr),
                crate::System::IntPtr,
                2usize,
            >("ovr_TestUserAppAccessArray_GetElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_TestUserAppAccessArray_GetElement", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUserAppAccessArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::UIntPtr,
                1usize,
            >("ovr_TestUserAppAccessArray_GetSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_TestUserAppAccessArray_GetSize", 1usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUserAppAccess_GetAccessToken(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_TestUserAppAccess_GetAccessToken")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_TestUserAppAccess_GetAccessToken", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUserAppAccess_GetAccessToken_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_TestUserAppAccess_GetAccessToken_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_TestUserAppAccess_GetAccessToken_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUserAppAccess_GetAppId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_TestUserAppAccess_GetAppId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_TestUserAppAccess_GetAppId", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUserAppAccess_GetUserId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_TestUserAppAccess_GetUserId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_TestUserAppAccess_GetUserId", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUser_GetAccessToken(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_TestUser_GetAccessToken")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_TestUser_GetAccessToken", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUser_GetAccessToken_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_TestUser_GetAccessToken_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_TestUser_GetAccessToken_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUser_GetAppAccessArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_TestUser_GetAppAccessArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_TestUser_GetAppAccessArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUser_GetFbAppAccessArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_TestUser_GetFbAppAccessArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_TestUser_GetFbAppAccessArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUser_GetFriendAccessToken(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_TestUser_GetFriendAccessToken")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_TestUser_GetFriendAccessToken", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUser_GetFriendAccessToken_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_TestUser_GetFriendAccessToken_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_TestUser_GetFriendAccessToken_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUser_GetFriendAppAccessArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_TestUser_GetFriendAppAccessArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_TestUser_GetFriendAppAccessArray", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUser_GetUserAlias(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_TestUser_GetUserAlias")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_TestUser_GetUserAlias", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUser_GetUserAlias_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_TestUser_GetUserAlias_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_TestUser_GetUserAlias_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUser_GetUserFbid(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_TestUser_GetUserFbid")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_TestUser_GetUserFbid", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUser_GetUserId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_TestUser_GetUserId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_TestUser_GetUserId", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UnityInitGlobals(
        loggingCB: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_UnityInitGlobals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UnityInitGlobals", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (loggingCB))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UnityInitWrapper(
        appId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("ovr_UnityInitWrapper")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UnityInitWrapper", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (appId)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UnityInitWrapperAsynchronous(
        appId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ovr_UnityInitWrapperAsynchronous")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UnityInitWrapperAsynchronous", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (appId)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UnityInitWrapperStandalone(
        accessToken: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loggingCB: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::System::IntPtr,
                ),
                bool,
                2usize,
            >("ovr_UnityInitWrapperStandalone")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UnityInitWrapperStandalone", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (accessToken, loggingCB))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UnityInitWrapperWindows(
        appId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loggingCB: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::System::IntPtr,
                ),
                bool,
                2usize,
            >("ovr_UnityInitWrapperWindows")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UnityInitWrapperWindows", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (appId, loggingCB))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UnityInitWrapperWindowsAsynchronous(
        appId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loggingCB: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::System::IntPtr,
                ),
                u64,
                2usize,
            >("ovr_UnityInitWrapperWindowsAsynchronous")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UnityInitWrapperWindowsAsynchronous", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (appId, loggingCB))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UnityResetTestPlatform() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ovr_UnityResetTestPlatform")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UnityResetTestPlatform", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserAccountAgeCategory_GetAgeCategory(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::AccountAgeCategory> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::Oculus::Platform::AccountAgeCategory,
                1usize,
            >("ovr_UserAccountAgeCategory_GetAgeCategory")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserAccountAgeCategory_GetAgeCategory", 1usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::AccountAgeCategory = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserAgeCategory_Get() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_UserAgeCategory_Get")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserAgeCategory_Get", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserAgeCategory_Report(
        age_category: crate::Oculus::Platform::AppAgeCategory,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Oculus::Platform::AppAgeCategory),
                u64,
                1usize,
            >("ovr_UserAgeCategory_Report")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserAgeCategory_Report", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (age_category)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::UIntPtr),
                crate::System::IntPtr,
                2usize,
            >("ovr_UserArray_GetElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserArray_GetElement", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserArray_GetNextUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_UserArray_GetNextUrl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserArray_GetNextUrl", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserArray_GetNextUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_UserArray_GetNextUrl_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserArray_GetNextUrl_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::UIntPtr,
                1usize,
            >("ovr_UserArray_GetSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserArray_GetSize", 1usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserArray_HasNextPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_UserArray_HasNextPage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserArray_HasNextPage", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserCapabilityArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::UIntPtr),
                crate::System::IntPtr,
                2usize,
            >("ovr_UserCapabilityArray_GetElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserCapabilityArray_GetElement", 2usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserCapabilityArray_GetNextUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_UserCapabilityArray_GetNextUrl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserCapabilityArray_GetNextUrl", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserCapabilityArray_GetNextUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_UserCapabilityArray_GetNextUrl_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserCapabilityArray_GetNextUrl_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserCapabilityArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::UIntPtr,
                1usize,
            >("ovr_UserCapabilityArray_GetSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserCapabilityArray_GetSize", 1usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserCapabilityArray_HasNextPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_UserCapabilityArray_HasNextPage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserCapabilityArray_HasNextPage", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserCapability_GetDescription(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_UserCapability_GetDescription")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserCapability_GetDescription", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserCapability_GetDescription_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_UserCapability_GetDescription_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserCapability_GetDescription_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserCapability_GetIsEnabled(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_UserCapability_GetIsEnabled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserCapability_GetIsEnabled", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserCapability_GetName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_UserCapability_GetName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserCapability_GetName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserCapability_GetName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_UserCapability_GetName_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserCapability_GetName_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserCapability_GetReasonCode(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_UserCapability_GetReasonCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserCapability_GetReasonCode", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserCapability_GetReasonCode_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_UserCapability_GetReasonCode_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserCapability_GetReasonCode_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStoreUpdateResponse_GetSuccess(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_UserDataStoreUpdateResponse_GetSuccess")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserDataStoreUpdateResponse_GetSuccess", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStore_PrivateDeleteEntryByKey(
        userID: u64,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64, quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                2usize,
            >("ovr_UserDataStore_PrivateDeleteEntryByKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserDataStore_PrivateDeleteEntryByKey", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (userID, key)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStore_PrivateDeleteEntryByKey_Native(
        userID: u64,
        key: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64, crate::System::IntPtr),
                u64,
                2usize,
            >("ovr_UserDataStore_PrivateDeleteEntryByKey_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserDataStore_PrivateDeleteEntryByKey_Native", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (userID, key)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStore_PrivateGetEntries(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                u64,
                1usize,
            >("ovr_UserDataStore_PrivateGetEntries")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserDataStore_PrivateGetEntries", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (userID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStore_PrivateGetEntryByKey(
        userID: u64,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64, quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                2usize,
            >("ovr_UserDataStore_PrivateGetEntryByKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserDataStore_PrivateGetEntryByKey", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (userID, key)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStore_PrivateGetEntryByKey_Native(
        userID: u64,
        key: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64, crate::System::IntPtr),
                u64,
                2usize,
            >("ovr_UserDataStore_PrivateGetEntryByKey_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserDataStore_PrivateGetEntryByKey_Native", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (userID, key)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStore_PrivateWriteEntry(
        userID: u64,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    u64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                u64,
                3usize,
            >("ovr_UserDataStore_PrivateWriteEntry")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserDataStore_PrivateWriteEntry", 3usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (userID, key, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStore_PrivateWriteEntry_Native(
        userID: u64,
        key: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64, crate::System::IntPtr, crate::System::IntPtr),
                u64,
                3usize,
            >("ovr_UserDataStore_PrivateWriteEntry_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserDataStore_PrivateWriteEntry_Native", 3usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (userID, key, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStore_PublicDeleteEntryByKey(
        userID: u64,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64, quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                2usize,
            >("ovr_UserDataStore_PublicDeleteEntryByKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserDataStore_PublicDeleteEntryByKey", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (userID, key)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStore_PublicDeleteEntryByKey_Native(
        userID: u64,
        key: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64, crate::System::IntPtr),
                u64,
                2usize,
            >("ovr_UserDataStore_PublicDeleteEntryByKey_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserDataStore_PublicDeleteEntryByKey_Native", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (userID, key)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStore_PublicGetEntries(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                u64,
                1usize,
            >("ovr_UserDataStore_PublicGetEntries")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserDataStore_PublicGetEntries", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (userID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStore_PublicGetEntryByKey(
        userID: u64,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64, quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                2usize,
            >("ovr_UserDataStore_PublicGetEntryByKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserDataStore_PublicGetEntryByKey", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (userID, key)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStore_PublicGetEntryByKey_Native(
        userID: u64,
        key: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64, crate::System::IntPtr),
                u64,
                2usize,
            >("ovr_UserDataStore_PublicGetEntryByKey_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserDataStore_PublicGetEntryByKey_Native", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (userID, key)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStore_PublicWriteEntry(
        userID: u64,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    u64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                u64,
                3usize,
            >("ovr_UserDataStore_PublicWriteEntry")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserDataStore_PublicWriteEntry", 3usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (userID, key, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStore_PublicWriteEntry_Native(
        userID: u64,
        key: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64, crate::System::IntPtr, crate::System::IntPtr),
                u64,
                3usize,
            >("ovr_UserDataStore_PublicWriteEntry_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserDataStore_PublicWriteEntry_Native", 3usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (userID, key, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserOptions_AddServiceProvider(
        handle: crate::System::IntPtr,
        value: crate::Oculus::Platform::ServiceProvider,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::Oculus::Platform::ServiceProvider),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_UserOptions_AddServiceProvider")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserOptions_AddServiceProvider", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserOptions_ClearServiceProviders(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_UserOptions_ClearServiceProviders")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserOptions_ClearServiceProviders", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserOptions_Create() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::System::IntPtr,
                0usize,
            >("ovr_UserOptions_Create")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserOptions_Create", 0usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserOptions_Destroy(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_UserOptions_Destroy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserOptions_Destroy", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserOptions_SetMaxUsers(
        handle: crate::System::IntPtr,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, u32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_UserOptions_SetMaxUsers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserOptions_SetMaxUsers", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserOptions_SetTimeWindow(
        handle: crate::System::IntPtr,
        value: crate::Oculus::Platform::TimeWindow,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::Oculus::Platform::TimeWindow),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_UserOptions_SetTimeWindow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserOptions_SetTimeWindow", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserProof_GetNonce(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_UserProof_GetNonce")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserProof_GetNonce", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserProof_GetNonce_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_UserProof_GetNonce_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserProof_GetNonce_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserReportID_GetDidCancel(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("ovr_UserReportID_GetDidCancel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserReportID_GetDidCancel", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserReportID_GetID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_UserReportID_GetID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_UserReportID_GetID", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_CancelRecordingForReportFlow(
        recordingUUID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ovr_User_CancelRecordingForReportFlow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_CancelRecordingForReportFlow", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (recordingUUID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_CancelRecordingForReportFlow_Native(
        recordingUUID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_User_CancelRecordingForReportFlow_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_CancelRecordingForReportFlow_Native", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (recordingUUID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_Get(userID: u64) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("ovr_User_Get")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_Get", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (userID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetAccessToken() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_User_GetAccessToken")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetAccessToken", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetBlockedUsers() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_User_GetBlockedUsers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetBlockedUsers", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetDisplayName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_User_GetDisplayName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetDisplayName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetDisplayName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_User_GetDisplayName_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetDisplayName_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(crate::System::IntPtr), u64, 1usize>("ovr_User_GetID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetID", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetImageUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_User_GetImageUrl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetImageUrl", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetImageUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_User_GetImageUrl_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetImageUrl_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetLinkedAccounts(
        userOptions: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                u64,
                1usize,
            >("ovr_User_GetLinkedAccounts")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetLinkedAccounts", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (userOptions)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetLoggedInUser() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_User_GetLoggedInUser")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetLoggedInUser", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetLoggedInUserFriends() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_User_GetLoggedInUserFriends")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetLoggedInUserFriends", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetOculusID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_User_GetOculusID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetOculusID", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetOculusID_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_User_GetOculusID_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetOculusID_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetOrgScopedID(userID: u64) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("ovr_User_GetOrgScopedID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetOrgScopedID", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (userID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetPresence(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_User_GetPresence")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetPresence", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetPresenceDeeplinkMessage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_User_GetPresenceDeeplinkMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetPresenceDeeplinkMessage", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetPresenceDeeplinkMessage_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_User_GetPresenceDeeplinkMessage_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetPresenceDeeplinkMessage_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetPresenceDestinationApiName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_User_GetPresenceDestinationApiName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetPresenceDestinationApiName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetPresenceDestinationApiName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_User_GetPresenceDestinationApiName_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetPresenceDestinationApiName_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetPresenceLobbySessionId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_User_GetPresenceLobbySessionId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetPresenceLobbySessionId", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetPresenceLobbySessionId_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_User_GetPresenceLobbySessionId_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetPresenceLobbySessionId_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetPresenceMatchSessionId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_User_GetPresenceMatchSessionId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetPresenceMatchSessionId", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetPresenceMatchSessionId_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_User_GetPresenceMatchSessionId_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetPresenceMatchSessionId_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetPresenceStatus(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::UserPresenceStatus> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::Oculus::Platform::UserPresenceStatus,
                1usize,
            >("ovr_User_GetPresenceStatus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetPresenceStatus", 1usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::UserPresenceStatus = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetPresence_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_User_GetPresence_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetPresence_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetSdkAccounts() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_User_GetSdkAccounts")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetSdkAccounts", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetSmallImageUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ovr_User_GetSmallImageUrl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetSmallImageUrl", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetSmallImageUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("ovr_User_GetSmallImageUrl_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetSmallImageUrl_Native", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetUserCapabilities() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_User_GetUserCapabilities")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetUserCapabilities", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetUserProof() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_User_GetUserProof")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_GetUserProof", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_LaunchBlockFlow(userID: u64) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("ovr_User_LaunchBlockFlow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_LaunchBlockFlow", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (userID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_LaunchFriendRequestFlow(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("ovr_User_LaunchFriendRequestFlow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_LaunchFriendRequestFlow", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (userID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_LaunchReportFlow(userID: u64) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("ovr_User_LaunchReportFlow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_LaunchReportFlow", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (userID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_LaunchReportFlow2(
        optionalUserID: u64,
        abuseReportOptions: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64, crate::System::IntPtr),
                u64,
                2usize,
            >("ovr_User_LaunchReportFlow2")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_LaunchReportFlow2", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (optionalUserID, abuseReportOptions))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_LaunchUnblockFlow(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("ovr_User_LaunchUnblockFlow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_LaunchUnblockFlow", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (userID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_NewEntitledTestUser() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_User_NewEntitledTestUser")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_NewEntitledTestUser", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_NewTestUser() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_User_NewTestUser")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_NewTestUser", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_NewTestUserFriends() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_User_NewTestUserFriends")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_NewTestUserFriends", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_StartRecordingForReportFlow() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                u64,
                0usize,
            >("ovr_User_StartRecordingForReportFlow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_StartRecordingForReportFlow", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_StopRecordingAndLaunchReportFlow(
        optionalUserID: u64,
        optionalRecordingUUID: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64, quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                2usize,
            >("ovr_User_StopRecordingAndLaunchReportFlow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_StopRecordingAndLaunchReportFlow", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (optionalUserID, optionalRecordingUUID))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_StopRecordingAndLaunchReportFlow2(
        optionalUserID: u64,
        optionalRecordingUUID: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        abuseReportOptions: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    u64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::System::IntPtr,
                ),
                u64,
                3usize,
            >("ovr_User_StopRecordingAndLaunchReportFlow2")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_StopRecordingAndLaunchReportFlow2", 3usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (optionalUserID, optionalRecordingUUID, abuseReportOptions),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_StopRecordingAndLaunchReportFlow2_Native(
        optionalUserID: u64,
        optionalRecordingUUID: crate::System::IntPtr,
        abuseReportOptions: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64, crate::System::IntPtr, crate::System::IntPtr),
                u64,
                3usize,
            >("ovr_User_StopRecordingAndLaunchReportFlow2_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_StopRecordingAndLaunchReportFlow2_Native", 3usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (optionalUserID, optionalRecordingUUID, abuseReportOptions),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_StopRecordingAndLaunchReportFlow_Native(
        optionalUserID: u64,
        optionalRecordingUUID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64, crate::System::IntPtr),
                u64,
                2usize,
            >("ovr_User_StopRecordingAndLaunchReportFlow_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_StopRecordingAndLaunchReportFlow_Native", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (optionalUserID, optionalRecordingUUID))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_TestUserCreateDeviceManifest(
        deviceID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        appIDs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        numAppIDs: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
                    i32,
                ),
                u64,
                3usize,
            >("ovr_User_TestUserCreateDeviceManifest")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_TestUserCreateDeviceManifest", 3usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (deviceID, appIDs, numAppIDs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_TestUserCreateDeviceManifest_Native(
        deviceID: crate::System::IntPtr,
        appIDs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        numAppIDs: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
                    i32,
                ),
                u64,
                3usize,
            >("ovr_User_TestUserCreateDeviceManifest_Native")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_User_TestUserCreateDeviceManifest_Native", 3usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (deviceID, appIDs, numAppIDs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_VoipDecoder_Decode_UIntPtr1(
        obj: crate::System::IntPtr,
        compressedData: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        compressedSize: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    crate::System::UIntPtr,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("ovr_VoipDecoder_Decode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_VoipDecoder_Decode", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (obj, compressedData, compressedSize))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_VoipDecoder_Decode_u64_0(
        obj: crate::System::IntPtr,
        compressedData: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        compressedSize: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    u64,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("ovr_VoipDecoder_Decode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_VoipDecoder_Decode", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (obj, compressedData, compressedSize))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_VoipDecoder_GetDecodedPCM(
        obj: crate::System::IntPtr,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        outputBufferSize: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    crate::System::UIntPtr,
                ),
                crate::System::UIntPtr,
                3usize,
            >("ovr_VoipDecoder_GetDecodedPCM")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_VoipDecoder_GetDecodedPCM", 3usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj, outputBuffer, outputBufferSize))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_VoipEncoder_AddPCM(
        obj: crate::System::IntPtr,
        inputData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        inputSize: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    u32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("ovr_VoipEncoder_AddPCM")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_VoipEncoder_AddPCM", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (obj, inputData, inputSize))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_VoipEncoder_GetCompressedData(
        obj: crate::System::IntPtr,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        intputSize: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    crate::System::UIntPtr,
                ),
                crate::System::UIntPtr,
                3usize,
            >("ovr_VoipEncoder_GetCompressedData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_VoipEncoder_GetCompressedData", 3usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj, outputBuffer, intputSize))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_VoipEncoder_GetCompressedDataSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::UIntPtr,
                1usize,
            >("ovr_VoipEncoder_GetCompressedDataSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_VoipEncoder_GetCompressedDataSize", 1usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_VoipOptions_Create() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::System::IntPtr,
                0usize,
            >("ovr_VoipOptions_Create")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_VoipOptions_Create", 0usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_VoipOptions_Destroy(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_VoipOptions_Destroy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_VoipOptions_Destroy", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_VoipOptions_SetBitrateForNewConnections(
        handle: crate::System::IntPtr,
        value: crate::Oculus::Platform::VoipBitrate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::Oculus::Platform::VoipBitrate),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_VoipOptions_SetBitrateForNewConnections")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_VoipOptions_SetBitrateForNewConnections", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_VoipOptions_SetCreateNewConnectionUseDtx(
        handle: crate::System::IntPtr,
        value: crate::Oculus::Platform::VoipDtxState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::Oculus::Platform::VoipDtxState),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_VoipOptions_SetCreateNewConnectionUseDtx")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_VoipOptions_SetCreateNewConnectionUseDtx", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_Accept(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_Voip_Accept")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_Accept", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (userID))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_CreateDecoder() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::System::IntPtr,
                0usize,
            >("ovr_Voip_CreateDecoder")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_CreateDecoder", 0usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_CreateEncoder() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::System::IntPtr,
                0usize,
            >("ovr_Voip_CreateEncoder")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_CreateEncoder", 0usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_DestroyDecoder(
        decoder: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_Voip_DestroyDecoder")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_DestroyDecoder", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (decoder))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_DestroyEncoder(
        encoder: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_Voip_DestroyEncoder")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_DestroyEncoder", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (encoder))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_GetIsConnectionUsingDtx(
        peerID: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::VoipDtxState> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                crate::Oculus::Platform::VoipDtxState,
                1usize,
            >("ovr_Voip_GetIsConnectionUsingDtx")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_GetIsConnectionUsingDtx", 1usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::VoipDtxState = unsafe {
            method.invoke_unchecked((), (peerID))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_GetLocalBitrate(
        peerID: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::VoipBitrate> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                crate::Oculus::Platform::VoipBitrate,
                1usize,
            >("ovr_Voip_GetLocalBitrate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_GetLocalBitrate", 1usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::VoipBitrate = unsafe {
            method.invoke_unchecked((), (peerID))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_GetMicrophoneAvailability() -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), u64, 0usize>("ovr_Voip_GetMicrophoneAvailability")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_GetMicrophoneAvailability", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_GetOutputBufferMaxSize() -> quest_hook::libil2cpp::Result<
        crate::System::UIntPtr,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::System::UIntPtr,
                0usize,
            >("ovr_Voip_GetOutputBufferMaxSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_GetOutputBufferMaxSize", 0usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_GetPCM(
        senderID: u64,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
        outputBufferNumElements: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    u64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
                    crate::System::UIntPtr,
                ),
                crate::System::UIntPtr,
                3usize,
            >("ovr_Voip_GetPCM")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_GetPCM", 3usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method
                .invoke_unchecked((), (senderID, outputBuffer, outputBufferNumElements))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_GetPCMFloat(
        senderID: u64,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        outputBufferNumElements: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    u64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    crate::System::UIntPtr,
                ),
                crate::System::UIntPtr,
                3usize,
            >("ovr_Voip_GetPCMFloat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_GetPCMFloat", 3usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method
                .invoke_unchecked((), (senderID, outputBuffer, outputBufferNumElements))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_GetPCMSize(
        senderID: u64,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                crate::System::UIntPtr,
                1usize,
            >("ovr_Voip_GetPCMSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_GetPCMSize", 1usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method.invoke_unchecked((), (senderID))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_GetPCMWithTimestamp(
        senderID: u64,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
        outputBufferNumElements: crate::System::UIntPtr,
        timestamp: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    u64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
                    crate::System::UIntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                crate::System::UIntPtr,
                4usize,
            >("ovr_Voip_GetPCMWithTimestamp")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_GetPCMWithTimestamp", 4usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (senderID, outputBuffer, outputBufferNumElements, timestamp),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_GetPCMWithTimestampFloat(
        senderID: u64,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        outputBufferNumElements: crate::System::UIntPtr,
        timestamp: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    u64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    crate::System::UIntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                crate::System::UIntPtr,
                4usize,
            >("ovr_Voip_GetPCMWithTimestampFloat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_GetPCMWithTimestampFloat", 4usize
                )
            });
        let __cordl_ret: crate::System::UIntPtr = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (senderID, outputBuffer, outputBufferNumElements, timestamp),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_GetRemoteBitrate(
        peerID: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::VoipBitrate> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                crate::Oculus::Platform::VoipBitrate,
                1usize,
            >("ovr_Voip_GetRemoteBitrate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_GetRemoteBitrate", 1usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::VoipBitrate = unsafe {
            method.invoke_unchecked((), (peerID))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_GetSyncTimestamp(userID: u64) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u32, 1usize>("ovr_Voip_GetSyncTimestamp")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_GetSyncTimestamp", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (userID)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_GetSyncTimestampDifference(
        lhs: u32,
        rhs: u32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u32, u32),
                i64,
                2usize,
            >("ovr_Voip_GetSyncTimestampDifference")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_GetSyncTimestampDifference", 2usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (lhs, rhs)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_GetSystemVoipMicrophoneMuted() -> quest_hook::libil2cpp::Result<
        crate::Oculus::Platform::VoipMuteState,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::Oculus::Platform::VoipMuteState,
                0usize,
            >("ovr_Voip_GetSystemVoipMicrophoneMuted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_GetSystemVoipMicrophoneMuted", 0usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::VoipMuteState = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_GetSystemVoipStatus() -> quest_hook::libil2cpp::Result<
        crate::Oculus::Platform::SystemVoipStatus,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::Oculus::Platform::SystemVoipStatus,
                0usize,
            >("ovr_Voip_GetSystemVoipStatus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_GetSystemVoipStatus", 0usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::SystemVoipStatus = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_ReportAppVoipSessions(
        sessionIDs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>),
                u64,
                1usize,
            >("ovr_Voip_ReportAppVoipSessions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_ReportAppVoipSessions", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (sessionIDs)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_SetMicrophoneFilterCallback(
        cb: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::CAPI_FilterCallback>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::CAPI_FilterCallback,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_Voip_SetMicrophoneFilterCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_SetMicrophoneFilterCallback", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (cb))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_SetMicrophoneFilterCallbackWithFixedSizeBuffer(
        cb: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::CAPI_FilterCallback>,
        bufferSizeElements: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Oculus::Platform::CAPI_FilterCallback,
                    >,
                    crate::System::UIntPtr,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ovr_Voip_SetMicrophoneFilterCallbackWithFixedSizeBuffer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_SetMicrophoneFilterCallbackWithFixedSizeBuffer",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (cb, bufferSizeElements))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_SetMicrophoneMuted(
        state: crate::Oculus::Platform::VoipMuteState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Oculus::Platform::VoipMuteState),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_Voip_SetMicrophoneMuted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_SetMicrophoneMuted", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (state))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_SetNewConnectionOptions(
        voipOptions: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_Voip_SetNewConnectionOptions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_SetNewConnectionOptions", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (voipOptions))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_SetOutputSampleRate(
        rate: crate::Oculus::Platform::VoipSampleRate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Oculus::Platform::VoipSampleRate),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_Voip_SetOutputSampleRate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_SetOutputSampleRate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (rate))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_SetSystemVoipMicrophoneMuted(
        muted: crate::Oculus::Platform::VoipMuteState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Oculus::Platform::VoipMuteState),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_Voip_SetSystemVoipMicrophoneMuted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_SetSystemVoipMicrophoneMuted", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (muted))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_SetSystemVoipPassthrough(
        passthrough: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_Voip_SetSystemVoipPassthrough")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_SetSystemVoipPassthrough", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (passthrough))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_SetSystemVoipSuppressed(
        suppressed: bool,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (bool),
                u64,
                1usize,
            >("ovr_Voip_SetSystemVoipSuppressed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_SetSystemVoipSuppressed", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (suppressed)) };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_Start(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_Voip_Start")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_Start", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (userID))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_Stop(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ovr_Voip_Stop")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ovr_Voip_Stop", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (userID))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+CAPI")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::CAPI {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Oculus+Platform+CAPI+FilterCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct CAPI_FilterCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Oculus+Platform+CAPI+FilterCallback")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Oculus::Platform::CAPI_FilterCallback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "CAPI/FilterCallback";
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
#[cfg(feature = "Oculus+Platform+CAPI+FilterCallback")]
impl std::ops::Deref for crate::Oculus::Platform::CAPI_FilterCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+CAPI+FilterCallback")]
impl std::ops::DerefMut for crate::Oculus::Platform::CAPI_FilterCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+CAPI+FilterCallback")]
impl crate::Oculus::Platform::CAPI_FilterCallback {
    pub fn BeginInvoke(
        &mut self,
        pcmData: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
        >,
        pcmDataLength: crate::System::UIntPtr,
        frequency: i32,
        numChannels: i32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<i16>,
                        >,
                    >,
                    crate::System::UIntPtr,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                6usize,
            >("BeginInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "BeginInvoke", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (pcmData, pcmDataLength, frequency, numChannels, callback, object),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("EndInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EndInvoke", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (result))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pcmData: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
        >,
        pcmDataLength: crate::System::UIntPtr,
        frequency: i32,
        numChannels: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<i16>,
                        >,
                    >,
                    crate::System::UIntPtr,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Invoke", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(self, (pcmData, pcmDataLength, frequency, numChannels))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::IntPtr,
                ),
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
            method.invoke_unchecked(self, (object, method))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+CAPI+FilterCallback")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::CAPI_FilterCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Oculus+Platform+CAPI+OculusInitParams")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CAPI_OculusInitParams {
    pub sType: i32,
    pub email: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub appId: u64,
    pub uriPrefixOverride: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "Oculus+Platform+CAPI+OculusInitParams")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Oculus::Platform::CAPI_OculusInitParams {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "CAPI/OculusInitParams";
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
#[cfg(feature = "Oculus+Platform+CAPI+OculusInitParams")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Oculus::Platform::CAPI_OculusInitParams {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Oculus+Platform+CAPI+OculusInitParams")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Oculus::Platform::CAPI_OculusInitParams {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "Oculus+Platform+CAPI+OculusInitParams")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Oculus::Platform::CAPI_OculusInitParams {
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
#[cfg(feature = "Oculus+Platform+CAPI+OculusInitParams")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Oculus::Platform::CAPI_OculusInitParams {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "Oculus+Platform+CAPI+OculusInitParams")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Oculus::Platform::CAPI_OculusInitParams {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Oculus+Platform+CAPI+OculusInitParams")]
impl crate::Oculus::Platform::CAPI_OculusInitParams {}
#[cfg(feature = "Oculus+Platform+CAPI+ovrKeyValuePair")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CAPI_ovrKeyValuePair {
    pub key_: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub valueType_: crate::Oculus::Platform::KeyValuePairType,
    pub stringValue_: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub intValue_: i32,
    pub doubleValue_: f64,
}
#[cfg(feature = "Oculus+Platform+CAPI+ovrKeyValuePair")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Oculus::Platform::CAPI_ovrKeyValuePair {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "CAPI/ovrKeyValuePair";
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
#[cfg(feature = "Oculus+Platform+CAPI+ovrKeyValuePair")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Oculus::Platform::CAPI_ovrKeyValuePair {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Oculus+Platform+CAPI+ovrKeyValuePair")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Oculus::Platform::CAPI_ovrKeyValuePair {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "Oculus+Platform+CAPI+ovrKeyValuePair")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Oculus::Platform::CAPI_ovrKeyValuePair {
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
#[cfg(feature = "Oculus+Platform+CAPI+ovrKeyValuePair")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Oculus::Platform::CAPI_ovrKeyValuePair {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "Oculus+Platform+CAPI+ovrKeyValuePair")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Oculus::Platform::CAPI_ovrKeyValuePair {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Oculus+Platform+CAPI+ovrKeyValuePair")]
impl crate::Oculus::Platform::CAPI_ovrKeyValuePair {
    pub fn _ctor_Il2CppString0(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
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
            method.invoke_unchecked(self, (key, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f64_2(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, f64),
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
            method.invoke_unchecked(self, (key, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_1(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, i32),
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
            method.invoke_unchecked(self, (key, value))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+CAPI+ovrNetSyncVec3")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CAPI_ovrNetSyncVec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[cfg(feature = "Oculus+Platform+CAPI+ovrNetSyncVec3")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Oculus::Platform::CAPI_ovrNetSyncVec3 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "CAPI/ovrNetSyncVec3";
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
#[cfg(feature = "Oculus+Platform+CAPI+ovrNetSyncVec3")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Oculus::Platform::CAPI_ovrNetSyncVec3 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Oculus+Platform+CAPI+ovrNetSyncVec3")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Oculus::Platform::CAPI_ovrNetSyncVec3 {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "Oculus+Platform+CAPI+ovrNetSyncVec3")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Oculus::Platform::CAPI_ovrNetSyncVec3 {
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
#[cfg(feature = "Oculus+Platform+CAPI+ovrNetSyncVec3")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Oculus::Platform::CAPI_ovrNetSyncVec3 {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "Oculus+Platform+CAPI+ovrNetSyncVec3")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Oculus::Platform::CAPI_ovrNetSyncVec3 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Oculus+Platform+CAPI+ovrNetSyncVec3")]
impl crate::Oculus::Platform::CAPI_ovrNetSyncVec3 {}
