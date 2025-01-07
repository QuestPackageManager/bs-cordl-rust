#[cfg(feature = "BeatmapSaveDataVersion3+LightRotationBaseData")]
#[repr(C)]
#[derive(Debug)]
pub struct LightRotationBaseData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub b: f32,
    pub p: i32,
    pub e: crate::BeatmapSaveDataCommon::EaseType,
    pub l: i32,
    pub r: f32,
    pub o: crate::BeatmapSaveDataCommon::RotationDirection,
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightRotationBaseData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapSaveDataVersion3::LightRotationBaseData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapSaveDataVersion3";
    const CLASS_NAME: &'static str = "LightRotationBaseData";
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
#[cfg(feature = "BeatmapSaveDataVersion3+LightRotationBaseData")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion3::LightRotationBaseData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightRotationBaseData")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion3::LightRotationBaseData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightRotationBaseData")]
impl crate::BeatmapSaveDataVersion3::LightRotationBaseData {
    pub fn New(
        beat: f32,
        usePreviousEventRotationValue: bool,
        easeType: crate::BeatmapSaveDataCommon::EaseType,
        loopsCount: i32,
        rotation: f32,
        rotationDirection: crate::BeatmapSaveDataCommon::RotationDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    beat,
                    usePreviousEventRotationValue,
                    easeType,
                    loopsCount,
                    rotation,
                    rotationDirection,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        beat: f32,
        usePreviousEventRotationValue: bool,
        easeType: crate::BeatmapSaveDataCommon::EaseType,
        loopsCount: i32,
        rotation: f32,
        rotationDirection: crate::BeatmapSaveDataCommon::RotationDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    beat,
                    usePreviousEventRotationValue,
                    easeType,
                    loopsCount,
                    rotation,
                    rotationDirection,
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
    pub fn get_easeType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::EaseType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataCommon::EaseType = __cordl_object
            .invoke("get_easeType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_loopsCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_loopsCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rotation(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_rotation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rotationDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::RotationDirection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataCommon::RotationDirection = __cordl_object
            .invoke("get_rotationDirection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_usePreviousEventRotationValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_usePreviousEventRotationValue", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightRotationBaseData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion3::LightRotationBaseData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
