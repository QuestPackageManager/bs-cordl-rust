#[cfg(feature = "BeatmapSaveDataVersion3+LightColorEventBox")]
#[repr(C)]
#[derive(Debug)]
pub struct LightColorEventBox {
    __cordl_parent: crate::BeatmapSaveDataVersion3::EventBox,
    pub r: f32,
    pub t: crate::BeatmapSaveDataCommon::DistributionParamType,
    pub b: i32,
    pub i: crate::BeatmapSaveDataCommon::EaseType,
    pub e: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion3::LightColorBaseData>,
        >,
    >,
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightColorEventBox")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapSaveDataVersion3::LightColorEventBox {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapSaveDataVersion3";
    const CLASS_NAME: &'static str = "LightColorEventBox";
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
#[cfg(feature = "BeatmapSaveDataVersion3+LightColorEventBox")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion3::LightColorEventBox {
    type Target = crate::BeatmapSaveDataVersion3::EventBox;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightColorEventBox")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion3::LightColorEventBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightColorEventBox")]
impl crate::BeatmapSaveDataVersion3::LightColorEventBox {
    pub fn New(
        indexFilter: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::IndexFilter,
        >,
        beatDistributionParam: f32,
        beatDistributionParamType: crate::BeatmapSaveDataCommon::DistributionParamType,
        brightnessDistributionParam: f32,
        brightnessDistributionShouldAffectFirstBaseEvent: bool,
        brightnessDistributionParamType: crate::BeatmapSaveDataCommon::DistributionParamType,
        brightnessDistributionEaseType: crate::BeatmapSaveDataCommon::EaseType,
        lightColorBaseDataList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatmapSaveDataVersion3::LightColorBaseData,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    indexFilter,
                    beatDistributionParam,
                    beatDistributionParamType,
                    brightnessDistributionParam,
                    brightnessDistributionShouldAffectFirstBaseEvent,
                    brightnessDistributionParamType,
                    brightnessDistributionEaseType,
                    lightColorBaseDataList,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        indexFilter: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::IndexFilter,
        >,
        beatDistributionParam: f32,
        beatDistributionParamType: crate::BeatmapSaveDataCommon::DistributionParamType,
        brightnessDistributionParam: f32,
        brightnessDistributionShouldAffectFirstBaseEvent: bool,
        brightnessDistributionParamType: crate::BeatmapSaveDataCommon::DistributionParamType,
        brightnessDistributionEaseType: crate::BeatmapSaveDataCommon::EaseType,
        lightColorBaseDataList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatmapSaveDataVersion3::LightColorBaseData,
                >,
            >,
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
                    brightnessDistributionParam,
                    brightnessDistributionShouldAffectFirstBaseEvent,
                    brightnessDistributionParamType,
                    brightnessDistributionEaseType,
                    lightColorBaseDataList,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_brightnessDistributionEaseType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::EaseType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataCommon::EaseType = __cordl_object
            .invoke("get_brightnessDistributionEaseType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_brightnessDistributionParam(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_brightnessDistributionParam", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_brightnessDistributionParamType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatmapSaveDataCommon::DistributionParamType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataCommon::DistributionParamType = __cordl_object
            .invoke("get_brightnessDistributionParamType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_brightnessDistributionShouldAffectFirstBaseEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_brightnessDistributionShouldAffectFirstBaseEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lightColorBaseDataList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatmapSaveDataVersion3::LightColorBaseData,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatmapSaveDataVersion3::LightColorBaseData,
                >,
            >,
        > = __cordl_object.invoke("get_lightColorBaseDataList", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightColorEventBox")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion3::LightColorEventBox {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
