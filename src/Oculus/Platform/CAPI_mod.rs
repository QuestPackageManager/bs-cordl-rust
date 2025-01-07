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
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ArrayOfStructsToIntPtr", (ar))?;
        Ok(__cordl_ret.into())
    }
    pub fn BlobFromNative(
        _cordl_size: u32,
        pointer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BlobFromNative", (_cordl_size, pointer))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DataStoreFromNative", (pointer))?;
        Ok(__cordl_ret.into())
    }
    pub fn DateTimeFromNative(
        seconds_since_the_one_true_epoch: u64,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DateTimeFromNative", (seconds_since_the_one_true_epoch))?;
        Ok(__cordl_ret.into())
    }
    pub fn DateTimeToNative(
        dt: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DateTimeToNative", (dt))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::Oculus::Platform::CAPI_ovrKeyValuePair,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DictionaryToOVRKeyValuePairs", (dict))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::Oculus::Platform::CAPI_ovrKeyValuePair,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DictionaryToOVRKeyValuePairs", (dict))?;
        Ok(__cordl_ret.into())
    }
    pub fn FiledataFromNative(
        _cordl_size: u32,
        pointer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FiledataFromNative", (_cordl_size, pointer))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNativeStringLengthNotIncludingNullTerminator(
        pointer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNativeStringLengthNotIncludingNullTerminator", (pointer))?;
        Ok(__cordl_ret.into())
    }
    pub fn IntPtrToByteArray(
        data: crate::System::IntPtr,
        _cordl_size: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IntPtrToByteArray", (data, _cordl_size))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogNewEvent", (eventName, values))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogNewUnifiedEvent", (eventName, values))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StringFromNative", (pointer))?;
        Ok(__cordl_ret.into())
    }
    pub fn StringToNative(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StringToNative", (s))?;
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
    pub fn ovr_AbuseReportOptions_Create() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AbuseReportOptions_Create", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AbuseReportOptions_Destroy(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AbuseReportOptions_Destroy", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AbuseReportOptions_SetPreventPeopleChooser(
        handle: crate::System::IntPtr,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AbuseReportOptions_SetPreventPeopleChooser", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AbuseReportOptions_SetReportType(
        handle: crate::System::IntPtr,
        value: crate::Oculus::Platform::AbuseReportType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AbuseReportOptions_SetReportType", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AbuseReportRecording_GetRecordingUuid(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AbuseReportRecording_GetRecordingUuid", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AbuseReportRecording_GetRecordingUuid_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AbuseReportRecording_GetRecordingUuid_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AbuseReport_LaunchAdvancedReportFlow(
        content_id: u64,
        abuse_report_options: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_AbuseReport_LaunchAdvancedReportFlow",
                (content_id, abuse_report_options),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AbuseReport_ReportRequestHandled(
        response: crate::Oculus::Platform::ReportRequestResponse,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AbuseReport_ReportRequestHandled", (response))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementDefinitionArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AchievementDefinitionArray_GetElement", (obj, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementDefinitionArray_GetNextUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AchievementDefinitionArray_GetNextUrl", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementDefinitionArray_GetNextUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AchievementDefinitionArray_GetNextUrl_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementDefinitionArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AchievementDefinitionArray_GetSize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementDefinitionArray_HasNextPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AchievementDefinitionArray_HasNextPage", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementDefinition_GetBitfieldLength(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AchievementDefinition_GetBitfieldLength", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementDefinition_GetName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AchievementDefinition_GetName", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementDefinition_GetName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AchievementDefinition_GetName_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementDefinition_GetTarget(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AchievementDefinition_GetTarget", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementDefinition_GetType(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::AchievementType> {
        let __cordl_ret: crate::Oculus::Platform::AchievementType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AchievementDefinition_GetType", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementProgressArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AchievementProgressArray_GetElement", (obj, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementProgressArray_GetNextUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AchievementProgressArray_GetNextUrl", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementProgressArray_GetNextUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AchievementProgressArray_GetNextUrl_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementProgressArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AchievementProgressArray_GetSize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementProgressArray_HasNextPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AchievementProgressArray_HasNextPage", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementProgress_GetBitfield(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AchievementProgress_GetBitfield", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementProgress_GetBitfield_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AchievementProgress_GetBitfield_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementProgress_GetCount(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AchievementProgress_GetCount", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementProgress_GetIsUnlocked(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AchievementProgress_GetIsUnlocked", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementProgress_GetName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AchievementProgress_GetName", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementProgress_GetName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AchievementProgress_GetName_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementProgress_GetUnlockTime(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AchievementProgress_GetUnlockTime", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementProgress_GetUnlockTime_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AchievementProgress_GetUnlockTime_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementUpdate_GetJustUnlocked(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AchievementUpdate_GetJustUnlocked", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementUpdate_GetName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AchievementUpdate_GetName", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AchievementUpdate_GetName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AchievementUpdate_GetName_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Achievements_AddCount(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        count: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Achievements_AddCount", (name, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Achievements_AddCount_Native(
        name: crate::System::IntPtr,
        count: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Achievements_AddCount_Native", (name, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Achievements_AddFields(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fields: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Achievements_AddFields", (name, fields))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Achievements_AddFields_Native(
        name: crate::System::IntPtr,
        fields: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Achievements_AddFields_Native", (name, fields))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Achievements_GetAllDefinitions() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Achievements_GetAllDefinitions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Achievements_GetAllProgress() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Achievements_GetAllProgress", ())?;
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
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Achievements_GetDefinitionsByName", (names, count))?;
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
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Achievements_GetProgressByName", (names, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Achievements_Unlock(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Achievements_Unlock", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Achievements_Unlock_Native(
        name: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Achievements_Unlock_Native", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AdvancedAbuseReportOptions_AddSuggestedUser(
        handle: crate::System::IntPtr,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AdvancedAbuseReportOptions_AddSuggestedUser", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AdvancedAbuseReportOptions_ClearDeveloperDefinedContext(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_AdvancedAbuseReportOptions_ClearDeveloperDefinedContext",
                (handle),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AdvancedAbuseReportOptions_ClearSuggestedUsers(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AdvancedAbuseReportOptions_ClearSuggestedUsers", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AdvancedAbuseReportOptions_Create() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AdvancedAbuseReportOptions_Create", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AdvancedAbuseReportOptions_Destroy(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AdvancedAbuseReportOptions_Destroy", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AdvancedAbuseReportOptions_SetDeveloperDefinedContextString(
        handle: crate::System::IntPtr,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_AdvancedAbuseReportOptions_SetDeveloperDefinedContextString",
                (handle, key, value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AdvancedAbuseReportOptions_SetDeveloperDefinedContextString_Native(
        handle: crate::System::IntPtr,
        key: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_AdvancedAbuseReportOptions_SetDeveloperDefinedContextString_Native",
                (handle, key, value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AdvancedAbuseReportOptions_SetObjectType(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AdvancedAbuseReportOptions_SetObjectType", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AdvancedAbuseReportOptions_SetObjectType_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_AdvancedAbuseReportOptions_SetObjectType_Native",
                (handle, value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AdvancedAbuseReportOptions_SetReportType(
        handle: crate::System::IntPtr,
        value: crate::Oculus::Platform::AbuseReportType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AdvancedAbuseReportOptions_SetReportType", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AdvancedAbuseReportOptions_SetVideoMode(
        handle: crate::System::IntPtr,
        value: crate::Oculus::Platform::AbuseReportVideoMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AdvancedAbuseReportOptions_SetVideoMode", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AppDownloadProgressResult_GetDownloadBytes(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AppDownloadProgressResult_GetDownloadBytes", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AppDownloadProgressResult_GetDownloadedBytes(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AppDownloadProgressResult_GetDownloadedBytes", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AppDownloadProgressResult_GetStatusCode(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::AppStatus> {
        let __cordl_ret: crate::Oculus::Platform::AppStatus = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AppDownloadProgressResult_GetStatusCode", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AppDownloadResult_GetTimestamp(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AppDownloadResult_GetTimestamp", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationInviteArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationInviteArray_GetElement", (obj, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationInviteArray_GetNextUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationInviteArray_GetNextUrl", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationInviteArray_GetNextUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationInviteArray_GetNextUrl_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationInviteArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationInviteArray_GetSize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationInviteArray_HasNextPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationInviteArray_HasNextPage", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationInvite_GetDestination(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationInvite_GetDestination", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationInvite_GetID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationInvite_GetID", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationInvite_GetIsActive(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationInvite_GetIsActive", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationInvite_GetLobbySessionId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationInvite_GetLobbySessionId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationInvite_GetLobbySessionId_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationInvite_GetLobbySessionId_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationInvite_GetMatchSessionId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationInvite_GetMatchSessionId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationInvite_GetMatchSessionId_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationInvite_GetMatchSessionId_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationInvite_GetRecipient(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationInvite_GetRecipient", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationLifecycle_GetLaunchDetails() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationLifecycle_GetLaunchDetails", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationLifecycle_GetRegisteredPIDs() -> quest_hook::libil2cpp::Result<
        u64,
    > {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationLifecycle_GetRegisteredPIDs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationLifecycle_GetSessionKey() -> quest_hook::libil2cpp::Result<
        u64,
    > {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationLifecycle_GetSessionKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationLifecycle_LogDeeplinkResult(
        trackingID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: crate::Oculus::Platform::LaunchResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationLifecycle_LogDeeplinkResult", (trackingID, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationLifecycle_LogDeeplinkResult_Native(
        trackingID: crate::System::IntPtr,
        result: crate::Oculus::Platform::LaunchResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_ApplicationLifecycle_LogDeeplinkResult_Native",
                (trackingID, result),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationLifecycle_RegisterSessionKey(
        sessionKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationLifecycle_RegisterSessionKey", (sessionKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationLifecycle_RegisterSessionKey_Native(
        sessionKey: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationLifecycle_RegisterSessionKey_Native", (sessionKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationOptions_Create() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationOptions_Create", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationOptions_Destroy(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationOptions_Destroy", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationOptions_SetDeeplinkMessage(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationOptions_SetDeeplinkMessage", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationOptions_SetDeeplinkMessage_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_ApplicationOptions_SetDeeplinkMessage_Native",
                (handle, value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationOptions_SetDestinationApiName(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationOptions_SetDestinationApiName", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationOptions_SetDestinationApiName_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_ApplicationOptions_SetDestinationApiName_Native",
                (handle, value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationOptions_SetLobbySessionId(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationOptions_SetLobbySessionId", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationOptions_SetLobbySessionId_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationOptions_SetLobbySessionId_Native", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationOptions_SetMatchSessionId(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationOptions_SetMatchSessionId", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationOptions_SetMatchSessionId_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationOptions_SetMatchSessionId_Native", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationOptions_SetRoomId(
        handle: crate::System::IntPtr,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationOptions_SetRoomId", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationVersion_GetCurrentCode(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationVersion_GetCurrentCode", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationVersion_GetCurrentName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationVersion_GetCurrentName", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationVersion_GetCurrentName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationVersion_GetCurrentName_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationVersion_GetLatestCode(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationVersion_GetLatestCode", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationVersion_GetLatestName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationVersion_GetLatestName", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationVersion_GetLatestName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationVersion_GetLatestName_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationVersion_GetReleaseDate(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationVersion_GetReleaseDate", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationVersion_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationVersion_GetSize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ApplicationVersion_GetSize_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ApplicationVersion_GetSize_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Application_CancelAppDownload() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Application_CancelAppDownload", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Application_CheckAppDownloadProgress() -> quest_hook::libil2cpp::Result<
        u64,
    > {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Application_CheckAppDownloadProgress", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Application_GetID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Application_GetID", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Application_GetInstalledApplications() -> quest_hook::libil2cpp::Result<
        u64,
    > {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Application_GetInstalledApplications", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Application_GetVersion() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Application_GetVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Application_InstallAppUpdateAndRelaunch(
        deeplink_options: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Application_InstallAppUpdateAndRelaunch", (deeplink_options))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Application_LaunchOtherApp(
        appID: u64,
        deeplink_options: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Application_LaunchOtherApp", (appID, deeplink_options))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Application_StartAppDownload() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Application_StartAppDownload", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetDetailsArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetDetailsArray_GetElement", (obj, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetDetailsArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetDetailsArray_GetSize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetDetails_GetAssetId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetDetails_GetAssetId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetDetails_GetAssetType(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetDetails_GetAssetType", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetDetails_GetAssetType_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetDetails_GetAssetType_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetDetails_GetDownloadStatus(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetDetails_GetDownloadStatus", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetDetails_GetDownloadStatus_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetDetails_GetDownloadStatus_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetDetails_GetFilepath(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetDetails_GetFilepath", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetDetails_GetFilepath_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetDetails_GetFilepath_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetDetails_GetIapStatus(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetDetails_GetIapStatus", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetDetails_GetIapStatus_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetDetails_GetIapStatus_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetDetails_GetLanguage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetDetails_GetLanguage", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetDetails_GetMetadata(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetDetails_GetMetadata", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetDetails_GetMetadata_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetDetails_GetMetadata_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDeleteResult_GetAssetFileId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFileDeleteResult_GetAssetFileId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDeleteResult_GetAssetId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFileDeleteResult_GetAssetId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDeleteResult_GetFilepath(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFileDeleteResult_GetFilepath", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDeleteResult_GetFilepath_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFileDeleteResult_GetFilepath_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDeleteResult_GetSuccess(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFileDeleteResult_GetSuccess", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadCancelResult_GetAssetFileId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFileDownloadCancelResult_GetAssetFileId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadCancelResult_GetAssetId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFileDownloadCancelResult_GetAssetId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadCancelResult_GetFilepath(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFileDownloadCancelResult_GetFilepath", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadCancelResult_GetFilepath_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFileDownloadCancelResult_GetFilepath_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadCancelResult_GetSuccess(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFileDownloadCancelResult_GetSuccess", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadResult_GetAssetId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFileDownloadResult_GetAssetId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadResult_GetFilepath(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFileDownloadResult_GetFilepath", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadResult_GetFilepath_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFileDownloadResult_GetFilepath_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadUpdate_GetAssetFileId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFileDownloadUpdate_GetAssetFileId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadUpdate_GetAssetId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFileDownloadUpdate_GetAssetId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadUpdate_GetBytesTotal(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFileDownloadUpdate_GetBytesTotal", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadUpdate_GetBytesTotalLong(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFileDownloadUpdate_GetBytesTotalLong", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadUpdate_GetBytesTransferred(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFileDownloadUpdate_GetBytesTransferred", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadUpdate_GetBytesTransferredLong(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFileDownloadUpdate_GetBytesTransferredLong", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFileDownloadUpdate_GetCompleted(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFileDownloadUpdate_GetCompleted", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_Delete(assetFileID: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFile_Delete", (assetFileID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_DeleteById(
        assetFileID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFile_DeleteById", (assetFileID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_DeleteByName(
        assetFileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFile_DeleteByName", (assetFileName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_DeleteByName_Native(
        assetFileName: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFile_DeleteByName_Native", (assetFileName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_Download(
        assetFileID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFile_Download", (assetFileID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_DownloadById(
        assetFileID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFile_DownloadById", (assetFileID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_DownloadByName(
        assetFileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFile_DownloadByName", (assetFileName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_DownloadByName_Native(
        assetFileName: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFile_DownloadByName_Native", (assetFileName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_DownloadCancel(
        assetFileID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFile_DownloadCancel", (assetFileID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_DownloadCancelById(
        assetFileID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFile_DownloadCancelById", (assetFileID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_DownloadCancelByName(
        assetFileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFile_DownloadCancelByName", (assetFileName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_DownloadCancelByName_Native(
        assetFileName: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFile_DownloadCancelByName_Native", (assetFileName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_GetList() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFile_GetList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_Status(assetFileID: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFile_Status", (assetFileID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_StatusById(
        assetFileID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFile_StatusById", (assetFileID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_StatusByName(
        assetFileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFile_StatusByName", (assetFileName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AssetFile_StatusByName_Native(
        assetFileName: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AssetFile_StatusByName_Native", (assetFileName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AvatarEditorOptions_Create() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AvatarEditorOptions_Create", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AvatarEditorOptions_Destroy(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AvatarEditorOptions_Destroy", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AvatarEditorOptions_SetSourceOverride(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AvatarEditorOptions_SetSourceOverride", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AvatarEditorOptions_SetSourceOverride_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_AvatarEditorOptions_SetSourceOverride_Native",
                (handle, value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_AvatarEditorResult_GetRequestSent(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_AvatarEditorResult_GetRequestSent", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Avatar_LaunchAvatarEditor(
        options: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Avatar_LaunchAvatarEditor", (options))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Avatar_UpdateMetaData(
        avatarMetaData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        imageFilePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Avatar_UpdateMetaData", (avatarMetaData, imageFilePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Avatar_UpdateMetaData_Native(
        avatarMetaData: crate::System::IntPtr,
        imageFilePath: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_Avatar_UpdateMetaData_Native",
                (avatarMetaData, imageFilePath),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_BlockedUserArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_BlockedUserArray_GetElement", (obj, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_BlockedUserArray_GetNextUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_BlockedUserArray_GetNextUrl", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_BlockedUserArray_GetNextUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_BlockedUserArray_GetNextUrl_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_BlockedUserArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_BlockedUserArray_GetSize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_BlockedUserArray_HasNextPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_BlockedUserArray_HasNextPage", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_BlockedUser_GetId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_BlockedUser_GetId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeArray_GetElement", (obj, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeArray_GetNextUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeArray_GetNextUrl", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeArray_GetNextUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeArray_GetNextUrl_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeArray_GetPreviousUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeArray_GetPreviousUrl", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeArray_GetPreviousUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeArray_GetPreviousUrl_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeArray_GetSize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeArray_GetTotalCount(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeArray_GetTotalCount", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeArray_HasNextPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeArray_HasNextPage", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeArray_HasPreviousPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeArray_HasPreviousPage", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntryArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeEntryArray_GetElement", (obj, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntryArray_GetNextUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeEntryArray_GetNextUrl", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntryArray_GetNextUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeEntryArray_GetNextUrl_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntryArray_GetPreviousUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeEntryArray_GetPreviousUrl", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntryArray_GetPreviousUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeEntryArray_GetPreviousUrl_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntryArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeEntryArray_GetSize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntryArray_GetTotalCount(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeEntryArray_GetTotalCount", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntryArray_HasNextPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeEntryArray_HasNextPage", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntryArray_HasPreviousPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeEntryArray_HasPreviousPage", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntry_GetDisplayScore(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeEntry_GetDisplayScore", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntry_GetDisplayScore_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeEntry_GetDisplayScore_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntry_GetExtraData(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeEntry_GetExtraData", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntry_GetExtraDataLength(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeEntry_GetExtraDataLength", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntry_GetExtraData_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeEntry_GetExtraData_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntry_GetID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeEntry_GetID", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntry_GetRank(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeEntry_GetRank", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntry_GetScore(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeEntry_GetScore", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntry_GetTimestamp(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeEntry_GetTimestamp", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntry_GetTimestamp_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeEntry_GetTimestamp_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeEntry_GetUser(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeEntry_GetUser", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_Create() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeOptions_Create", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_Destroy(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeOptions_Destroy", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetDescription(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeOptions_SetDescription", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetDescription_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeOptions_SetDescription_Native", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetEndDate(
        handle: crate::System::IntPtr,
        value: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeOptions_SetEndDate", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetEndDate_Native(
        handle: crate::System::IntPtr,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeOptions_SetEndDate_Native", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetIncludeActiveChallenges(
        handle: crate::System::IntPtr,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeOptions_SetIncludeActiveChallenges", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetIncludeFutureChallenges(
        handle: crate::System::IntPtr,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeOptions_SetIncludeFutureChallenges", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetIncludePastChallenges(
        handle: crate::System::IntPtr,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeOptions_SetIncludePastChallenges", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetLeaderboardName(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeOptions_SetLeaderboardName", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetLeaderboardName_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeOptions_SetLeaderboardName_Native", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetStartDate(
        handle: crate::System::IntPtr,
        value: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeOptions_SetStartDate", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetStartDate_Native(
        handle: crate::System::IntPtr,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeOptions_SetStartDate_Native", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetTitle(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeOptions_SetTitle", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetTitle_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeOptions_SetTitle_Native", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetViewerFilter(
        handle: crate::System::IntPtr,
        value: crate::Oculus::Platform::ChallengeViewerFilter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeOptions_SetViewerFilter", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ChallengeOptions_SetVisibility(
        handle: crate::System::IntPtr,
        value: crate::Oculus::Platform::ChallengeVisibility,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ChallengeOptions_SetVisibility", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenge_GetCreationType(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::ChallengeCreationType> {
        let __cordl_ret: crate::Oculus::Platform::ChallengeCreationType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Challenge_GetCreationType", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenge_GetDescription(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Challenge_GetDescription", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenge_GetDescription_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Challenge_GetDescription_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenge_GetEndDate(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Challenge_GetEndDate", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenge_GetEndDate_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Challenge_GetEndDate_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenge_GetID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Challenge_GetID", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenge_GetInvitedUsers(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Challenge_GetInvitedUsers", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenge_GetLeaderboard(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Challenge_GetLeaderboard", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenge_GetParticipants(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Challenge_GetParticipants", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenge_GetStartDate(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Challenge_GetStartDate", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenge_GetStartDate_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Challenge_GetStartDate_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenge_GetTitle(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Challenge_GetTitle", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenge_GetTitle_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Challenge_GetTitle_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenge_GetVisibility(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::ChallengeVisibility> {
        let __cordl_ret: crate::Oculus::Platform::ChallengeVisibility = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Challenge_GetVisibility", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_Create(
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        challengeOptions: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Challenges_Create", (leaderboardName, challengeOptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_Create_Native(
        leaderboardName: crate::System::IntPtr,
        challengeOptions: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_Challenges_Create_Native",
                (leaderboardName, challengeOptions),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_DeclineInvite(
        challengeID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Challenges_DeclineInvite", (challengeID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_Delete(
        challengeID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Challenges_Delete", (challengeID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_Get(challengeID: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Challenges_Get", (challengeID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_GetEntries(
        challengeID: u64,
        limit: i32,
        filter: crate::Oculus::Platform::LeaderboardFilterType,
        startAt: crate::Oculus::Platform::LeaderboardStartAt,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Challenges_GetEntries", (challengeID, limit, filter, startAt))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_GetEntriesAfterRank(
        challengeID: u64,
        limit: i32,
        afterRank: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_Challenges_GetEntriesAfterRank",
                (challengeID, limit, afterRank),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_GetEntriesByIds(
        challengeID: u64,
        limit: i32,
        startAt: crate::Oculus::Platform::LeaderboardStartAt,
        userIDs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        userIDLength: u32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_Challenges_GetEntriesByIds",
                (challengeID, limit, startAt, userIDs, userIDLength),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_GetList(
        challengeOptions: crate::System::IntPtr,
        limit: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Challenges_GetList", (challengeOptions, limit))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_GetNextChallenges(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Challenges_GetNextChallenges", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_GetNextEntries(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Challenges_GetNextEntries", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_GetPreviousChallenges(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Challenges_GetPreviousChallenges", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_GetPreviousEntries(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Challenges_GetPreviousEntries", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_Join(challengeID: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Challenges_Join", (challengeID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_Leave(challengeID: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Challenges_Leave", (challengeID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Challenges_UpdateInfo(
        challengeID: u64,
        challengeOptions: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Challenges_UpdateInfo", (challengeID, challengeOptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Colocation_GetCurrentMapUuid() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Colocation_GetCurrentMapUuid", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Colocation_RequestMap(
        uuid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Colocation_RequestMap", (uuid))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Colocation_RequestMap_Native(
        uuid: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Colocation_RequestMap_Native", (uuid))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Colocation_ShareMap(
        uuid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Colocation_ShareMap", (uuid))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Colocation_ShareMap_Native(
        uuid: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Colocation_ShareMap_Native", (uuid))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_CrashApplication() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_CrashApplication", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DataStore_Contains(
        obj: crate::System::IntPtr,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_DataStore_Contains", (obj, key))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DataStore_Contains_Native(
        obj: crate::System::IntPtr,
        key: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_DataStore_Contains_Native", (obj, key))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DataStore_GetKey(
        obj: crate::System::IntPtr,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_DataStore_GetKey", (obj, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DataStore_GetKey_Native(
        obj: crate::System::IntPtr,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_DataStore_GetKey_Native", (obj, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DataStore_GetNumKeys(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_DataStore_GetNumKeys", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DataStore_GetValue(
        obj: crate::System::IntPtr,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_DataStore_GetValue", (obj, key))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DataStore_GetValue_Native(
        obj: crate::System::IntPtr,
        key: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_DataStore_GetValue_Native", (obj, key))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DestinationArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_DestinationArray_GetElement", (obj, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DestinationArray_GetNextUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_DestinationArray_GetNextUrl", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DestinationArray_GetNextUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_DestinationArray_GetNextUrl_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DestinationArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_DestinationArray_GetSize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DestinationArray_HasNextPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_DestinationArray_HasNextPage", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Destination_GetApiName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Destination_GetApiName", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Destination_GetApiName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Destination_GetApiName_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Destination_GetDeeplinkMessage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Destination_GetDeeplinkMessage", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Destination_GetDeeplinkMessage_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Destination_GetDeeplinkMessage_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Destination_GetDisplayName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Destination_GetDisplayName", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Destination_GetDisplayName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Destination_GetDisplayName_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DeviceApplicationIntegrity_GetAttestationToken(
        challenge_nonce: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_DeviceApplicationIntegrity_GetAttestationToken",
                (challenge_nonce),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DeviceApplicationIntegrity_GetAttestationToken_Native(
        challenge_nonce: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_DeviceApplicationIntegrity_GetAttestationToken_Native",
                (challenge_nonce),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DeviceApplicationIntegrity_GetIntegrityToken(
        challenge_nonce: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_DeviceApplicationIntegrity_GetIntegrityToken",
                (challenge_nonce),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_DeviceApplicationIntegrity_GetIntegrityToken_Native(
        challenge_nonce: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_DeviceApplicationIntegrity_GetIntegrityToken_Native",
                (challenge_nonce),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Entitlement_GetIsViewerEntitled() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Entitlement_GetIsViewerEntitled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Error_GetCode(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Error_GetCode", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Error_GetDisplayableMessage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Error_GetDisplayableMessage", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Error_GetDisplayableMessage_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Error_GetDisplayableMessage_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Error_GetHttpCode(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Error_GetHttpCode", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Error_GetMessage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Error_GetMessage", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Error_GetMessage_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Error_GetMessage_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_FreeMessage(
        message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_FreeMessage", (message))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GetLoggedInUserLocale() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GetLoggedInUserLocale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GetLoggedInUserLocale_Native() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GetLoggedInUserLocale_Native", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GraphAPI_Get(
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GraphAPI_Get", (url))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GraphAPI_Get_Native(
        url: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GraphAPI_Get_Native", (url))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GraphAPI_Post(
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GraphAPI_Post", (url))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GraphAPI_Post_Native(
        url: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GraphAPI_Post_Native", (url))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceJoinIntent_GetDeeplinkMessage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresenceJoinIntent_GetDeeplinkMessage", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceJoinIntent_GetDeeplinkMessage_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresenceJoinIntent_GetDeeplinkMessage_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceJoinIntent_GetDestinationApiName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresenceJoinIntent_GetDestinationApiName", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceJoinIntent_GetDestinationApiName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresenceJoinIntent_GetDestinationApiName_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceJoinIntent_GetLobbySessionId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresenceJoinIntent_GetLobbySessionId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceJoinIntent_GetLobbySessionId_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresenceJoinIntent_GetLobbySessionId_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceJoinIntent_GetMatchSessionId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresenceJoinIntent_GetMatchSessionId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceJoinIntent_GetMatchSessionId_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresenceJoinIntent_GetMatchSessionId_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceLeaveIntent_GetDestinationApiName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresenceLeaveIntent_GetDestinationApiName", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceLeaveIntent_GetDestinationApiName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresenceLeaveIntent_GetDestinationApiName_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceLeaveIntent_GetLobbySessionId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresenceLeaveIntent_GetLobbySessionId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceLeaveIntent_GetLobbySessionId_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresenceLeaveIntent_GetLobbySessionId_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceLeaveIntent_GetMatchSessionId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresenceLeaveIntent_GetMatchSessionId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceLeaveIntent_GetMatchSessionId_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresenceLeaveIntent_GetMatchSessionId_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceOptions_Create() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresenceOptions_Create", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceOptions_Destroy(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresenceOptions_Destroy", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceOptions_SetDeeplinkMessageOverride(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_GroupPresenceOptions_SetDeeplinkMessageOverride",
                (handle, value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceOptions_SetDeeplinkMessageOverride_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_GroupPresenceOptions_SetDeeplinkMessageOverride_Native",
                (handle, value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceOptions_SetDestinationApiName(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresenceOptions_SetDestinationApiName", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceOptions_SetDestinationApiName_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_GroupPresenceOptions_SetDestinationApiName_Native",
                (handle, value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceOptions_SetIsJoinable(
        handle: crate::System::IntPtr,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresenceOptions_SetIsJoinable", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceOptions_SetLobbySessionId(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresenceOptions_SetLobbySessionId", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceOptions_SetLobbySessionId_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_GroupPresenceOptions_SetLobbySessionId_Native",
                (handle, value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceOptions_SetMatchSessionId(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresenceOptions_SetMatchSessionId", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresenceOptions_SetMatchSessionId_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_GroupPresenceOptions_SetMatchSessionId_Native",
                (handle, value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_Clear() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresence_Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_GetInvitableUsers(
        options: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresence_GetInvitableUsers", (options))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_GetSentInvites() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresence_GetSentInvites", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_LaunchInvitePanel(
        options: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresence_LaunchInvitePanel", (options))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_LaunchMultiplayerErrorDialog(
        options: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresence_LaunchMultiplayerErrorDialog", (options))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_LaunchRejoinDialog(
        lobby_session_id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        match_session_id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        destination_api_name: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_GroupPresence_LaunchRejoinDialog",
                (lobby_session_id, match_session_id, destination_api_name),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_LaunchRejoinDialog_Native(
        lobby_session_id: crate::System::IntPtr,
        match_session_id: crate::System::IntPtr,
        destination_api_name: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_GroupPresence_LaunchRejoinDialog_Native",
                (lobby_session_id, match_session_id, destination_api_name),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_LaunchRosterPanel(
        options: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresence_LaunchRosterPanel", (options))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_SendInvites(
        userIDs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        userIDLength: u32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresence_SendInvites", (userIDs, userIDLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_Set(
        groupPresenceOptions: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresence_Set", (groupPresenceOptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_SetDeeplinkMessageOverride(
        deeplink_message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresence_SetDeeplinkMessageOverride", (deeplink_message))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_SetDeeplinkMessageOverride_Native(
        deeplink_message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_GroupPresence_SetDeeplinkMessageOverride_Native",
                (deeplink_message),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_SetDestination(
        api_name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresence_SetDestination", (api_name))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_SetDestination_Native(
        api_name: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresence_SetDestination_Native", (api_name))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_SetIsJoinable(
        is_joinable: bool,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresence_SetIsJoinable", (is_joinable))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_SetLobbySession(
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresence_SetLobbySession", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_SetLobbySession_Native(
        id: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresence_SetLobbySession_Native", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_SetMatchSession(
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresence_SetMatchSession", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_GroupPresence_SetMatchSession_Native(
        id: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_GroupPresence_SetMatchSession_Native", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HTTP_Get(
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_HTTP_Get", (url))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HTTP_GetToFile(
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        diskFile: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_HTTP_GetToFile", (url, diskFile))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HTTP_GetToFile_Native(
        url: crate::System::IntPtr,
        diskFile: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_HTTP_GetToFile_Native", (url, diskFile))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HTTP_GetWithMessageType(
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        messageType: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_HTTP_GetWithMessageType", (url, messageType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HTTP_Get_Native(
        url: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_HTTP_Get_Native", (url))?;
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
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_HTTP_MultiPartPost",
                (url, filepath_param_name, filepath, access_token, post_params),
            )?;
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
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_HTTP_MultiPartPost_Native",
                (url, filepath_param_name, filepath, access_token, post_params, numItems),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HTTP_Post(
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_HTTP_Post", (url))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HTTP_Post_Native(
        url: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_HTTP_Post_Native", (url))?;
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
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_HTTP_StartTransfer", (url, headers))?;
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
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_HTTP_StartTransfer_Native", (url, headers, numItems))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HTTP_Write(
        transferId: u64,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        length: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_HTTP_Write", (transferId, bytes, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HTTP_WriteEOM(
        transferId: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_HTTP_WriteEOM", (transferId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HttpTransferUpdate_GetBytes(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_HttpTransferUpdate_GetBytes", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HttpTransferUpdate_GetID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_HttpTransferUpdate_GetID", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HttpTransferUpdate_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_HttpTransferUpdate_GetSize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_HttpTransferUpdate_IsCompleted(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_HttpTransferUpdate_IsCompleted", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_IAP_ConsumePurchase(
        sku: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_IAP_ConsumePurchase", (sku))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_IAP_ConsumePurchase_Native(
        sku: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_IAP_ConsumePurchase_Native", (sku))?;
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
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_IAP_GetProductsBySKU", (skus, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_IAP_GetViewerPurchases() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_IAP_GetViewerPurchases", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_IAP_GetViewerPurchasesDurableCache() -> quest_hook::libil2cpp::Result<
        u64,
    > {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_IAP_GetViewerPurchasesDurableCache", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_IAP_LaunchCheckoutFlow(
        sku: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_IAP_LaunchCheckoutFlow", (sku))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_IAP_LaunchCheckoutFlow_Native(
        sku: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_IAP_LaunchCheckoutFlow_Native", (sku))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InstalledApplicationArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_InstalledApplicationArray_GetElement", (obj, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InstalledApplicationArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_InstalledApplicationArray_GetSize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InstalledApplication_GetApplicationId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_InstalledApplication_GetApplicationId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InstalledApplication_GetApplicationId_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_InstalledApplication_GetApplicationId_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InstalledApplication_GetPackageName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_InstalledApplication_GetPackageName", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InstalledApplication_GetPackageName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_InstalledApplication_GetPackageName_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InstalledApplication_GetStatus(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_InstalledApplication_GetStatus", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InstalledApplication_GetStatus_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_InstalledApplication_GetStatus_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InstalledApplication_GetVersionCode(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_InstalledApplication_GetVersionCode", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InstalledApplication_GetVersionName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_InstalledApplication_GetVersionName", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InstalledApplication_GetVersionName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_InstalledApplication_GetVersionName_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InviteOptions_AddSuggestedUser(
        handle: crate::System::IntPtr,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_InviteOptions_AddSuggestedUser", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InviteOptions_ClearSuggestedUsers(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_InviteOptions_ClearSuggestedUsers", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InviteOptions_Create() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_InviteOptions_Create", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InviteOptions_Destroy(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_InviteOptions_Destroy", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_InvitePanelResultInfo_GetInvitesSent(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_InvitePanelResultInfo_GetInvitesSent", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LanguagePackInfo_GetEnglishName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LanguagePackInfo_GetEnglishName", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LanguagePackInfo_GetEnglishName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LanguagePackInfo_GetEnglishName_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LanguagePackInfo_GetNativeName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LanguagePackInfo_GetNativeName", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LanguagePackInfo_GetNativeName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LanguagePackInfo_GetNativeName_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LanguagePackInfo_GetTag(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LanguagePackInfo_GetTag", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LanguagePackInfo_GetTag_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LanguagePackInfo_GetTag_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LanguagePack_GetCurrent() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LanguagePack_GetCurrent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LanguagePack_SetCurrent(
        tag: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LanguagePack_SetCurrent", (tag))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LanguagePack_SetCurrent_Native(
        tag: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LanguagePack_SetCurrent_Native", (tag))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchBlockFlowResult_GetDidBlock(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LaunchBlockFlowResult_GetDidBlock", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchBlockFlowResult_GetDidCancel(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LaunchBlockFlowResult_GetDidCancel", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchDetails_GetDeeplinkMessage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LaunchDetails_GetDeeplinkMessage", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchDetails_GetDeeplinkMessage_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LaunchDetails_GetDeeplinkMessage_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchDetails_GetDestinationApiName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LaunchDetails_GetDestinationApiName", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchDetails_GetDestinationApiName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LaunchDetails_GetDestinationApiName_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchDetails_GetLaunchSource(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LaunchDetails_GetLaunchSource", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchDetails_GetLaunchSource_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LaunchDetails_GetLaunchSource_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchDetails_GetLaunchType(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::LaunchType> {
        let __cordl_ret: crate::Oculus::Platform::LaunchType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LaunchDetails_GetLaunchType", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchDetails_GetTrackingID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LaunchDetails_GetTrackingID", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchDetails_GetTrackingID_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LaunchDetails_GetTrackingID_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchDetails_GetUsers(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LaunchDetails_GetUsers", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchFriendRequestFlowResult_GetDidCancel(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LaunchFriendRequestFlowResult_GetDidCancel", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchFriendRequestFlowResult_GetDidSendRequest(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LaunchFriendRequestFlowResult_GetDidSendRequest", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchInvitePanelFlowResult_GetInvitedUsers(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LaunchInvitePanelFlowResult_GetInvitedUsers", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchReportFlowResult_GetDidCancel(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LaunchReportFlowResult_GetDidCancel", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchReportFlowResult_GetUserReportId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LaunchReportFlowResult_GetUserReportId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchUnblockFlowResult_GetDidCancel(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LaunchUnblockFlowResult_GetDidCancel", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LaunchUnblockFlowResult_GetDidUnblock(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LaunchUnblockFlowResult_GetDidUnblock", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardArray_GetElement", (obj, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardArray_GetNextUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardArray_GetNextUrl", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardArray_GetNextUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardArray_GetNextUrl_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardArray_GetSize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardArray_HasNextPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardArray_HasNextPage", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntryArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardEntryArray_GetElement", (obj, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntryArray_GetNextUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardEntryArray_GetNextUrl", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntryArray_GetNextUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardEntryArray_GetNextUrl_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntryArray_GetPreviousUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardEntryArray_GetPreviousUrl", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntryArray_GetPreviousUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardEntryArray_GetPreviousUrl_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntryArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardEntryArray_GetSize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntryArray_GetTotalCount(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardEntryArray_GetTotalCount", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntryArray_HasNextPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardEntryArray_HasNextPage", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntryArray_HasPreviousPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardEntryArray_HasPreviousPage", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntry_GetDisplayScore(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardEntry_GetDisplayScore", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntry_GetDisplayScore_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardEntry_GetDisplayScore_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntry_GetExtraData(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardEntry_GetExtraData", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntry_GetExtraDataLength(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardEntry_GetExtraDataLength", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntry_GetExtraData_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardEntry_GetExtraData_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntry_GetID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardEntry_GetID", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntry_GetRank(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardEntry_GetRank", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntry_GetScore(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardEntry_GetScore", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntry_GetSupplementaryMetric(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardEntry_GetSupplementaryMetric", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntry_GetTimestamp(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardEntry_GetTimestamp", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntry_GetTimestamp_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardEntry_GetTimestamp_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardEntry_GetUser(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardEntry_GetUser", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardUpdateStatus_GetDidUpdate(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardUpdateStatus_GetDidUpdate", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardUpdateStatus_GetUpdatedChallengeId(
        obj: crate::System::IntPtr,
        index: u32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardUpdateStatus_GetUpdatedChallengeId", (obj, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LeaderboardUpdateStatus_GetUpdatedChallengeIdsSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LeaderboardUpdateStatus_GetUpdatedChallengeIdsSize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_Get(
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Leaderboard_Get", (leaderboardName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_GetApiName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Leaderboard_GetApiName", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_GetApiName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Leaderboard_GetApiName_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_GetDestination(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Leaderboard_GetDestination", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_GetEntries(
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        limit: i32,
        filter: crate::Oculus::Platform::LeaderboardFilterType,
        startAt: crate::Oculus::Platform::LeaderboardStartAt,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_Leaderboard_GetEntries",
                (leaderboardName, limit, filter, startAt),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_GetEntriesAfterRank(
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        limit: i32,
        afterRank: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_Leaderboard_GetEntriesAfterRank",
                (leaderboardName, limit, afterRank),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_GetEntriesAfterRank_Native(
        leaderboardName: crate::System::IntPtr,
        limit: i32,
        afterRank: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_Leaderboard_GetEntriesAfterRank_Native",
                (leaderboardName, limit, afterRank),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_GetEntriesByIds(
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        limit: i32,
        startAt: crate::Oculus::Platform::LeaderboardStartAt,
        userIDs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        userIDLength: u32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_Leaderboard_GetEntriesByIds",
                (leaderboardName, limit, startAt, userIDs, userIDLength),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_GetEntriesByIds_Native(
        leaderboardName: crate::System::IntPtr,
        limit: i32,
        startAt: crate::Oculus::Platform::LeaderboardStartAt,
        userIDs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        userIDLength: u32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_Leaderboard_GetEntriesByIds_Native",
                (leaderboardName, limit, startAt, userIDs, userIDLength),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_GetEntries_Native(
        leaderboardName: crate::System::IntPtr,
        limit: i32,
        filter: crate::Oculus::Platform::LeaderboardFilterType,
        startAt: crate::Oculus::Platform::LeaderboardStartAt,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_Leaderboard_GetEntries_Native",
                (leaderboardName, limit, filter, startAt),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_GetID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Leaderboard_GetID", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_GetNextEntries(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Leaderboard_GetNextEntries", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_GetPreviousEntries(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Leaderboard_GetPreviousEntries", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_Get_Native(
        leaderboardName: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Leaderboard_Get_Native", (leaderboardName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_WriteEntry(
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        score: i64,
        extraData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        extraDataLength: u32,
        forceUpdate: bool,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_Leaderboard_WriteEntry",
                (leaderboardName, score, extraData, extraDataLength, forceUpdate),
            )?;
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
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_Leaderboard_WriteEntryWithSupplementaryMetric",
                (
                    leaderboardName,
                    score,
                    supplementaryMetric,
                    extraData,
                    extraDataLength,
                    forceUpdate,
                ),
            )?;
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
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_Leaderboard_WriteEntryWithSupplementaryMetric_Native",
                (
                    leaderboardName,
                    score,
                    supplementaryMetric,
                    extraData,
                    extraDataLength,
                    forceUpdate,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Leaderboard_WriteEntry_Native(
        leaderboardName: crate::System::IntPtr,
        score: i64,
        extraData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        extraDataLength: u32,
        forceUpdate: bool,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_Leaderboard_WriteEntry_Native",
                (leaderboardName, score, extraData, extraDataLength, forceUpdate),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LinkedAccountArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LinkedAccountArray_GetElement", (obj, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LinkedAccountArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LinkedAccountArray_GetSize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LinkedAccount_GetAccessToken(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LinkedAccount_GetAccessToken", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LinkedAccount_GetAccessToken_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LinkedAccount_GetAccessToken_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LinkedAccount_GetServiceProvider(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::ServiceProvider> {
        let __cordl_ret: crate::Oculus::Platform::ServiceProvider = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LinkedAccount_GetServiceProvider", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LinkedAccount_GetUserId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LinkedAccount_GetUserId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LinkedAccount_GetUserId_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LinkedAccount_GetUserId_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LivestreamingApplicationStatus_GetStreamingEnabled(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LivestreamingApplicationStatus_GetStreamingEnabled", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LivestreamingStartResult_GetStreamingResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        crate::Oculus::Platform::LivestreamingStartStatus,
    > {
        let __cordl_ret: crate::Oculus::Platform::LivestreamingStartStatus = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LivestreamingStartResult_GetStreamingResult", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LivestreamingStatus_GetCommentsVisible(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LivestreamingStatus_GetCommentsVisible", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LivestreamingStatus_GetIsPaused(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LivestreamingStatus_GetIsPaused", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LivestreamingStatus_GetLivestreamingEnabled(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LivestreamingStatus_GetLivestreamingEnabled", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LivestreamingStatus_GetLivestreamingType(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LivestreamingStatus_GetLivestreamingType", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LivestreamingStatus_GetMicEnabled(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LivestreamingStatus_GetMicEnabled", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LivestreamingVideoStats_GetCommentCount(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LivestreamingVideoStats_GetCommentCount", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LivestreamingVideoStats_GetReactionCount(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LivestreamingVideoStats_GetReactionCount", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LivestreamingVideoStats_GetTotalViews(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LivestreamingVideoStats_GetTotalViews", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_LivestreamingVideoStats_GetTotalViews_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_LivestreamingVideoStats_GetTotalViews_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Livestreaming_IsAllowedForApplication(
        packageName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Livestreaming_IsAllowedForApplication", (packageName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Livestreaming_IsAllowedForApplication_Native(
        packageName: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Livestreaming_IsAllowedForApplication_Native", (packageName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Livestreaming_StartPartyStream() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Livestreaming_StartPartyStream", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Livestreaming_StartStream(
        audience: crate::Oculus::Platform::LivestreamingAudience,
        micStatus: crate::Oculus::Platform::LivestreamingMicrophoneStatus,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Livestreaming_StartStream", (audience, micStatus))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Livestreaming_StopPartyStream() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Livestreaming_StopPartyStream", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Livestreaming_StopStream() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Livestreaming_StopStream", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Livestreaming_UpdateMicStatus(
        micStatus: crate::Oculus::Platform::LivestreamingMicrophoneStatus,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Livestreaming_UpdateMicStatus", (micStatus))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Log_NewEvent(
        eventName: crate::System::IntPtr,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::IntPtr>,
        >,
        length: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Log_NewEvent", (eventName, values, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Media_ShareToFacebook(
        postTextSuggestion: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        contentType: crate::Oculus::Platform::MediaContentType,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_Media_ShareToFacebook",
                (postTextSuggestion, filePath, contentType),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Media_ShareToFacebook_Native(
        postTextSuggestion: crate::System::IntPtr,
        filePath: crate::System::IntPtr,
        contentType: crate::Oculus::Platform::MediaContentType,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_Media_ShareToFacebook_Native",
                (postTextSuggestion, filePath, contentType),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetAbuseReportRecording(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetAbuseReportRecording", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetAchievementDefinitionArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetAchievementDefinitionArray", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetAchievementProgressArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetAchievementProgressArray", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetAchievementUpdate(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetAchievementUpdate", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetAppDownloadProgressResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetAppDownloadProgressResult", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetAppDownloadResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetAppDownloadResult", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetApplicationInviteArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetApplicationInviteArray", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetApplicationVersion(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetApplicationVersion", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetAssetDetails(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetAssetDetails", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetAssetDetailsArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetAssetDetailsArray", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetAssetFileDeleteResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetAssetFileDeleteResult", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetAssetFileDownloadCancelResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetAssetFileDownloadCancelResult", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetAssetFileDownloadResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetAssetFileDownloadResult", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetAssetFileDownloadUpdate(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetAssetFileDownloadUpdate", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetAvatarEditorResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetAvatarEditorResult", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetBlockedUserArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetBlockedUserArray", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetChallenge(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetChallenge", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetChallengeArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetChallengeArray", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetChallengeEntryArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetChallengeEntryArray", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetDataStore(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetDataStore", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetDestinationArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetDestinationArray", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetError(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetError", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetGroupPresenceJoinIntent(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetGroupPresenceJoinIntent", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetGroupPresenceLeaveIntent(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetGroupPresenceLeaveIntent", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetHttpTransferUpdate(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetHttpTransferUpdate", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetInstalledApplicationArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetInstalledApplicationArray", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetInvitePanelResultInfo(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetInvitePanelResultInfo", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetLaunchBlockFlowResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetLaunchBlockFlowResult", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetLaunchFriendRequestFlowResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetLaunchFriendRequestFlowResult", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetLaunchInvitePanelFlowResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetLaunchInvitePanelFlowResult", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetLaunchReportFlowResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetLaunchReportFlowResult", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetLaunchUnblockFlowResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetLaunchUnblockFlowResult", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetLeaderboardArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetLeaderboardArray", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetLeaderboardEntryArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetLeaderboardEntryArray", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetLeaderboardUpdateStatus(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetLeaderboardUpdateStatus", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetLinkedAccountArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetLinkedAccountArray", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetLivestreamingApplicationStatus(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetLivestreamingApplicationStatus", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetLivestreamingStartResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetLivestreamingStartResult", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetLivestreamingStatus(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetLivestreamingStatus", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetLivestreamingVideoStats(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetLivestreamingVideoStats", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetMicrophoneAvailabilityState(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetMicrophoneAvailabilityState", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetNativeMessage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetNativeMessage", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetNetSyncConnection(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetNetSyncConnection", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetNetSyncSessionArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetNetSyncSessionArray", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetNetSyncSessionsChangedNotification(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetNetSyncSessionsChangedNotification", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetNetSyncSetSessionPropertyResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetNetSyncSetSessionPropertyResult", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetNetSyncVoipAttenuationValueArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetNetSyncVoipAttenuationValueArray", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetOrgScopedID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetOrgScopedID", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetParty(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetParty", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetPartyID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetPartyID", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetPartyUpdateNotification(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetPartyUpdateNotification", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetPidArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetPidArray", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetPlatformInitialize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetPlatformInitialize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetProductArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetProductArray", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetPurchase(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetPurchase", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetPurchaseArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetPurchaseArray", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetRejoinDialogResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetRejoinDialogResult", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetRequestID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetRequestID", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetSdkAccountArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetSdkAccountArray", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetSendInvitesResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetSendInvitesResult", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetShareMediaResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetShareMediaResult", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetString(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetString", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetStringForJavascript(
        message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetStringForJavascript", (message))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetStringForJavascript_Native(
        message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetStringForJavascript_Native", (message))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetString_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetString_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetSystemVoipState(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetSystemVoipState", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetType(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::Message_MessageType> {
        let __cordl_ret: crate::Oculus::Platform::Message_MessageType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetType", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetUser(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetUser", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetUserAccountAgeCategory(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetUserAccountAgeCategory", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetUserArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetUserArray", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetUserCapabilityArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetUserCapabilityArray", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetUserDataStoreUpdateResponse(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetUserDataStoreUpdateResponse", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetUserProof(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetUserProof", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_GetUserReportID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_GetUserReportID", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Message_IsError(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Message_IsError", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_MicrophoneAvailabilityState_GetMicrophoneAvailable(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_MicrophoneAvailabilityState_GetMicrophoneAvailable", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Microphone_Create() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Microphone_Create", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Microphone_Destroy(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Microphone_Destroy", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Microphone_GetNumSamplesAvailable(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Microphone_GetNumSamplesAvailable", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Microphone_GetOutputBufferMaxSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Microphone_GetOutputBufferMaxSize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Microphone_GetPCM(
        obj: crate::System::IntPtr,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
        outputBufferNumElements: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_Microphone_GetPCM",
                (obj, outputBuffer, outputBufferNumElements),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Microphone_GetPCMFloat(
        obj: crate::System::IntPtr,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        outputBufferNumElements: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_Microphone_GetPCMFloat",
                (obj, outputBuffer, outputBufferNumElements),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Microphone_ReadData(
        obj: crate::System::IntPtr,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        outputBufferSize: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Microphone_ReadData", (obj, outputBuffer, outputBufferSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Microphone_SetAcceptableRecordingDelayHint(
        obj: crate::System::IntPtr,
        delayMs: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Microphone_SetAcceptableRecordingDelayHint", (obj, delayMs))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Microphone_Start(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Microphone_Start", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Microphone_Stop(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Microphone_Stop", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_MultiplayerErrorOptions_Create() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_MultiplayerErrorOptions_Create", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_MultiplayerErrorOptions_Destroy(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_MultiplayerErrorOptions_Destroy", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_MultiplayerErrorOptions_SetErrorKey(
        handle: crate::System::IntPtr,
        value: crate::Oculus::Platform::MultiplayerErrorErrorKey,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_MultiplayerErrorOptions_SetErrorKey", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncConnection_GetConnectionId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSyncConnection_GetConnectionId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncConnection_GetDisconnectReason(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        crate::Oculus::Platform::NetSyncDisconnectReason,
    > {
        let __cordl_ret: crate::Oculus::Platform::NetSyncDisconnectReason = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSyncConnection_GetDisconnectReason", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncConnection_GetSessionId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSyncConnection_GetSessionId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncConnection_GetStatus(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        crate::Oculus::Platform::NetSyncConnectionStatus,
    > {
        let __cordl_ret: crate::Oculus::Platform::NetSyncConnectionStatus = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSyncConnection_GetStatus", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncConnection_GetZoneId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSyncConnection_GetZoneId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncConnection_GetZoneId_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSyncConnection_GetZoneId_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncOptions_Create() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSyncOptions_Create", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncOptions_Destroy(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSyncOptions_Destroy", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncOptions_SetVoipGroup(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSyncOptions_SetVoipGroup", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncOptions_SetVoipGroup_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSyncOptions_SetVoipGroup_Native", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncOptions_SetVoipStreamDefault(
        handle: crate::System::IntPtr,
        value: crate::Oculus::Platform::NetSyncVoipStreamMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSyncOptions_SetVoipStreamDefault", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncOptions_SetZoneId(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSyncOptions_SetZoneId", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncOptions_SetZoneId_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSyncOptions_SetZoneId_Native", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncSessionArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSyncSessionArray_GetElement", (obj, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncSessionArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSyncSessionArray_GetSize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncSession_GetConnectionId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSyncSession_GetConnectionId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncSession_GetMuted(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSyncSession_GetMuted", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncSession_GetSessionId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSyncSession_GetSessionId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncSession_GetUserId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSyncSession_GetUserId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncSession_GetVoipGroup(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSyncSession_GetVoipGroup", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncSession_GetVoipGroup_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSyncSession_GetVoipGroup_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncSessionsChangedNotification_GetConnectionId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSyncSessionsChangedNotification_GetConnectionId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncSessionsChangedNotification_GetSessions(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSyncSessionsChangedNotification_GetSessions", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncSetSessionPropertyResult_GetSession(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSyncSetSessionPropertyResult_GetSession", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncVoipAttenuationValueArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSyncVoipAttenuationValueArray_GetElement", (obj, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncVoipAttenuationValueArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSyncVoipAttenuationValueArray_GetSize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncVoipAttenuationValue_GetDecibels(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSyncVoipAttenuationValue_GetDecibels", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSyncVoipAttenuationValue_GetDistance(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSyncVoipAttenuationValue_GetDistance", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_Connect(
        connect_options: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSync_Connect", (connect_options))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_Disconnect(
        connection_id: i64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSync_Disconnect", (connection_id))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_GetAmbisonicFloatPCM(
        connection_id: i64,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        outputBufferNumElements: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_NetSync_GetAmbisonicFloatPCM",
                (connection_id, outputBuffer, outputBufferNumElements),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_GetAmbisonicInt16PCM(
        connection_id: i64,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
        outputBufferNumElements: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_NetSync_GetAmbisonicInt16PCM",
                (connection_id, outputBuffer, outputBufferNumElements),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_GetAmbisonicInterleavedFloatPCM(
        connection_id: i64,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        outputBufferNumElements: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_NetSync_GetAmbisonicInterleavedFloatPCM",
                (connection_id, outputBuffer, outputBufferNumElements),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_GetAmbisonicInterleavedInt16PCM(
        connection_id: i64,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
        outputBufferNumElements: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_NetSync_GetAmbisonicInterleavedInt16PCM",
                (connection_id, outputBuffer, outputBufferNumElements),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_GetListenerPosition(
        connection_id: i64,
        sessionId: u64,
        position: quest_hook::libil2cpp::ByRefMut<
            crate::Oculus::Platform::CAPI_ovrNetSyncVec3,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_NetSync_GetListenerPosition",
                (connection_id, sessionId, position),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_GetMonostreamFloatPCM(
        connection_id: i64,
        sessionId: u64,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        outputBufferNumElements: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_NetSync_GetMonostreamFloatPCM",
                (connection_id, sessionId, outputBuffer, outputBufferNumElements),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_GetMonostreamInt16PCM(
        connection_id: i64,
        session_id: u64,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
        outputBufferNumElements: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_NetSync_GetMonostreamInt16PCM",
                (connection_id, session_id, outputBuffer, outputBufferNumElements),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_GetPcmBufferMaxSamples() -> quest_hook::libil2cpp::Result<
        crate::System::UIntPtr,
    > {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSync_GetPcmBufferMaxSamples", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_GetSessions(
        connection_id: i64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSync_GetSessions", (connection_id))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_GetVoipAmplitude(
        connection_id: i64,
        sessionId: u64,
        amplitude: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_NetSync_GetVoipAmplitude",
                (connection_id, sessionId, amplitude),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_GetVoipAttenuation(
        connection_id: i64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSync_GetVoipAttenuation", (connection_id))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_GetVoipAttenuationDefault() -> quest_hook::libil2cpp::Result<
        u64,
    > {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSync_GetVoipAttenuationDefault", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_SetListenerPosition(
        connection_id: i64,
        position: quest_hook::libil2cpp::ByRefMut<
            crate::Oculus::Platform::CAPI_ovrNetSyncVec3,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSync_SetListenerPosition", (connection_id, position))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_SetVoipAttenuation(
        connection_id: i64,
        distances: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        decibels: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        count: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_NetSync_SetVoipAttenuation",
                (connection_id, distances, decibels, count),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_SetVoipAttenuationModel(
        connection_id: i64,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        distances: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        decibels: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        count: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_NetSync_SetVoipAttenuationModel",
                (connection_id, name, distances, decibels, count),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_SetVoipAttenuationModel_Native(
        connection_id: i64,
        name: crate::System::IntPtr,
        distances: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        decibels: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        count: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_NetSync_SetVoipAttenuationModel_Native",
                (connection_id, name, distances, decibels, count),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_SetVoipChannelCfg(
        connection_id: i64,
        channel_name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attnmodel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        disable_spatialization: bool,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_NetSync_SetVoipChannelCfg",
                (connection_id, channel_name, attnmodel, disable_spatialization),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_SetVoipChannelCfg_Native(
        connection_id: i64,
        channel_name: crate::System::IntPtr,
        attnmodel: crate::System::IntPtr,
        disable_spatialization: bool,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_NetSync_SetVoipChannelCfg_Native",
                (connection_id, channel_name, attnmodel, disable_spatialization),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_SetVoipGroup(
        connection_id: i64,
        group_id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSync_SetVoipGroup", (connection_id, group_id))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_SetVoipGroup_Native(
        connection_id: i64,
        group_id: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSync_SetVoipGroup_Native", (connection_id, group_id))?;
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
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_NetSync_SetVoipListentoChannels",
                (connection_id, listento_channels, count),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_SetVoipMicSource(
        connection_id: i64,
        mic_source: crate::Oculus::Platform::NetSyncVoipMicSource,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_NetSync_SetVoipMicSource", (connection_id, mic_source))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_SetVoipSessionMuted(
        connection_id: i64,
        session_id: u64,
        muted: bool,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_NetSync_SetVoipSessionMuted",
                (connection_id, session_id, muted),
            )?;
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
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_NetSync_SetVoipSpeaktoChannels",
                (connection_id, speakto_channels, count),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_NetSync_SetVoipStreamMode(
        connection_id: i64,
        sessionId: u64,
        streamMode: crate::Oculus::Platform::NetSyncVoipStreamMode,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_NetSync_SetVoipStreamMode",
                (connection_id, sessionId, streamMode),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Notification_MarkAsRead(
        notificationID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Notification_MarkAsRead", (notificationID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_OrgScopedID_GetID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_OrgScopedID_GetID", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Packet_Free(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Packet_Free", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Packet_GetBytes(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Packet_GetBytes", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Packet_GetSenderID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Packet_GetSenderID", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Packet_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Packet_GetSize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PartyID_GetID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_PartyID_GetID", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PartyUpdateNotification_GetAction(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::PartyUpdateAction> {
        let __cordl_ret: crate::Oculus::Platform::PartyUpdateAction = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_PartyUpdateNotification_GetAction", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PartyUpdateNotification_GetPartyId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_PartyUpdateNotification_GetPartyId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PartyUpdateNotification_GetSenderId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_PartyUpdateNotification_GetSenderId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PartyUpdateNotification_GetUpdateTimestamp(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_PartyUpdateNotification_GetUpdateTimestamp", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PartyUpdateNotification_GetUpdateTimestamp_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_PartyUpdateNotification_GetUpdateTimestamp_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PartyUpdateNotification_GetUserAlias(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_PartyUpdateNotification_GetUserAlias", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PartyUpdateNotification_GetUserAlias_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_PartyUpdateNotification_GetUserAlias_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PartyUpdateNotification_GetUserId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_PartyUpdateNotification_GetUserId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PartyUpdateNotification_GetUserName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_PartyUpdateNotification_GetUserName", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PartyUpdateNotification_GetUserName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_PartyUpdateNotification_GetUserName_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_Create() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Party_Create", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_GatherInApplication(
        partyID: u64,
        appID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Party_GatherInApplication", (partyID, appID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_Get(partyID: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Party_Get", (partyID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_GetCurrent() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Party_GetCurrent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_GetCurrentForUser(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Party_GetCurrentForUser", (userID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_GetID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Party_GetID", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_GetInvitedUsers(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Party_GetInvitedUsers", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_GetLeader(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Party_GetLeader", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_GetUsers(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Party_GetUsers", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_Invite(
        partyID: u64,
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Party_Invite", (partyID, userID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_Join(partyID: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Party_Join", (partyID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_Leave(partyID: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Party_Leave", (partyID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_PluginGetSharedMemHandle() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Party_PluginGetSharedMemHandle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_PluginGetVoipMicrophoneMuted() -> quest_hook::libil2cpp::Result<
        crate::Oculus::Platform::VoipMuteState,
    > {
        let __cordl_ret: crate::Oculus::Platform::VoipMuteState = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Party_PluginGetVoipMicrophoneMuted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_PluginGetVoipPassthrough() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Party_PluginGetVoipPassthrough", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Party_PluginGetVoipStatus() -> quest_hook::libil2cpp::Result<
        crate::Oculus::Platform::SystemVoipStatus,
    > {
        let __cordl_ret: crate::Oculus::Platform::SystemVoipStatus = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Party_PluginGetVoipStatus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PidArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_PidArray_GetElement", (obj, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PidArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_PidArray_GetSize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Pid_GetId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Pid_GetId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Pid_GetId_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Pid_GetId_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PlatformInitializeWithAccessToken(
        appId: u64,
        accessToken: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_PlatformInitializeWithAccessToken", (appId, accessToken))?;
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
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_PlatformInitializeWithAccessTokenAndOptions",
                (appId, accessToken, configOptions, numOptions),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PlatformInitialize_GetResult(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        crate::Oculus::Platform::PlatformInitializeResult,
    > {
        let __cordl_ret: crate::Oculus::Platform::PlatformInitializeResult = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_PlatformInitialize_GetResult", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Platform_InitializeStandaloneOculus(
        init: quest_hook::libil2cpp::ByRefMut<
            crate::Oculus::Platform::CAPI_OculusInitParams,
        >,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Platform_InitializeStandaloneOculus", (init))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PopMessage() -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_PopMessage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Price_GetAmountInHundredths(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Price_GetAmountInHundredths", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Price_GetCurrency(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Price_GetCurrency", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Price_GetCurrency_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Price_GetCurrency_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Price_GetFormatted(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Price_GetFormatted", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Price_GetFormatted_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Price_GetFormatted_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ProductArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ProductArray_GetElement", (obj, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ProductArray_GetNextUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ProductArray_GetNextUrl", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ProductArray_GetNextUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ProductArray_GetNextUrl_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ProductArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ProductArray_GetSize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ProductArray_HasNextPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ProductArray_HasNextPage", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Product_GetDescription(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Product_GetDescription", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Product_GetDescription_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Product_GetDescription_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Product_GetFormattedPrice(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Product_GetFormattedPrice", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Product_GetFormattedPrice_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Product_GetFormattedPrice_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Product_GetName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Product_GetName", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Product_GetName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Product_GetName_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Product_GetSKU(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Product_GetSKU", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Product_GetSKU_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Product_GetSKU_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PurchaseArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_PurchaseArray_GetElement", (obj, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PurchaseArray_GetNextUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_PurchaseArray_GetNextUrl", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PurchaseArray_GetNextUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_PurchaseArray_GetNextUrl_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PurchaseArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_PurchaseArray_GetSize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_PurchaseArray_HasNextPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_PurchaseArray_HasNextPage", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Purchase_GetDeveloperPayload(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Purchase_GetDeveloperPayload", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Purchase_GetDeveloperPayload_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Purchase_GetDeveloperPayload_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Purchase_GetExpirationTime(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Purchase_GetExpirationTime", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Purchase_GetExpirationTime_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Purchase_GetExpirationTime_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Purchase_GetGrantTime(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Purchase_GetGrantTime", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Purchase_GetGrantTime_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Purchase_GetGrantTime_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Purchase_GetPurchaseID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Purchase_GetPurchaseID", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Purchase_GetPurchaseStrID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Purchase_GetPurchaseStrID", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Purchase_GetPurchaseStrID_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Purchase_GetPurchaseStrID_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Purchase_GetReportingId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Purchase_GetReportingId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Purchase_GetReportingId_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Purchase_GetReportingId_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Purchase_GetSKU(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Purchase_GetSKU", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Purchase_GetSKU_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Purchase_GetSKU_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RejoinDialogResult_GetRejoinSelected(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_RejoinDialogResult_GetRejoinSelected", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresenceOptions_Create() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_RichPresenceOptions_Create", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresenceOptions_Destroy(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_RichPresenceOptions_Destroy", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresenceOptions_SetApiName(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_RichPresenceOptions_SetApiName", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresenceOptions_SetApiName_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_RichPresenceOptions_SetApiName_Native", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresenceOptions_SetDeeplinkMessageOverride(
        handle: crate::System::IntPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_RichPresenceOptions_SetDeeplinkMessageOverride",
                (handle, value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresenceOptions_SetDeeplinkMessageOverride_Native(
        handle: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_RichPresenceOptions_SetDeeplinkMessageOverride_Native",
                (handle, value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresenceOptions_SetIsJoinable(
        handle: crate::System::IntPtr,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_RichPresenceOptions_SetIsJoinable", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresence_Clear() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_RichPresence_Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresence_GetDestinations() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_RichPresence_GetDestinations", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresence_Set(
        richPresenceOptions: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_RichPresence_Set", (richPresenceOptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresence_SetDestination(
        api_name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_RichPresence_SetDestination", (api_name))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresence_SetDestination_Native(
        api_name: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_RichPresence_SetDestination_Native", (api_name))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresence_SetIsJoinable(
        is_joinable: bool,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_RichPresence_SetIsJoinable", (is_joinable))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresence_SetLobbySession(
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_RichPresence_SetLobbySession", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresence_SetLobbySession_Native(
        id: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_RichPresence_SetLobbySession_Native", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresence_SetMatchSession(
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_RichPresence_SetMatchSession", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RichPresence_SetMatchSession_Native(
        id: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_RichPresence_SetMatchSession_Native", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RosterOptions_AddSuggestedUser(
        handle: crate::System::IntPtr,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_RosterOptions_AddSuggestedUser", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RosterOptions_ClearSuggestedUsers(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_RosterOptions_ClearSuggestedUsers", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RosterOptions_Create() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_RosterOptions_Create", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_RosterOptions_Destroy(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_RosterOptions_Destroy", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_SdkAccountArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_SdkAccountArray_GetElement", (obj, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_SdkAccountArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_SdkAccountArray_GetSize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_SdkAccount_GetAccountType(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::SdkAccountType> {
        let __cordl_ret: crate::Oculus::Platform::SdkAccountType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_SdkAccount_GetAccountType", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_SdkAccount_GetUserId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_SdkAccount_GetUserId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_SendInvitesResult_GetInvites(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_SendInvitesResult_GetInvites", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_SetDeveloperAccessToken(
        accessToken: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_SetDeveloperAccessToken", (accessToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_ShareMediaResult_GetStatus(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::ShareMediaStatus> {
        let __cordl_ret: crate::Oculus::Platform::ShareMediaStatus = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_ShareMediaResult_GetStatus", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_SupplementaryMetric_GetID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_SupplementaryMetric_GetID", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_SupplementaryMetric_GetMetric(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_SupplementaryMetric_GetMetric", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_SystemVoipState_GetMicrophoneMuted(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::VoipMuteState> {
        let __cordl_ret: crate::Oculus::Platform::VoipMuteState = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_SystemVoipState_GetMicrophoneMuted", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_SystemVoipState_GetStatus(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::SystemVoipStatus> {
        let __cordl_ret: crate::Oculus::Platform::SystemVoipStatus = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_SystemVoipState_GetStatus", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUserAppAccessArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_TestUserAppAccessArray_GetElement", (obj, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUserAppAccessArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_TestUserAppAccessArray_GetSize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUserAppAccess_GetAccessToken(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_TestUserAppAccess_GetAccessToken", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUserAppAccess_GetAccessToken_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_TestUserAppAccess_GetAccessToken_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUserAppAccess_GetAppId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_TestUserAppAccess_GetAppId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUserAppAccess_GetUserId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_TestUserAppAccess_GetUserId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUser_GetAccessToken(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_TestUser_GetAccessToken", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUser_GetAccessToken_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_TestUser_GetAccessToken_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUser_GetAppAccessArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_TestUser_GetAppAccessArray", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUser_GetFbAppAccessArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_TestUser_GetFbAppAccessArray", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUser_GetFriendAccessToken(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_TestUser_GetFriendAccessToken", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUser_GetFriendAccessToken_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_TestUser_GetFriendAccessToken_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUser_GetFriendAppAccessArray(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_TestUser_GetFriendAppAccessArray", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUser_GetUserAlias(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_TestUser_GetUserAlias", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUser_GetUserAlias_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_TestUser_GetUserAlias_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUser_GetUserFbid(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_TestUser_GetUserFbid", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_TestUser_GetUserId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_TestUser_GetUserId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UnityInitGlobals(
        loggingCB: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UnityInitGlobals", (loggingCB))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UnityInitWrapper(
        appId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UnityInitWrapper", (appId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UnityInitWrapperAsynchronous(
        appId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UnityInitWrapperAsynchronous", (appId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UnityInitWrapperStandalone(
        accessToken: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loggingCB: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UnityInitWrapperStandalone", (accessToken, loggingCB))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UnityInitWrapperWindows(
        appId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loggingCB: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UnityInitWrapperWindows", (appId, loggingCB))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UnityInitWrapperWindowsAsynchronous(
        appId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loggingCB: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UnityInitWrapperWindowsAsynchronous", (appId, loggingCB))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UnityResetTestPlatform() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UnityResetTestPlatform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserAccountAgeCategory_GetAgeCategory(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::AccountAgeCategory> {
        let __cordl_ret: crate::Oculus::Platform::AccountAgeCategory = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserAccountAgeCategory_GetAgeCategory", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserAgeCategory_Get() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserAgeCategory_Get", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserAgeCategory_Report(
        age_category: crate::Oculus::Platform::AppAgeCategory,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserAgeCategory_Report", (age_category))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserArray_GetElement", (obj, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserArray_GetNextUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserArray_GetNextUrl", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserArray_GetNextUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserArray_GetNextUrl_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserArray_GetSize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserArray_HasNextPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserArray_HasNextPage", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserCapabilityArray_GetElement(
        obj: crate::System::IntPtr,
        index: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserCapabilityArray_GetElement", (obj, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserCapabilityArray_GetNextUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserCapabilityArray_GetNextUrl", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserCapabilityArray_GetNextUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserCapabilityArray_GetNextUrl_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserCapabilityArray_GetSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserCapabilityArray_GetSize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserCapabilityArray_HasNextPage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserCapabilityArray_HasNextPage", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserCapability_GetDescription(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserCapability_GetDescription", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserCapability_GetDescription_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserCapability_GetDescription_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserCapability_GetIsEnabled(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserCapability_GetIsEnabled", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserCapability_GetName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserCapability_GetName", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserCapability_GetName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserCapability_GetName_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserCapability_GetReasonCode(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserCapability_GetReasonCode", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserCapability_GetReasonCode_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserCapability_GetReasonCode_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStoreUpdateResponse_GetSuccess(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserDataStoreUpdateResponse_GetSuccess", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStore_PrivateDeleteEntryByKey(
        userID: u64,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserDataStore_PrivateDeleteEntryByKey", (userID, key))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStore_PrivateDeleteEntryByKey_Native(
        userID: u64,
        key: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserDataStore_PrivateDeleteEntryByKey_Native", (userID, key))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStore_PrivateGetEntries(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserDataStore_PrivateGetEntries", (userID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStore_PrivateGetEntryByKey(
        userID: u64,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserDataStore_PrivateGetEntryByKey", (userID, key))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStore_PrivateGetEntryByKey_Native(
        userID: u64,
        key: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserDataStore_PrivateGetEntryByKey_Native", (userID, key))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStore_PrivateWriteEntry(
        userID: u64,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserDataStore_PrivateWriteEntry", (userID, key, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStore_PrivateWriteEntry_Native(
        userID: u64,
        key: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserDataStore_PrivateWriteEntry_Native", (userID, key, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStore_PublicDeleteEntryByKey(
        userID: u64,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserDataStore_PublicDeleteEntryByKey", (userID, key))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStore_PublicDeleteEntryByKey_Native(
        userID: u64,
        key: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserDataStore_PublicDeleteEntryByKey_Native", (userID, key))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStore_PublicGetEntries(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserDataStore_PublicGetEntries", (userID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStore_PublicGetEntryByKey(
        userID: u64,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserDataStore_PublicGetEntryByKey", (userID, key))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStore_PublicGetEntryByKey_Native(
        userID: u64,
        key: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserDataStore_PublicGetEntryByKey_Native", (userID, key))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStore_PublicWriteEntry(
        userID: u64,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserDataStore_PublicWriteEntry", (userID, key, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserDataStore_PublicWriteEntry_Native(
        userID: u64,
        key: crate::System::IntPtr,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserDataStore_PublicWriteEntry_Native", (userID, key, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserOptions_AddServiceProvider(
        handle: crate::System::IntPtr,
        value: crate::Oculus::Platform::ServiceProvider,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserOptions_AddServiceProvider", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserOptions_ClearServiceProviders(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserOptions_ClearServiceProviders", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserOptions_Create() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserOptions_Create", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserOptions_Destroy(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserOptions_Destroy", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserOptions_SetMaxUsers(
        handle: crate::System::IntPtr,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserOptions_SetMaxUsers", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserOptions_SetTimeWindow(
        handle: crate::System::IntPtr,
        value: crate::Oculus::Platform::TimeWindow,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserOptions_SetTimeWindow", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserProof_GetNonce(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserProof_GetNonce", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserProof_GetNonce_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserProof_GetNonce_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserReportID_GetDidCancel(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserReportID_GetDidCancel", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_UserReportID_GetID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_UserReportID_GetID", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_CancelRecordingForReportFlow(
        recordingUUID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_CancelRecordingForReportFlow", (recordingUUID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_CancelRecordingForReportFlow_Native(
        recordingUUID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_CancelRecordingForReportFlow_Native", (recordingUUID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_Get(userID: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_Get", (userID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetAccessToken() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetAccessToken", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetBlockedUsers() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetBlockedUsers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetDisplayName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetDisplayName", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetDisplayName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetDisplayName_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetID", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetImageUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetImageUrl", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetImageUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetImageUrl_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetLinkedAccounts(
        userOptions: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetLinkedAccounts", (userOptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetLoggedInUser() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetLoggedInUser", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetLoggedInUserFriends() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetLoggedInUserFriends", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetOculusID(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetOculusID", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetOculusID_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetOculusID_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetOrgScopedID(userID: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetOrgScopedID", (userID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetPresence(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetPresence", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetPresenceDeeplinkMessage(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetPresenceDeeplinkMessage", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetPresenceDeeplinkMessage_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetPresenceDeeplinkMessage_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetPresenceDestinationApiName(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetPresenceDestinationApiName", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetPresenceDestinationApiName_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetPresenceDestinationApiName_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetPresenceLobbySessionId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetPresenceLobbySessionId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetPresenceLobbySessionId_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetPresenceLobbySessionId_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetPresenceMatchSessionId(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetPresenceMatchSessionId", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetPresenceMatchSessionId_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetPresenceMatchSessionId_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetPresenceStatus(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::UserPresenceStatus> {
        let __cordl_ret: crate::Oculus::Platform::UserPresenceStatus = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetPresenceStatus", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetPresence_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetPresence_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetSdkAccounts() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetSdkAccounts", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetSmallImageUrl(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetSmallImageUrl", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetSmallImageUrl_Native(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetSmallImageUrl_Native", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetUserCapabilities() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetUserCapabilities", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_GetUserProof() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_GetUserProof", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_LaunchBlockFlow(userID: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_LaunchBlockFlow", (userID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_LaunchFriendRequestFlow(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_LaunchFriendRequestFlow", (userID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_LaunchReportFlow(userID: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_LaunchReportFlow", (userID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_LaunchReportFlow2(
        optionalUserID: u64,
        abuseReportOptions: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_LaunchReportFlow2", (optionalUserID, abuseReportOptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_LaunchUnblockFlow(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_LaunchUnblockFlow", (userID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_NewEntitledTestUser() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_NewEntitledTestUser", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_NewTestUser() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_NewTestUser", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_NewTestUserFriends() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_NewTestUserFriends", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_StartRecordingForReportFlow() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_User_StartRecordingForReportFlow", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_StopRecordingAndLaunchReportFlow(
        optionalUserID: u64,
        optionalRecordingUUID: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_User_StopRecordingAndLaunchReportFlow",
                (optionalUserID, optionalRecordingUUID),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_StopRecordingAndLaunchReportFlow2(
        optionalUserID: u64,
        optionalRecordingUUID: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        abuseReportOptions: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_User_StopRecordingAndLaunchReportFlow2",
                (optionalUserID, optionalRecordingUUID, abuseReportOptions),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_StopRecordingAndLaunchReportFlow2_Native(
        optionalUserID: u64,
        optionalRecordingUUID: crate::System::IntPtr,
        abuseReportOptions: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_User_StopRecordingAndLaunchReportFlow2_Native",
                (optionalUserID, optionalRecordingUUID, abuseReportOptions),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_StopRecordingAndLaunchReportFlow_Native(
        optionalUserID: u64,
        optionalRecordingUUID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_User_StopRecordingAndLaunchReportFlow_Native",
                (optionalUserID, optionalRecordingUUID),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_TestUserCreateDeviceManifest(
        deviceID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        appIDs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        numAppIDs: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_User_TestUserCreateDeviceManifest",
                (deviceID, appIDs, numAppIDs),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_User_TestUserCreateDeviceManifest_Native(
        deviceID: crate::System::IntPtr,
        appIDs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        numAppIDs: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_User_TestUserCreateDeviceManifest_Native",
                (deviceID, appIDs, numAppIDs),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_VoipDecoder_Decode_UIntPtr1(
        obj: crate::System::IntPtr,
        compressedData: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        compressedSize: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_VoipDecoder_Decode", (obj, compressedData, compressedSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_VoipDecoder_Decode_u64_0(
        obj: crate::System::IntPtr,
        compressedData: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        compressedSize: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_VoipDecoder_Decode", (obj, compressedData, compressedSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_VoipDecoder_GetDecodedPCM(
        obj: crate::System::IntPtr,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        outputBufferSize: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_VoipDecoder_GetDecodedPCM",
                (obj, outputBuffer, outputBufferSize),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_VoipEncoder_AddPCM(
        obj: crate::System::IntPtr,
        inputData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        inputSize: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_VoipEncoder_AddPCM", (obj, inputData, inputSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_VoipEncoder_GetCompressedData(
        obj: crate::System::IntPtr,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        intputSize: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_VoipEncoder_GetCompressedData",
                (obj, outputBuffer, intputSize),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_VoipEncoder_GetCompressedDataSize(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_VoipEncoder_GetCompressedDataSize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_VoipOptions_Create() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_VoipOptions_Create", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_VoipOptions_Destroy(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_VoipOptions_Destroy", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_VoipOptions_SetBitrateForNewConnections(
        handle: crate::System::IntPtr,
        value: crate::Oculus::Platform::VoipBitrate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_VoipOptions_SetBitrateForNewConnections", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_VoipOptions_SetCreateNewConnectionUseDtx(
        handle: crate::System::IntPtr,
        value: crate::Oculus::Platform::VoipDtxState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_VoipOptions_SetCreateNewConnectionUseDtx", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_Accept(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Voip_Accept", (userID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_CreateDecoder() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Voip_CreateDecoder", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_CreateEncoder() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Voip_CreateEncoder", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_DestroyDecoder(
        decoder: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Voip_DestroyDecoder", (decoder))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_DestroyEncoder(
        encoder: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Voip_DestroyEncoder", (encoder))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_GetIsConnectionUsingDtx(
        peerID: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::VoipDtxState> {
        let __cordl_ret: crate::Oculus::Platform::VoipDtxState = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Voip_GetIsConnectionUsingDtx", (peerID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_GetLocalBitrate(
        peerID: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::VoipBitrate> {
        let __cordl_ret: crate::Oculus::Platform::VoipBitrate = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Voip_GetLocalBitrate", (peerID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_GetMicrophoneAvailability() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Voip_GetMicrophoneAvailability", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_GetOutputBufferMaxSize() -> quest_hook::libil2cpp::Result<
        crate::System::UIntPtr,
    > {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Voip_GetOutputBufferMaxSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_GetPCM(
        senderID: u64,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
        outputBufferNumElements: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_Voip_GetPCM",
                (senderID, outputBuffer, outputBufferNumElements),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_GetPCMFloat(
        senderID: u64,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        outputBufferNumElements: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_Voip_GetPCMFloat",
                (senderID, outputBuffer, outputBufferNumElements),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_GetPCMSize(
        senderID: u64,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Voip_GetPCMSize", (senderID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_GetPCMWithTimestamp(
        senderID: u64,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
        outputBufferNumElements: crate::System::UIntPtr,
        timestamp: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_Voip_GetPCMWithTimestamp",
                (senderID, outputBuffer, outputBufferNumElements, timestamp),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_GetPCMWithTimestampFloat(
        senderID: u64,
        outputBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        outputBufferNumElements: crate::System::UIntPtr,
        timestamp: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::UIntPtr> {
        let __cordl_ret: crate::System::UIntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_Voip_GetPCMWithTimestampFloat",
                (senderID, outputBuffer, outputBufferNumElements, timestamp),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_GetRemoteBitrate(
        peerID: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::VoipBitrate> {
        let __cordl_ret: crate::Oculus::Platform::VoipBitrate = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Voip_GetRemoteBitrate", (peerID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_GetSyncTimestamp(userID: u64) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Voip_GetSyncTimestamp", (userID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_GetSyncTimestampDifference(
        lhs: u32,
        rhs: u32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Voip_GetSyncTimestampDifference", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_GetSystemVoipMicrophoneMuted() -> quest_hook::libil2cpp::Result<
        crate::Oculus::Platform::VoipMuteState,
    > {
        let __cordl_ret: crate::Oculus::Platform::VoipMuteState = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Voip_GetSystemVoipMicrophoneMuted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_GetSystemVoipStatus() -> quest_hook::libil2cpp::Result<
        crate::Oculus::Platform::SystemVoipStatus,
    > {
        let __cordl_ret: crate::Oculus::Platform::SystemVoipStatus = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Voip_GetSystemVoipStatus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_ReportAppVoipSessions(
        sessionIDs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Voip_ReportAppVoipSessions", (sessionIDs))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_SetMicrophoneFilterCallback(
        cb: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::CAPI_FilterCallback>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Voip_SetMicrophoneFilterCallback", (cb))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_SetMicrophoneFilterCallbackWithFixedSizeBuffer(
        cb: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::CAPI_FilterCallback>,
        bufferSizeElements: crate::System::UIntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovr_Voip_SetMicrophoneFilterCallbackWithFixedSizeBuffer",
                (cb, bufferSizeElements),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_SetMicrophoneMuted(
        state: crate::Oculus::Platform::VoipMuteState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Voip_SetMicrophoneMuted", (state))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_SetNewConnectionOptions(
        voipOptions: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Voip_SetNewConnectionOptions", (voipOptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_SetOutputSampleRate(
        rate: crate::Oculus::Platform::VoipSampleRate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Voip_SetOutputSampleRate", (rate))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_SetSystemVoipMicrophoneMuted(
        muted: crate::Oculus::Platform::VoipMuteState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Voip_SetSystemVoipMicrophoneMuted", (muted))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_SetSystemVoipPassthrough(
        passthrough: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Voip_SetSystemVoipPassthrough", (passthrough))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_SetSystemVoipSuppressed(
        suppressed: bool,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Voip_SetSystemVoipSuppressed", (suppressed))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_Start(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Voip_Start", (userID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovr_Voip_Stop(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovr_Voip_Stop", (userID))?;
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
    const CLASS_NAME: &'static str = "FilterCallback";
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (pcmData, pcmDataLength, frequency, numChannels, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (pcmData, pcmDataLength, frequency, numChannels))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
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
    const CLASS_NAME: &'static str = "OculusInitParams";
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
    const CLASS_NAME: &'static str = "ovrKeyValuePair";
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
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (key, value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f64_2(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (key, value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_1(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (key, value),
        )?;
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
    const CLASS_NAME: &'static str = "ovrNetSyncVec3";
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
