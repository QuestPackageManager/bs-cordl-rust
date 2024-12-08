#[cfg(feature = "HMUI+TimeSlider")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeSlider {
    __cordl_parent: crate::HMUI::RangeValuesTextSlider,
    pub _timeType: crate::HMUI::TimeSlider_TimeType,
    pub _valuesValid: bool,
    pub _lowerValue: f32,
    pub _upperValue: f32,
}
#[cfg(feature = "HMUI+TimeSlider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::TimeSlider => "HMUI"."TimeSlider"
);
#[cfg(feature = "HMUI+TimeSlider")]
impl std::ops::Deref for crate::HMUI::TimeSlider {
    type Target = crate::HMUI::RangeValuesTextSlider;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+TimeSlider")]
impl std::ops::DerefMut for crate::HMUI::TimeSlider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+TimeSlider")]
impl crate::HMUI::TimeSlider {
    #[cfg(feature = "HMUI+TimeSlider+TimeType")]
    pub type TimeType = crate::HMUI::TimeSlider_TimeType;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetBounds(
        &mut self,
        valuesValid: bool,
        lowerValue: f32,
        upperValue: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBounds", (valuesValid, lowerValue, upperValue))?;
        Ok(__cordl_ret)
    }
    pub fn TextForValue(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("TextForValue", (value))?;
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
}
#[cfg(feature = "HMUI+TimeSlider")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::TimeSlider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+TimeSlider+TimeType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeSlider_TimeType {
    Default = 0i32,
    Milliseconds = 1i32,
    Normalized = 2i32,
}
#[cfg(feature = "HMUI+TimeSlider+TimeType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HMUI::TimeSlider_TimeType => "HMUI"
    ."TimeSlider/TimeType"
);
