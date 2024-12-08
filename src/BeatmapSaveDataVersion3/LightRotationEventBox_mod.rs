#[cfg(feature = "BeatmapSaveDataVersion3+LightRotationEventBox")]
#[repr(C)]
#[derive(Debug)]
pub struct LightRotationEventBox {
    __cordl_parent: crate::BeatmapSaveDataVersion3::EventBox,
    pub s: f32,
    pub t: crate::BeatmapSaveDataCommon::DistributionParamType,
    pub a: crate::BeatmapSaveDataCommon::Axis,
    pub r: i32,
    pub b: i32,
    pub i: crate::BeatmapSaveDataCommon::EaseType,
    pub l: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BeatmapSaveDataVersion3::LightRotationBaseData,
    >,
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightRotationEventBox")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion3::LightRotationEventBox
    => "BeatmapSaveDataVersion3"."LightRotationEventBox"
);
#[cfg(feature = "BeatmapSaveDataVersion3+LightRotationEventBox")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion3::LightRotationEventBox {
    type Target = crate::BeatmapSaveDataVersion3::EventBox;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightRotationEventBox")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion3::LightRotationEventBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightRotationEventBox")]
impl crate::BeatmapSaveDataVersion3::LightRotationEventBox {
    pub fn get_rotationDistributionParamType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatmapSaveDataCommon::DistributionParamType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataCommon::DistributionParamType = __cordl_object
            .invoke("get_rotationDistributionParamType", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        indexFilter: *mut crate::BeatmapSaveDataVersion3::IndexFilter,
        beatDistributionParam: f32,
        beatDistributionParamType: crate::BeatmapSaveDataCommon::DistributionParamType,
        rotationDistributionParam: f32,
        rotationDistributionParamType: crate::BeatmapSaveDataCommon::DistributionParamType,
        rotationDistributionShouldAffectFirstBaseEvent: bool,
        rotationDistributionEaseType: crate::BeatmapSaveDataCommon::EaseType,
        axis: crate::BeatmapSaveDataCommon::Axis,
        flipRotation: bool,
        lightRotationBaseDataList: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::LightRotationBaseData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    indexFilter,
                    beatDistributionParam,
                    beatDistributionParamType,
                    rotationDistributionParam,
                    rotationDistributionParamType,
                    rotationDistributionShouldAffectFirstBaseEvent,
                    rotationDistributionEaseType,
                    axis,
                    flipRotation,
                    lightRotationBaseDataList,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_rotationDistributionParam(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_rotationDistributionParam", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_flipRotation(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_flipRotation", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_axis(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::Axis> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataCommon::Axis = __cordl_object
            .invoke("get_axis", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rotationDistributionShouldAffectFirstBaseEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_rotationDistributionShouldAffectFirstBaseEvent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lightRotationBaseDataList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::BeatmapSaveDataVersion3::LightRotationBaseData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::BeatmapSaveDataVersion3::LightRotationBaseData,
        > = __cordl_object.invoke("get_lightRotationBaseDataList", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rotationDistributionEaseType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::EaseType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataCommon::EaseType = __cordl_object
            .invoke("get_rotationDistributionEaseType", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        indexFilter: *mut crate::BeatmapSaveDataVersion3::IndexFilter,
        beatDistributionParam: f32,
        beatDistributionParamType: crate::BeatmapSaveDataCommon::DistributionParamType,
        rotationDistributionParam: f32,
        rotationDistributionParamType: crate::BeatmapSaveDataCommon::DistributionParamType,
        rotationDistributionShouldAffectFirstBaseEvent: bool,
        rotationDistributionEaseType: crate::BeatmapSaveDataCommon::EaseType,
        axis: crate::BeatmapSaveDataCommon::Axis,
        flipRotation: bool,
        lightRotationBaseDataList: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::LightRotationBaseData,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    indexFilter,
                    beatDistributionParam,
                    beatDistributionParamType,
                    rotationDistributionParam,
                    rotationDistributionParamType,
                    rotationDistributionShouldAffectFirstBaseEvent,
                    rotationDistributionEaseType,
                    axis,
                    flipRotation,
                    lightRotationBaseDataList,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightRotationEventBox")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion3::LightRotationEventBox {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
