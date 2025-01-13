#[cfg(feature = "System+Globalization+CultureInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct CultureInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_isReadOnly: bool,
    pub cultureID: i32,
    pub parent_lcid: i32,
    pub datetime_index: i32,
    pub number_index: i32,
    pub default_calendar_type: i32,
    pub m_useUserOverride: bool,
    pub numInfo: quest_hook::libil2cpp::Gc<
        crate::System::Globalization::NumberFormatInfo,
    >,
    pub dateTimeInfo: quest_hook::libil2cpp::Gc<
        crate::System::Globalization::DateTimeFormatInfo,
    >,
    pub textInfo: quest_hook::libil2cpp::Gc<crate::System::Globalization::TextInfo>,
    pub m_name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub englishname: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub nativename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub iso3lang: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub iso2lang: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub win3lang: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub territory: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub native_calendar_names: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub compareInfo: quest_hook::libil2cpp::Gc<
        crate::System::Globalization::CompareInfo,
    >,
    pub textinfo_data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_dataItem: i32,
    pub calendar: quest_hook::libil2cpp::Gc<crate::System::Globalization::Calendar>,
    pub parent_culture: quest_hook::libil2cpp::Gc<
        crate::System::Globalization::CultureInfo,
    >,
    pub constructed: bool,
    pub cached_serialized_form: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub m_cultureData: quest_hook::libil2cpp::Gc<
        crate::System::Globalization::CultureData,
    >,
    pub m_isInherited: bool,
}
#[cfg(feature = "System+Globalization+CultureInfo")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Globalization::CultureInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "CultureInfo";
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
#[cfg(feature = "System+Globalization+CultureInfo")]
impl std::ops::Deref for crate::System::Globalization::CultureInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+CultureInfo")]
impl std::ops::DerefMut for crate::System::Globalization::CultureInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+CultureInfo")]
impl crate::System::Globalization::CultureInfo {
    pub const CalendarTypeBits: i32 = 8i32;
    pub const InvariantCultureId: i32 = 127i32;
    pub const LOCALE_INVARIANT: i32 = 127i32;
    pub const MSG_READONLY: &'static str = "This instance is read only";
    #[cfg(feature = "System+Globalization+CultureInfo+Data")]
    pub type Data = crate::System::Globalization::CultureInfo_Data;
    #[cfg(feature = "System+Globalization+CultureInfo+OnCultureInfoChangedDelegate")]
    pub type OnCultureInfoChangedDelegate = crate::System::Globalization::CultureInfo_OnCultureInfoChangedDelegate;
    pub fn CheckNeutral(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckNeutral", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Clone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Construct(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Construct", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ConstructCurrentCulture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConstructCurrentCulture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ConstructCurrentUICulture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConstructCurrentUICulture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ConstructInvariant(
        &mut self,
        read_only: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConstructInvariant", (read_only))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConstructLocaleFromName(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ConstructLocaleFromName", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCalendar(
        calendarType: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::Calendar>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::Calendar,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateCalendar", (calendarType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCulture(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        reference: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateCulture", (name, reference))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateNotFoundException(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateNotFoundException", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSpecificCulture(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateSpecificCulture", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSpecificCultureFromNeutral(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateSpecificCultureFromNeutral", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateTextInfo(
        &mut self,
        readOnly: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::TextInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::TextInfo,
        > = __cordl_object.invoke("CreateTextInfo", (readOnly))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCultureInfoForUserPreferredLanguageInAppX() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCultureInfoForUserPreferredLanguageInAppX", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCultureInfo_Il2CppString1(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCultureInfo", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCultureInfo_i32_0(
        culture: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCultureInfo", (culture))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCultures(
        types: crate::System::Globalization::CultureTypes,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCultures", (types))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFormat(
        &mut self,
        formatType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetFormat", (formatType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTextInfoData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Globalization::CultureInfo_Data> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Globalization::CultureInfo_Data = __cordl_object
            .invoke("GetTextInfoData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeUserPreferredCultureInfoInAppX(
        onCultureInfoChangedInAppX: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo_OnCultureInfoChangedDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InitializeUserPreferredCultureInfoInAppX",
                (onCultureInfoChangedInAppX),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New_6() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString3(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString__cordl_bool4(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        useUserOverride: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, useUserOverride))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString__cordl_bool__cordl_bool5(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        useUserOverride: bool,
        read_only: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, useUserOverride, read_only))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_0(
        culture: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (culture))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32__cordl_bool1(
        culture: i32,
        useUserOverride: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (culture, useUserOverride))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32__cordl_bool__cordl_bool2(
        culture: i32,
        useUserOverride: bool,
        read_only: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (culture, useUserOverride, read_only))?;
        Ok(__cordl_object.into())
    }
    pub fn OnCultureInfoChangedInAppX(
        language: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnCultureInfoChangedInAppX", (language))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCultureInfoForUserPreferredLanguageInAppX(
        cultureInfo: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetCultureInfoForUserPreferredLanguageInAppX", (cultureInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUserPreferredCultureInfoInAppX(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetUserPreferredCultureInfoInAppX", (name))?;
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
    pub fn VerifyCultureName_CultureInfo1(
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        throwException: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VerifyCultureName", (culture, throwException))?;
        Ok(__cordl_ret.into())
    }
    pub fn VerifyCultureName_Il2CppString0(
        cultureName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        throwException: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VerifyCultureName", (cultureName, throwException))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_6(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString3(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString__cordl_bool4(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        useUserOverride: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, useUserOverride))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString__cordl_bool__cordl_bool5(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        useUserOverride: bool,
        read_only: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, useUserOverride, read_only))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_0(
        &mut self,
        culture: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (culture))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32__cordl_bool1(
        &mut self,
        culture: i32,
        useUserOverride: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (culture, useUserOverride))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32__cordl_bool__cordl_bool2(
        &mut self,
        culture: i32,
        useUserOverride: bool,
        read_only: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (culture, useUserOverride, read_only))?;
        Ok(__cordl_ret.into())
    }
    pub fn construct_internal_locale_from_lcid(
        &mut self,
        lcid: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("construct_internal_locale_from_lcid", (lcid))?;
        Ok(__cordl_ret.into())
    }
    pub fn construct_internal_locale_from_name(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("construct_internal_locale_from_name", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Calendar(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::Calendar>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::Calendar,
        > = __cordl_object.invoke("get_Calendar", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CalendarType(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_CalendarType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CompareInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CompareInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CompareInfo,
        > = __cordl_object.invoke("get_CompareInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CurrentCulture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CurrentCulture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CurrentUICulture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CurrentUICulture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DateTimeFormat(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        > = __cordl_object.invoke("get_DateTimeFormat", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultThreadCurrentCulture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_DefaultThreadCurrentCulture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultThreadCurrentUICulture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_DefaultThreadCurrentUICulture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EnglishName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_EnglishName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InvariantCulture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_InvariantCulture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsNeutralCulture(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsNeutralCulture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LCID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LCID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NumberFormat(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::NumberFormatInfo,
        > = __cordl_object.invoke("get_NumberFormat", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Parent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        > = __cordl_object.invoke("get_Parent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SortName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_SortName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Territory(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Territory", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TextInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::TextInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::TextInfo,
        > = __cordl_object.invoke("get_TextInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UserDefaultCulture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_UserDefaultCulture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UserDefaultUICulture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_UserDefaultUICulture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get__cultureData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureData,
        > = __cordl_object.invoke("get__cultureData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get__isInherited(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get__isInherited", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get__name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get__name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_current_locale_name() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_current_locale_name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn insert_into_shared_tables(
        c: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("insert_into_shared_tables", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn internal_get_cultures(
        neutral: bool,
        specific: bool,
        installed: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("internal_get_cultures", (neutral, specific, installed))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CurrentCulture(
        value: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_CurrentCulture", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CurrentUICulture(
        value: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_CurrentUICulture", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_DateTimeFormat(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DateTimeFormat", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_DefaultThreadCurrentCulture(
        value: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_DefaultThreadCurrentCulture", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_DefaultThreadCurrentUICulture(
        value: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_DefaultThreadCurrentUICulture", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_NumberFormat(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_NumberFormat", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Globalization+CultureInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Globalization::CultureInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Globalization+CultureInfo")]
impl AsRef<crate::System::ICloneable> for crate::System::Globalization::CultureInfo {
    fn as_ref(&self) -> &crate::System::ICloneable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Globalization+CultureInfo")]
impl AsMut<crate::System::ICloneable> for crate::System::Globalization::CultureInfo {
    fn as_mut(&mut self) -> &mut crate::System::ICloneable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Globalization+CultureInfo")]
impl AsRef<crate::System::IFormatProvider>
for crate::System::Globalization::CultureInfo {
    fn as_ref(&self) -> &crate::System::IFormatProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Globalization+CultureInfo")]
impl AsMut<crate::System::IFormatProvider>
for crate::System::Globalization::CultureInfo {
    fn as_mut(&mut self) -> &mut crate::System::IFormatProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Globalization+CultureInfo+Data")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CultureInfo_Data {
    pub ansi: i32,
    pub ebcdic: i32,
    pub mac: i32,
    pub oem: i32,
    pub right_to_left: bool,
    pub list_sep: u8,
}
#[cfg(feature = "System+Globalization+CultureInfo+Data")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Globalization::CultureInfo_Data {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "Data";
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
#[cfg(feature = "System+Globalization+CultureInfo+Data")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Globalization::CultureInfo_Data {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Globalization+CultureInfo+Data")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Globalization::CultureInfo_Data {
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
#[cfg(feature = "System+Globalization+CultureInfo+Data")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Globalization::CultureInfo_Data {
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
#[cfg(feature = "System+Globalization+CultureInfo+Data")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Globalization::CultureInfo_Data {
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
#[cfg(feature = "System+Globalization+CultureInfo+Data")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Globalization::CultureInfo_Data {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Globalization+CultureInfo+Data")]
impl crate::System::Globalization::CultureInfo_Data {}
#[cfg(feature = "System+Globalization+CultureInfo+OnCultureInfoChangedDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct CultureInfo_OnCultureInfoChangedDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+Globalization+CultureInfo+OnCultureInfoChangedDelegate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Globalization::CultureInfo_OnCultureInfoChangedDelegate {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "CultureInfo/OnCultureInfoChangedDelegate";
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
#[cfg(feature = "System+Globalization+CultureInfo+OnCultureInfoChangedDelegate")]
impl std::ops::Deref
for crate::System::Globalization::CultureInfo_OnCultureInfoChangedDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+CultureInfo+OnCultureInfoChangedDelegate")]
impl std::ops::DerefMut
for crate::System::Globalization::CultureInfo_OnCultureInfoChangedDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+CultureInfo+OnCultureInfoChangedDelegate")]
impl crate::System::Globalization::CultureInfo_OnCultureInfoChangedDelegate {
    pub fn Invoke(
        &mut self,
        language: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (language))?;
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
#[cfg(feature = "System+Globalization+CultureInfo+OnCultureInfoChangedDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Globalization::CultureInfo_OnCultureInfoChangedDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
