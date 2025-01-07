#[cfg(feature = "LightmapLightWithIdsGroupEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct LightmapLightWithIdsGroupEntry {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _lightmapLightWithIds: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LightmapLightWithIds,
    >,
    pub _groupLightData: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_GroupLightData,
            >,
        >,
    >,
    pub _individualLightData: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_LightIntensitiesWithId,
            >,
        >,
    >,
    pub _excludedLightIds: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub _groupLightsWeighting: crate::GlobalNamespace::ColorMixAndWeightingApproach,
}
#[cfg(feature = "LightmapLightWithIdsGroupEntry")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::LightmapLightWithIdsGroupEntry {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LightmapLightWithIdsGroupEntry";
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
#[cfg(feature = "LightmapLightWithIdsGroupEntry")]
impl std::ops::Deref for crate::GlobalNamespace::LightmapLightWithIdsGroupEntry {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightmapLightWithIdsGroupEntry")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightmapLightWithIdsGroupEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightmapLightWithIdsGroupEntry")]
impl crate::GlobalNamespace::LightmapLightWithIdsGroupEntry {
    #[cfg(feature = "LightmapLightWithIdsGroupEntry+GroupLightData")]
    pub type GroupLightData = crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_GroupLightData;
    #[cfg(feature = "LightmapLightWithIdsGroupEntry+LightIntensitiesWithId")]
    pub type LightIntensitiesWithId = crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_LightIntensitiesWithId;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_groupLightData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_GroupLightData,
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
                    crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_GroupLightData,
                >,
            >,
        > = __cordl_object.invoke("get_groupLightData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_groupLightsWeighting(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::ColorMixAndWeightingApproach,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::ColorMixAndWeightingApproach = __cordl_object
            .invoke("get_groupLightsWeighting", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_individualLightData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_LightIntensitiesWithId,
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
                    crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_LightIntensitiesWithId,
                >,
            >,
        > = __cordl_object.invoke("get_individualLightData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lightmapLightWithIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightmapLightWithIds>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LightmapLightWithIds,
        > = __cordl_object.invoke("get_lightmapLightWithIds", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LightmapLightWithIdsGroupEntry")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightmapLightWithIdsGroupEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LightmapLightWithIdsGroupEntry+GroupLightData")]
#[repr(C)]
#[derive(Debug)]
pub struct LightmapLightWithIdsGroupEntry_GroupLightData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _lightGroup: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightGroupSO>,
    pub _groupIntensity: f32,
    pub _groupProbeHighlightsIntensityMultiplier: f32,
}
#[cfg(feature = "LightmapLightWithIdsGroupEntry+GroupLightData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_GroupLightData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GroupLightData";
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
#[cfg(feature = "LightmapLightWithIdsGroupEntry+GroupLightData")]
impl std::ops::Deref
for crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_GroupLightData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightmapLightWithIdsGroupEntry+GroupLightData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_GroupLightData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightmapLightWithIdsGroupEntry+GroupLightData")]
impl crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_GroupLightData {
    pub fn New(
        lightGroup: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightGroupSO>,
        groupIntensity: f32,
        groupProbeHighlightsIntensityMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (lightGroup, groupIntensity, groupProbeHighlightsIntensityMultiplier),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        lightGroup: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightGroupSO>,
        groupIntensity: f32,
        groupProbeHighlightsIntensityMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (lightGroup, groupIntensity, groupProbeHighlightsIntensityMultiplier),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_groupIntensity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_groupIntensity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_groupProbeHighlightsIntensityMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_groupProbeHighlightsIntensityMultiplier", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lightGroup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightGroupSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LightGroupSO,
        > = __cordl_object.invoke("get_lightGroup", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LightmapLightWithIdsGroupEntry+GroupLightData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_GroupLightData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LightmapLightWithIdsGroupEntry+LightIntensitiesWithId")]
#[repr(C)]
#[derive(Debug)]
pub struct LightmapLightWithIdsGroupEntry_LightIntensitiesWithId {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _lightId: i32,
    pub _intensity: f32,
    pub _probeHighlightsIntensityMultiplier: f32,
}
#[cfg(feature = "LightmapLightWithIdsGroupEntry+LightIntensitiesWithId")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_LightIntensitiesWithId {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LightIntensitiesWithId";
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
#[cfg(feature = "LightmapLightWithIdsGroupEntry+LightIntensitiesWithId")]
impl std::ops::Deref
for crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_LightIntensitiesWithId {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightmapLightWithIdsGroupEntry+LightIntensitiesWithId")]
impl std::ops::DerefMut
for crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_LightIntensitiesWithId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightmapLightWithIdsGroupEntry+LightIntensitiesWithId")]
impl crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_LightIntensitiesWithId {
    pub fn New(
        lightId: i32,
        intensity: f32,
        probeHighlightsIntensityMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (lightId, intensity, probeHighlightsIntensityMultiplier),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        lightId: i32,
        intensity: f32,
        probeHighlightsIntensityMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lightId, intensity, probeHighlightsIntensityMultiplier))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_intensity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_intensity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lightId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_lightId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_probeHighlightsIntensityMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_probeHighlightsIntensityMultiplier", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LightmapLightWithIdsGroupEntry+LightIntensitiesWithId")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_LightIntensitiesWithId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
