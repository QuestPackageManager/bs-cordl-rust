#[cfg(feature = "BeatmapSaveDataVersion3+LightColorBaseData")]
#[repr(C)]
#[derive(Debug)]
pub struct LightColorBaseData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub b: f32,
    pub i: crate::BeatmapSaveDataVersion3::TransitionType,
    pub c: crate::BeatmapSaveDataCommon::EnvironmentColorType,
    pub s: f32,
    pub f: i32,
    pub sb: f32,
    pub sf: i32,
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightColorBaseData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapSaveDataVersion3::LightColorBaseData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapSaveDataVersion3";
    const CLASS_NAME: &'static str = "LightColorBaseData";
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
#[cfg(feature = "BeatmapSaveDataVersion3+LightColorBaseData")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion3::LightColorBaseData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightColorBaseData")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion3::LightColorBaseData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightColorBaseData")]
impl crate::BeatmapSaveDataVersion3::LightColorBaseData {
    pub fn New(
        beat: f32,
        transitionType: crate::BeatmapSaveDataVersion3::TransitionType,
        colorType: crate::BeatmapSaveDataCommon::EnvironmentColorType,
        brightness: f32,
        strobeFrequency: i32,
        strobeBrightness: f32,
        strobeFade: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    beat,
                    transitionType,
                    colorType,
                    brightness,
                    strobeFrequency,
                    strobeBrightness,
                    strobeFade,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        beat: f32,
        transitionType: crate::BeatmapSaveDataVersion3::TransitionType,
        colorType: crate::BeatmapSaveDataCommon::EnvironmentColorType,
        brightness: f32,
        strobeFrequency: i32,
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
                    transitionType,
                    colorType,
                    brightness,
                    strobeFrequency,
                    strobeBrightness,
                    strobeFade,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beat(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_beat", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_brightness(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_brightness", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_colorType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatmapSaveDataCommon::EnvironmentColorType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataCommon::EnvironmentColorType = __cordl_object
            .invoke("get_colorType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_strobeBeatFrequency(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_strobeBeatFrequency", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_strobeBrightness(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_strobeBrightness", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_strobeFade(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_strobeFade", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_transitionType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataVersion3::TransitionType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataVersion3::TransitionType = __cordl_object
            .invoke("get_transitionType", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightColorBaseData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion3::LightColorBaseData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
