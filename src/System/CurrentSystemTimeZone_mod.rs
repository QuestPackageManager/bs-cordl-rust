#[cfg(feature = "System+CurrentSystemTimeZone")]
#[repr(C)]
#[derive(Debug)]
pub struct CurrentSystemTimeZone {
    __cordl_parent: crate::System::TimeZone,
    pub m_ticksOffset: i64,
    pub m_standardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_daylightName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_CachedDaylightChanges: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Hashtable,
    >,
}
#[cfg(feature = "System+CurrentSystemTimeZone")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::CurrentSystemTimeZone {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "CurrentSystemTimeZone";
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
#[cfg(feature = "System+CurrentSystemTimeZone")]
impl std::ops::Deref for crate::System::CurrentSystemTimeZone {
    type Target = crate::System::TimeZone;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+CurrentSystemTimeZone")]
impl std::ops::DerefMut for crate::System::CurrentSystemTimeZone {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+CurrentSystemTimeZone")]
impl crate::System::CurrentSystemTimeZone {
    pub fn CreateDaylightChanges(
        year: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::DaylightTime>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Globalization::DaylightTime,
                        >,
                        1usize,
                    >("CreateDaylightChanges")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateDaylightChanges", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DaylightTime,
        > = unsafe { cordl_method_info.invoke_unchecked((), (year))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCachedDaylightChanges(
        &mut self,
        year: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::DaylightTime>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Globalization::DaylightTime,
                        >,
                        1usize,
                    >("GetCachedDaylightChanges")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCachedDaylightChanges", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DaylightTime,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (year))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDaylightChanges(
        &mut self,
        year: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::DaylightTime>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Globalization::DaylightTime,
                        >,
                        1usize,
                    >("GetDaylightChanges")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDaylightChanges", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DaylightTime,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (year))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTimeZoneData(
        year: i32,
        data: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i64>>,
        >,
        names: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                >,
            >,
        >,
        daylight_inverted: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<i64>,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        quest_hook::libil2cpp::Gc<
                                            quest_hook::libil2cpp::Il2CppString,
                                        >,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                        ),
                        bool,
                        4usize,
                    >("GetTimeZoneData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTimeZoneData", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (year, data, names, daylight_inverted))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetUtcOffset(
        &mut self,
        _cordl_time: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::DateTime),
                        crate::System::TimeSpan,
                        1usize,
                    >("GetUtcOffset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetUtcOffset", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::TimeSpan = unsafe {
            cordl_method_info.invoke_unchecked(self, (_cordl_time))?
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
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
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
