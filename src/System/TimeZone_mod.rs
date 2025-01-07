#[cfg(feature = "System+TimeZone")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeZone {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+TimeZone")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::TimeZone {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "TimeZone";
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
#[cfg(feature = "System+TimeZone")]
impl std::ops::Deref for crate::System::TimeZone {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+TimeZone")]
impl std::ops::DerefMut for crate::System::TimeZone {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+TimeZone")]
impl crate::System::TimeZone {
    pub fn CalculateUtcOffset(
        _cordl_time: crate::System::DateTime,
        daylightTimes: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DaylightTime,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateUtcOffset", (_cordl_time, daylightTimes))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDaylightChanges(
        &mut self,
        year: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::DaylightTime>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DaylightTime,
        > = __cordl_object.invoke("GetDaylightChanges", (year))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUtcOffset(
        &mut self,
        _cordl_time: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::TimeSpan = __cordl_object
            .invoke("GetUtcOffset", (_cordl_time))?;
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
    pub fn get_CurrentTimeZone() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::TimeZone>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::TimeZone> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CurrentTimeZone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InternalSyncObject() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_InternalSyncObject", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+TimeZone")]
impl quest_hook::libil2cpp::ObjectType for crate::System::TimeZone {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
