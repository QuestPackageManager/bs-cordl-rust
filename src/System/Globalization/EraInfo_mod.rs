#[cfg(feature = "System+Globalization+EraInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct EraInfo {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::EraInfo =>
    "System.Globalization"."EraInfo"
);
#[cfg(feature = "System+Globalization+EraInfo")]
impl std::ops::Deref for crate::System::Globalization::EraInfo {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+EraInfo")]
impl std::ops::DerefMut for crate::System::Globalization::EraInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+EraInfo")]
impl crate::System::Globalization::EraInfo {
    pub fn New_Gc_Gc_Gc1(
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
    pub fn _ctor_Gc_Gc_Gc1(
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
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
