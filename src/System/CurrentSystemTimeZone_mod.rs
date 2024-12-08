#[cfg(feature = "System+CurrentSystemTimeZone")]
#[repr(C)]
#[derive(Debug)]
pub struct CurrentSystemTimeZone {
    __cordl_parent: crate::System::TimeZone,
    pub m_ticksOffset: i64,
    pub m_standardName: *mut crate::System::String,
    pub m_daylightName: *mut crate::System::String,
    pub m_CachedDaylightChanges: *mut crate::System::Collections::Hashtable,
}
#[cfg(feature = "System+CurrentSystemTimeZone")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::CurrentSystemTimeZone => "System"
    ."CurrentSystemTimeZone"
);
#[cfg(feature = "System+CurrentSystemTimeZone")]
impl std::ops::Deref for crate::System::CurrentSystemTimeZone {
    type Target = crate::System::TimeZone;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+CurrentSystemTimeZone")]
impl std::ops::DerefMut for crate::System::CurrentSystemTimeZone {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+CurrentSystemTimeZone")]
impl crate::System::CurrentSystemTimeZone {
    pub fn GetDaylightChanges(
        &mut self,
        year: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Globalization::DaylightTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Globalization::DaylightTime = __cordl_object
            .invoke("GetDaylightChanges", (year))?;
        Ok(__cordl_ret)
    }
    pub fn GetCachedDaylightChanges(
        &mut self,
        year: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Globalization::DaylightTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Globalization::DaylightTime = __cordl_object
            .invoke("GetCachedDaylightChanges", (year))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+CurrentSystemTimeZone")]
impl quest_hook::libil2cpp::ObjectType for crate::System::CurrentSystemTimeZone {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
