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
    pub numInfo: *mut crate::System::Globalization::NumberFormatInfo,
    pub dateTimeInfo: *mut crate::System::Globalization::DateTimeFormatInfo,
    pub textInfo: *mut crate::System::Globalization::TextInfo,
    pub m_name: *mut quest_hook::libil2cpp::Il2CppString,
    pub englishname: *mut quest_hook::libil2cpp::Il2CppString,
    pub nativename: *mut quest_hook::libil2cpp::Il2CppString,
    pub iso3lang: *mut quest_hook::libil2cpp::Il2CppString,
    pub iso2lang: *mut quest_hook::libil2cpp::Il2CppString,
    pub win3lang: *mut quest_hook::libil2cpp::Il2CppString,
    pub territory: *mut quest_hook::libil2cpp::Il2CppString,
    pub native_calendar_names: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub compareInfo: *mut crate::System::Globalization::CompareInfo,
    pub textinfo_data: *mut quest_hook::libil2cpp::Il2CppObject,
    pub m_dataItem: i32,
    pub calendar: *mut crate::System::Globalization::Calendar,
    pub parent_culture: *mut crate::System::Globalization::CultureInfo,
    pub constructed: bool,
    pub cached_serialized_form: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub m_cultureData: *mut crate::System::Globalization::CultureData,
    pub m_isInherited: bool,
}
#[cfg(feature = "System+Globalization+CultureInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::CultureInfo =>
    "System.Globalization"."CultureInfo"
);
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
#[derive(Debug, Clone)]
pub struct CultureInfo_Data {
    pub ansi: i32,
    pub ebcdic: i32,
    pub mac: i32,
    pub oem: i32,
    pub right_to_left: bool,
    pub list_sep: u8,
}
#[cfg(feature = "System+Globalization+CultureInfo+Data")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::CultureInfo_Data =>
    "System.Globalization"."CultureInfo/Data"
);
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Globalization::CultureInfo_OnCultureInfoChangedDelegate =>
    "System.Globalization"."CultureInfo/OnCultureInfoChangedDelegate"
);
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
