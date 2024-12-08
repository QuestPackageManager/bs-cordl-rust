#[cfg(feature = "BeatmapSaveDataVersion3+LightTranslationEventBox")]
#[repr(C)]
#[derive(Debug)]
pub struct LightTranslationEventBox {
    __cordl_parent: crate::BeatmapSaveDataVersion3::EventBox,
    pub s: f32,
    pub t: crate::BeatmapSaveDataCommon::DistributionParamType,
    pub a: crate::BeatmapSaveDataCommon::Axis,
    pub r: i32,
    pub b: i32,
    pub i: crate::BeatmapSaveDataCommon::EaseType,
    pub l: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BeatmapSaveDataVersion3::LightTranslationBaseData,
    >,
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightTranslationEventBox")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapSaveDataVersion3::LightTranslationEventBox => "BeatmapSaveDataVersion3"
    ."LightTranslationEventBox"
);
#[cfg(feature = "BeatmapSaveDataVersion3+LightTranslationEventBox")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion3::LightTranslationEventBox {
    type Target = crate::BeatmapSaveDataVersion3::EventBox;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightTranslationEventBox")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion3::LightTranslationEventBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightTranslationEventBox")]
impl crate::BeatmapSaveDataVersion3::LightTranslationEventBox {
    pub fn _ctor(
        &mut self,
        indexFilter: *mut crate::BeatmapSaveDataVersion3::IndexFilter,
        beatDistributionParam: f32,
        beatDistributionParamType: crate::BeatmapSaveDataCommon::DistributionParamType,
        gapDistributionParam: f32,
        gapDistributionParamType: crate::BeatmapSaveDataCommon::DistributionParamType,
        gapDistributionShouldAffectFirstBaseEvent: bool,
        gapDistributionEaseType: crate::BeatmapSaveDataCommon::EaseType,
        axis: crate::BeatmapSaveDataCommon::Axis,
        flipTranslation: bool,
        lightTranslationBaseDataList: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::LightTranslationBaseData,
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
                    gapDistributionParam,
                    gapDistributionParamType,
                    gapDistributionShouldAffectFirstBaseEvent,
                    gapDistributionEaseType,
                    axis,
                    flipTranslation,
                    lightTranslationBaseDataList,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_gapDistributionParam(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_gapDistributionParam", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lightTranslationBaseDataList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::BeatmapSaveDataVersion3::LightTranslationBaseData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::BeatmapSaveDataVersion3::LightTranslationBaseData,
        > = __cordl_object.invoke("get_lightTranslationBaseDataList", ())?;
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
    pub fn get_gapDistributionShouldAffectFirstBaseEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_gapDistributionShouldAffectFirstBaseEvent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_flipTranslation(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_flipTranslation", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_gapDistributionEaseType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::EaseType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataCommon::EaseType = __cordl_object
            .invoke("get_gapDistributionEaseType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_gapDistributionParamType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatmapSaveDataCommon::DistributionParamType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataCommon::DistributionParamType = __cordl_object
            .invoke("get_gapDistributionParamType", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        indexFilter: *mut crate::BeatmapSaveDataVersion3::IndexFilter,
        beatDistributionParam: f32,
        beatDistributionParamType: crate::BeatmapSaveDataCommon::DistributionParamType,
        gapDistributionParam: f32,
        gapDistributionParamType: crate::BeatmapSaveDataCommon::DistributionParamType,
        gapDistributionShouldAffectFirstBaseEvent: bool,
        gapDistributionEaseType: crate::BeatmapSaveDataCommon::EaseType,
        axis: crate::BeatmapSaveDataCommon::Axis,
        flipTranslation: bool,
        lightTranslationBaseDataList: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::LightTranslationBaseData,
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
                    gapDistributionParam,
                    gapDistributionParamType,
                    gapDistributionShouldAffectFirstBaseEvent,
                    gapDistributionEaseType,
                    axis,
                    flipTranslation,
                    lightTranslationBaseDataList,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightTranslationEventBox")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion3::LightTranslationEventBox {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
