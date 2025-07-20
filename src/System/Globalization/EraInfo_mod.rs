#[cfg(feature = "System+Globalization+EraInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct EraInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub era: i32,
    pub ticks: i64,
    pub yearOffset: i32,
    pub minEraYear: i32,
    pub maxEraYear: i32,
    pub eraName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub abbrevEraName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub englishEraName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+Globalization+EraInfo")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Globalization::EraInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "EraInfo";
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
#[cfg(feature = "System+Globalization+EraInfo")]
impl std::ops::Deref for crate::System::Globalization::EraInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+EraInfo")]
impl std::ops::DerefMut for crate::System::Globalization::EraInfo {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+EraInfo")]
impl crate::System::Globalization::EraInfo {
    pub fn New_Il2CppString_Il2CppString_Il2CppString1(
        era: i32,
        startYear: i32,
        startMonth: i32,
        startDay: i32,
        yearOffset: i32,
        minEraYear: i32,
        maxEraYear: i32,
        eraName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        abbrevEraName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        englishEraName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    era,
                    startYear,
                    startMonth,
                    startDay,
                    yearOffset,
                    minEraYear,
                    maxEraYear,
                    eraName,
                    abbrevEraName,
                    englishEraName,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_i32_i32_i32_i32_i32_i32_0(
        era: i32,
        startYear: i32,
        startMonth: i32,
        startDay: i32,
        yearOffset: i32,
        minEraYear: i32,
        maxEraYear: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    era,
                    startYear,
                    startMonth,
                    startDay,
                    yearOffset,
                    minEraYear,
                    maxEraYear,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Il2CppString_Il2CppString_Il2CppString1(
        &mut self,
        era: i32,
        startYear: i32,
        startMonth: i32,
        startDay: i32,
        yearOffset: i32,
        minEraYear: i32,
        maxEraYear: i32,
        eraName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        abbrevEraName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        englishEraName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        10usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            10usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        era,
                        startYear,
                        startMonth,
                        startDay,
                        yearOffset,
                        minEraYear,
                        maxEraYear,
                        eraName,
                        abbrevEraName,
                        englishEraName,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_i32_i32_i32_i32_i32_0(
        &mut self,
        era: i32,
        startYear: i32,
        startMonth: i32,
        startDay: i32,
        yearOffset: i32,
        minEraYear: i32,
        maxEraYear: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, i32, i32, i32, i32, i32, i32),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        era,
                        startYear,
                        startMonth,
                        startDay,
                        yearOffset,
                        minEraYear,
                        maxEraYear,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Globalization+EraInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Globalization::EraInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
