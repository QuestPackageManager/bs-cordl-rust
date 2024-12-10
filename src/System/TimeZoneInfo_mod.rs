#[cfg(feature = "System+TimeZoneInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeZoneInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _id: *mut quest_hook::libil2cpp::Il2CppString,
    pub _displayName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _standardDisplayName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _daylightDisplayName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _baseUtcOffset: crate::System::TimeSpan,
    pub _supportsDaylightSavingTime: bool,
    pub _adjustmentRules: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::TimeZoneInfo_AdjustmentRule,
    >,
}
#[cfg(feature = "System+TimeZoneInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::TimeZoneInfo => "System"."TimeZoneInfo"
);
#[cfg(feature = "System+TimeZoneInfo")]
impl std::ops::Deref for crate::System::TimeZoneInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+TimeZoneInfo")]
impl std::ops::DerefMut for crate::System::TimeZoneInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+TimeZoneInfo")]
impl crate::System::TimeZoneInfo {
    #[cfg(feature = "System+TimeZoneInfo+AdjustmentRule")]
    pub type AdjustmentRule = crate::System::TimeZoneInfo_AdjustmentRule;
    #[cfg(feature = "System+TimeZoneInfo+CachedData")]
    pub type CachedData = crate::System::TimeZoneInfo_CachedData;
    #[cfg(feature = "System+TimeZoneInfo+TZVersion")]
    pub type TZVersion = crate::System::TimeZoneInfo_TZVersion;
    #[cfg(feature = "System+TimeZoneInfo+TZifHead")]
    pub type TZifHead = crate::System::TimeZoneInfo_TZifHead;
    #[cfg(feature = "System+TimeZoneInfo+TZifType")]
    pub type TZifType = crate::System::TimeZoneInfo_TZifType;
    #[cfg(feature = "System+TimeZoneInfo+TransitionTime")]
    pub type TransitionTime = crate::System::TimeZoneInfo_TransitionTime;
    #[cfg(feature = "System+TimeZoneInfo+__c")]
    pub type __c = crate::System::TimeZoneInfo___c;
    #[cfg(feature = "System+TimeZoneInfo+__c__DisplayClass16_0")]
    pub type __c__DisplayClass16_0 = crate::System::TimeZoneInfo___c__DisplayClass16_0;
    pub fn CompareAdjustmentRuleToDateTime(
        &mut self,
        rule: quest_hook::libil2cpp::Gc<crate::System::TimeZoneInfo_AdjustmentRule>,
        previousRule: quest_hook::libil2cpp::Gc<
            crate::System::TimeZoneInfo_AdjustmentRule,
        >,
        dateTime: crate::System::DateTime,
        dateOnly: crate::System::DateTime,
        dateTimeisUtc: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "CompareAdjustmentRuleToDateTime",
                (rule, previousRule, dateTime, dateOnly, dateTimeisUtc),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertFromUtc(
        &mut self,
        dateTime: crate::System::DateTime,
        daylightDelta: crate::System::TimeSpan,
        baseUtcOffsetDelta: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("ConvertFromUtc", (dateTime, daylightDelta, baseUtcOffsetDelta))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToFromUtc(
        &mut self,
        dateTime: crate::System::DateTime,
        daylightDelta: crate::System::TimeSpan,
        baseUtcOffsetDelta: crate::System::TimeSpan,
        convertToUtc: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke(
                "ConvertToFromUtc",
                (dateTime, daylightDelta, baseUtcOffsetDelta, convertToUtc),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToUtc(
        &mut self,
        dateTime: crate::System::DateTime,
        daylightDelta: crate::System::TimeSpan,
        baseUtcOffsetDelta: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("ConvertToUtc", (dateTime, daylightDelta, baseUtcOffsetDelta))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_TimeZoneInfo0(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::System::TimeZoneInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAdjustmentRuleForTime_ByRefMut0(
        &mut self,
        dateTime: crate::System::DateTime,
        ruleIndex: quest_hook::libil2cpp::ByRefMut<crate::System::Nullable_1<i32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::TimeZoneInfo_AdjustmentRule>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::TimeZoneInfo_AdjustmentRule,
        > = __cordl_object.invoke("GetAdjustmentRuleForTime", (dateTime, ruleIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAdjustmentRuleForTime__cordl_bool_ByRefMut1(
        &mut self,
        dateTime: crate::System::DateTime,
        dateTimeisUtc: bool,
        ruleIndex: quest_hook::libil2cpp::ByRefMut<crate::System::Nullable_1<i32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::TimeZoneInfo_AdjustmentRule>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::TimeZoneInfo_AdjustmentRule,
        > = __cordl_object
            .invoke("GetAdjustmentRuleForTime", (dateTime, dateTimeisUtc, ruleIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAdjustmentRules(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::TimeZoneInfo_AdjustmentRule,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::TimeZoneInfo_AdjustmentRule,
            >,
        > = __cordl_object.invoke("GetAdjustmentRules", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDaylightSavingsEndOffsetFromUtc(
        &mut self,
        baseUtcOffset: crate::System::TimeSpan,
        rule: quest_hook::libil2cpp::Gc<crate::System::TimeZoneInfo_AdjustmentRule>,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::TimeSpan = __cordl_object
            .invoke("GetDaylightSavingsEndOffsetFromUtc", (baseUtcOffset, rule))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDaylightSavingsStartOffsetFromUtc(
        &mut self,
        baseUtcOffset: crate::System::TimeSpan,
        rule: quest_hook::libil2cpp::Gc<crate::System::TimeZoneInfo_AdjustmentRule>,
        ruleIndex: crate::System::Nullable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::TimeSpan = __cordl_object
            .invoke(
                "GetDaylightSavingsStartOffsetFromUtc",
                (baseUtcOffset, rule, ruleIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDaylightTime(
        &mut self,
        year: i32,
        rule: quest_hook::libil2cpp::Gc<crate::System::TimeZoneInfo_AdjustmentRule>,
        ruleIndex: crate::System::Nullable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Globalization::DaylightTimeStruct,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Globalization::DaylightTimeStruct = __cordl_object
            .invoke("GetDaylightTime", (year, rule, ruleIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPreviousAdjustmentRule(
        &mut self,
        rule: quest_hook::libil2cpp::Gc<crate::System::TimeZoneInfo_AdjustmentRule>,
        ruleIndex: crate::System::Nullable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::TimeZoneInfo_AdjustmentRule>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::TimeZoneInfo_AdjustmentRule,
        > = __cordl_object.invoke("GetPreviousAdjustmentRule", (rule, ruleIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUtcOffset_DateTime0(
        &mut self,
        dateTime: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::TimeSpan = __cordl_object
            .invoke("GetUtcOffset", (dateTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUtcOffset_TimeZoneInfoOptions1(
        &mut self,
        dateTime: crate::System::DateTime,
        flags: crate::System::TimeZoneInfoOptions,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::TimeSpan = __cordl_object
            .invoke("GetUtcOffset", (dateTime, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUtcOffset_TimeZoneInfoOptions_TimeZoneInfo_CachedData2(
        &mut self,
        dateTime: crate::System::DateTime,
        flags: crate::System::TimeZoneInfoOptions,
        cachedData: quest_hook::libil2cpp::Gc<crate::System::TimeZoneInfo_CachedData>,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::TimeSpan = __cordl_object
            .invoke("GetUtcOffset", (dateTime, flags, cachedData))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasSameRules(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::System::TimeZoneInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasSameRules", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_3() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppArray_Il2CppString__cordl_bool0(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dstDisabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (data, id, dstDisabled))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_TimeSpan_Il2CppString_Il2CppString_Il2CppString_Il2CppArray__cordl_bool1(
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        baseUtcOffset: crate::System::TimeSpan,
        displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        standardDisplayName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        daylightDisplayName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        adjustmentRules: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::TimeZoneInfo_AdjustmentRule,
            >,
        >,
        disableDaylightSavingTime: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    id,
                    baseUtcOffset,
                    displayName,
                    standardDisplayName,
                    daylightDisplayName,
                    adjustmentRules,
                    disableDaylightSavingTime,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_SerializationInfo_StreamingContext2(
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, context))?;
        Ok(__cordl_object.into())
    }
    pub fn System_Runtime_Serialization_IDeserializationCallback_OnDeserialization(
        &mut self,
        sender: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Runtime.Serialization.IDeserializationCallback.OnDeserialization",
                (sender),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Runtime_Serialization_ISerializable_GetObjectData(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Runtime.Serialization.ISerializable.GetObjectData",
                (info, context),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray_Il2CppString__cordl_bool0(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dstDisabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (data, id, dstDisabled))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_TimeSpan_Il2CppString_Il2CppString_Il2CppString_Il2CppArray__cordl_bool1(
        &mut self,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        baseUtcOffset: crate::System::TimeSpan,
        displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        standardDisplayName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        daylightDisplayName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        adjustmentRules: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::TimeZoneInfo_AdjustmentRule,
            >,
        >,
        disableDaylightSavingTime: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    id,
                    baseUtcOffset,
                    displayName,
                    standardDisplayName,
                    daylightDisplayName,
                    adjustmentRules,
                    disableDaylightSavingTime,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SerializationInfo_StreamingContext2(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BaseUtcOffset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::TimeSpan = __cordl_object
            .invoke("get_BaseUtcOffset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DaylightName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_DaylightName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DisplayName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_DisplayName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_StandardName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_StandardName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SupportsDaylightSavingTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_SupportsDaylightSavingTime", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+TimeZoneInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::TimeZoneInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+TimeZoneInfo")]
impl AsRef<crate::System::IEquatable_1<*mut crate::System::TimeZoneInfo>>
for crate::System::TimeZoneInfo {
    fn as_ref(&self) -> &crate::System::IEquatable_1<*mut crate::System::TimeZoneInfo> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+TimeZoneInfo")]
impl AsMut<crate::System::IEquatable_1<*mut crate::System::TimeZoneInfo>>
for crate::System::TimeZoneInfo {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<*mut crate::System::TimeZoneInfo> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+TimeZoneInfo")]
impl AsRef<crate::System::Runtime::Serialization::IDeserializationCallback>
for crate::System::TimeZoneInfo {
    fn as_ref(
        &self,
    ) -> &crate::System::Runtime::Serialization::IDeserializationCallback {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+TimeZoneInfo")]
impl AsMut<crate::System::Runtime::Serialization::IDeserializationCallback>
for crate::System::TimeZoneInfo {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Serialization::IDeserializationCallback {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+TimeZoneInfo")]
impl AsRef<crate::System::Runtime::Serialization::ISerializable>
for crate::System::TimeZoneInfo {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+TimeZoneInfo")]
impl AsMut<crate::System::Runtime::Serialization::ISerializable>
for crate::System::TimeZoneInfo {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+TimeZoneInfo+AdjustmentRule")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeZoneInfo_AdjustmentRule {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _dateStart: crate::System::DateTime,
    pub _dateEnd: crate::System::DateTime,
    pub _daylightDelta: crate::System::TimeSpan,
    pub _daylightTransitionStart: crate::System::TimeZoneInfo_TransitionTime,
    pub _daylightTransitionEnd: crate::System::TimeZoneInfo_TransitionTime,
    pub _baseUtcOffsetDelta: crate::System::TimeSpan,
    pub _noDaylightTransitions: bool,
}
#[cfg(feature = "System+TimeZoneInfo+AdjustmentRule")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::TimeZoneInfo_AdjustmentRule => "System"
    ."TimeZoneInfo/AdjustmentRule"
);
#[cfg(feature = "System+TimeZoneInfo+AdjustmentRule")]
impl std::ops::Deref for crate::System::TimeZoneInfo_AdjustmentRule {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+TimeZoneInfo+AdjustmentRule")]
impl std::ops::DerefMut for crate::System::TimeZoneInfo_AdjustmentRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+TimeZoneInfo+AdjustmentRule")]
impl crate::System::TimeZoneInfo_AdjustmentRule {
    pub fn Equals(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::System::TimeZoneInfo_AdjustmentRule>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEndDateMarkerForEndOfYear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsEndDateMarkerForEndOfYear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsStartDateMarkerForBeginningOfYear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsStartDateMarkerForBeginningOfYear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_2() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_DateTime_DateTime_TimeSpan_TimeZoneInfo_TransitionTime_TimeZoneInfo_TransitionTime_TimeSpan__cordl_bool0(
        dateStart: crate::System::DateTime,
        dateEnd: crate::System::DateTime,
        daylightDelta: crate::System::TimeSpan,
        daylightTransitionStart: crate::System::TimeZoneInfo_TransitionTime,
        daylightTransitionEnd: crate::System::TimeZoneInfo_TransitionTime,
        baseUtcOffsetDelta: crate::System::TimeSpan,
        noDaylightTransitions: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    dateStart,
                    dateEnd,
                    daylightDelta,
                    daylightTransitionStart,
                    daylightTransitionEnd,
                    baseUtcOffsetDelta,
                    noDaylightTransitions,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_SerializationInfo_StreamingContext1(
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, context))?;
        Ok(__cordl_object.into())
    }
    pub fn System_Runtime_Serialization_IDeserializationCallback_OnDeserialization(
        &mut self,
        sender: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Runtime.Serialization.IDeserializationCallback.OnDeserialization",
                (sender),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Runtime_Serialization_ISerializable_GetObjectData(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Runtime.Serialization.ISerializable.GetObjectData",
                (info, context),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DateTime_DateTime_TimeSpan_TimeZoneInfo_TransitionTime_TimeZoneInfo_TransitionTime_TimeSpan__cordl_bool0(
        &mut self,
        dateStart: crate::System::DateTime,
        dateEnd: crate::System::DateTime,
        daylightDelta: crate::System::TimeSpan,
        daylightTransitionStart: crate::System::TimeZoneInfo_TransitionTime,
        daylightTransitionEnd: crate::System::TimeZoneInfo_TransitionTime,
        baseUtcOffsetDelta: crate::System::TimeSpan,
        noDaylightTransitions: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    dateStart,
                    dateEnd,
                    daylightDelta,
                    daylightTransitionStart,
                    daylightTransitionEnd,
                    baseUtcOffsetDelta,
                    noDaylightTransitions,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SerializationInfo_StreamingContext1(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BaseUtcOffsetDelta(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::TimeSpan = __cordl_object
            .invoke("get_BaseUtcOffsetDelta", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DateEnd(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("get_DateEnd", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DateStart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("get_DateStart", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DaylightDelta(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::TimeSpan = __cordl_object
            .invoke("get_DaylightDelta", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DaylightTransitionEnd(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeZoneInfo_TransitionTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::TimeZoneInfo_TransitionTime = __cordl_object
            .invoke("get_DaylightTransitionEnd", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DaylightTransitionStart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeZoneInfo_TransitionTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::TimeZoneInfo_TransitionTime = __cordl_object
            .invoke("get_DaylightTransitionStart", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasDaylightSaving(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasDaylightSaving", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NoDaylightTransitions(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_NoDaylightTransitions", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+TimeZoneInfo+AdjustmentRule")]
impl quest_hook::libil2cpp::ObjectType for crate::System::TimeZoneInfo_AdjustmentRule {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+TimeZoneInfo+AdjustmentRule")]
impl AsRef<crate::System::IEquatable_1<*mut crate::System::TimeZoneInfo_AdjustmentRule>>
for crate::System::TimeZoneInfo_AdjustmentRule {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<*mut crate::System::TimeZoneInfo_AdjustmentRule> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+TimeZoneInfo+AdjustmentRule")]
impl AsMut<crate::System::IEquatable_1<*mut crate::System::TimeZoneInfo_AdjustmentRule>>
for crate::System::TimeZoneInfo_AdjustmentRule {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        *mut crate::System::TimeZoneInfo_AdjustmentRule,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+TimeZoneInfo+AdjustmentRule")]
impl AsRef<crate::System::Runtime::Serialization::IDeserializationCallback>
for crate::System::TimeZoneInfo_AdjustmentRule {
    fn as_ref(
        &self,
    ) -> &crate::System::Runtime::Serialization::IDeserializationCallback {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+TimeZoneInfo+AdjustmentRule")]
impl AsMut<crate::System::Runtime::Serialization::IDeserializationCallback>
for crate::System::TimeZoneInfo_AdjustmentRule {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Serialization::IDeserializationCallback {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+TimeZoneInfo+AdjustmentRule")]
impl AsRef<crate::System::Runtime::Serialization::ISerializable>
for crate::System::TimeZoneInfo_AdjustmentRule {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+TimeZoneInfo+AdjustmentRule")]
impl AsMut<crate::System::Runtime::Serialization::ISerializable>
for crate::System::TimeZoneInfo_AdjustmentRule {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+TimeZoneInfo+CachedData")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeZoneInfo_CachedData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _localTimeZone: *mut crate::System::TimeZoneInfo,
}
#[cfg(feature = "System+TimeZoneInfo+CachedData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::TimeZoneInfo_CachedData => "System"
    ."TimeZoneInfo/CachedData"
);
#[cfg(feature = "System+TimeZoneInfo+CachedData")]
impl std::ops::Deref for crate::System::TimeZoneInfo_CachedData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+TimeZoneInfo+CachedData")]
impl std::ops::DerefMut for crate::System::TimeZoneInfo_CachedData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+TimeZoneInfo+CachedData")]
impl crate::System::TimeZoneInfo_CachedData {
    pub fn CreateLocal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::TimeZoneInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::TimeZoneInfo> = __cordl_object
            .invoke("CreateLocal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCorrespondingKind(
        &mut self,
        timeZone: quest_hook::libil2cpp::Gc<crate::System::TimeZoneInfo>,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTimeKind> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTimeKind = __cordl_object
            .invoke("GetCorrespondingKind", (timeZone))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_Local(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::TimeZoneInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::TimeZoneInfo> = __cordl_object
            .invoke("get_Local", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+TimeZoneInfo+CachedData")]
impl quest_hook::libil2cpp::ObjectType for crate::System::TimeZoneInfo_CachedData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+TimeZoneInfo+TZVersion")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeZoneInfo_TZVersion {
    V1 = 0u8,
    V2 = 1u8,
    V3 = 2u8,
}
#[cfg(feature = "System+TimeZoneInfo+TZVersion")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::TimeZoneInfo_TZVersion => "System"
    ."TimeZoneInfo/TZVersion"
);
#[cfg(feature = "System+TimeZoneInfo+TZifHead")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TimeZoneInfo_TZifHead {
    pub Magic: u32,
    pub Version: crate::System::TimeZoneInfo_TZVersion,
    pub IsGmtCount: u32,
    pub IsStdCount: u32,
    pub LeapCount: u32,
    pub TimeCount: u32,
    pub TypeCount: u32,
    pub CharCount: u32,
}
#[cfg(feature = "System+TimeZoneInfo+TZifHead")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::TimeZoneInfo_TZifHead => "System"
    ."TimeZoneInfo/TZifHead"
);
#[cfg(feature = "System+TimeZoneInfo+TZifHead")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::TimeZoneInfo_TZifHead {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+TimeZoneInfo+TZifHead")]
impl crate::System::TimeZoneInfo_TZifHead {
    pub fn _ctor(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (data, index),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+TimeZoneInfo+TZifType")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TimeZoneInfo_TZifType {
    pub UtcOffset: crate::System::TimeSpan,
    pub IsDst: bool,
    pub AbbreviationIndex: u8,
}
#[cfg(feature = "System+TimeZoneInfo+TZifType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::TimeZoneInfo_TZifType => "System"
    ."TimeZoneInfo/TZifType"
);
#[cfg(feature = "System+TimeZoneInfo+TZifType")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::TimeZoneInfo_TZifType {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+TimeZoneInfo+TZifType")]
impl crate::System::TimeZoneInfo_TZifType {
    pub fn _ctor(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (data, index),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+TimeZoneInfo+TransitionTime")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TimeZoneInfo_TransitionTime {
    pub _timeOfDay: crate::System::DateTime,
    pub _month: u8,
    pub _week: u8,
    pub _day: u8,
    pub _dayOfWeek: crate::System::DayOfWeek,
    pub _isFixedDateRule: bool,
}
#[cfg(feature = "System+TimeZoneInfo+TransitionTime")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::TimeZoneInfo_TransitionTime => "System"
    ."TimeZoneInfo/TransitionTime"
);
#[cfg(feature = "System+TimeZoneInfo+TransitionTime")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::TimeZoneInfo_TransitionTime {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+TimeZoneInfo+TransitionTime")]
impl crate::System::TimeZoneInfo_TransitionTime {
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_TimeZoneInfo_TransitionTime1(
        &mut self,
        other: crate::System::TimeZoneInfo_TransitionTime,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Runtime_Serialization_IDeserializationCallback_OnDeserialization(
        &mut self,
        sender: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Runtime.Serialization.IDeserializationCallback.OnDeserialization",
            (sender),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Runtime_Serialization_ISerializable_GetObjectData(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Runtime.Serialization.ISerializable.GetObjectData",
            (info, context),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DateTime_i32_i32_i32_DayOfWeek__cordl_bool0(
        &mut self,
        timeOfDay: crate::System::DateTime,
        month: i32,
        week: i32,
        day: i32,
        dayOfWeek: crate::System::DayOfWeek,
        isFixedDateRule: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (timeOfDay, month, week, day, dayOfWeek, isFixedDateRule),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SerializationInfo_StreamingContext1(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (info, context),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Day(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Day",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DayOfWeek(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DayOfWeek> {
        let __cordl_ret: crate::System::DayOfWeek = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_DayOfWeek",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsFixedDateRule(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsFixedDateRule",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Month(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Month",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TimeOfDay(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_TimeOfDay",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Week(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Week",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+TimeZoneInfo+TransitionTime")]
impl AsRef<crate::System::IEquatable_1<crate::System::TimeZoneInfo_TransitionTime>>
for crate::System::TimeZoneInfo_TransitionTime {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::System::TimeZoneInfo_TransitionTime> {
        todo!()
    }
}
#[cfg(feature = "System+TimeZoneInfo+TransitionTime")]
impl AsMut<crate::System::IEquatable_1<crate::System::TimeZoneInfo_TransitionTime>>
for crate::System::TimeZoneInfo_TransitionTime {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::System::TimeZoneInfo_TransitionTime> {
        todo!()
    }
}
#[cfg(feature = "System+TimeZoneInfo+TransitionTime")]
impl AsRef<crate::System::Runtime::Serialization::IDeserializationCallback>
for crate::System::TimeZoneInfo_TransitionTime {
    fn as_ref(
        &self,
    ) -> &crate::System::Runtime::Serialization::IDeserializationCallback {
        todo!()
    }
}
#[cfg(feature = "System+TimeZoneInfo+TransitionTime")]
impl AsMut<crate::System::Runtime::Serialization::IDeserializationCallback>
for crate::System::TimeZoneInfo_TransitionTime {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Serialization::IDeserializationCallback {
        todo!()
    }
}
#[cfg(feature = "System+TimeZoneInfo+TransitionTime")]
impl AsRef<crate::System::Runtime::Serialization::ISerializable>
for crate::System::TimeZoneInfo_TransitionTime {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::ISerializable {
        todo!()
    }
}
#[cfg(feature = "System+TimeZoneInfo+TransitionTime")]
impl AsMut<crate::System::Runtime::Serialization::ISerializable>
for crate::System::TimeZoneInfo_TransitionTime {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Serialization::ISerializable {
        todo!()
    }
}
