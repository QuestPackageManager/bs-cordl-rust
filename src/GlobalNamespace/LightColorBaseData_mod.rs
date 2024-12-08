#[cfg(feature = "LightColorBaseData")]
#[repr(C)]
#[derive(Debug)]
pub struct LightColorBaseData {
    __cordl_parent: crate::System::Object,
    pub beat: f32,
    pub usePreviousValue: bool,
    pub easeType: EaseType,
    pub colorType: EnvironmentColorType,
    pub brightness: f32,
    pub strobeBeatFrequency: i32,
    pub strobeBrightness: f32,
    pub strobeFade: bool,
}
#[cfg(feature = "LightColorBaseData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LightColorBaseData => ""."LightColorBaseData"
);
#[cfg(feature = "LightColorBaseData")]
impl std::ops::Deref for LightColorBaseData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightColorBaseData")]
impl std::ops::DerefMut for LightColorBaseData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightColorBaseData")]
impl LightColorBaseData {
    pub fn _ctor(
        &mut self,
        beat: f32,
        usePreviousValue: bool,
        easeType: EaseType,
        colorType: EnvironmentColorType,
        brightness: f32,
        strobeBeatFrequency: i32,
        strobeBrightness: f32,
        strobeFade: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    beat,
                    usePreviousValue,
                    easeType,
                    colorType,
                    brightness,
                    strobeBeatFrequency,
                    strobeBrightness,
                    strobeFade,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        beat: f32,
        usePreviousValue: bool,
        easeType: EaseType,
        colorType: EnvironmentColorType,
        brightness: f32,
        strobeBeatFrequency: i32,
        strobeBrightness: f32,
        strobeFade: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    beat,
                    usePreviousValue,
                    easeType,
                    colorType,
                    brightness,
                    strobeBeatFrequency,
                    strobeBrightness,
                    strobeFade,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "LightColorBaseData")]
impl quest_hook::libil2cpp::ObjectType for LightColorBaseData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
