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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_GroupLightData,
                        >,
                    >,
                >,
                0usize,
            >("get_groupLightData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_groupLightData", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_GroupLightData,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_groupLightsWeighting(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::ColorMixAndWeightingApproach,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::ColorMixAndWeightingApproach,
                0usize,
            >("get_groupLightsWeighting")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_groupLightsWeighting", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::ColorMixAndWeightingApproach = unsafe {
            method.invoke_unchecked(self, ())
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_LightIntensitiesWithId,
                        >,
                    >,
                >,
                0usize,
            >("get_individualLightData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_individualLightData", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_LightIntensitiesWithId,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_lightmapLightWithIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightmapLightWithIds>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightmapLightWithIds>,
                0usize,
            >("get_lightmapLightWithIds")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_lightmapLightWithIds", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LightmapLightWithIds,
        > = unsafe { method.invoke_unchecked(self, ()) };
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
    const CLASS_NAME: &'static str = "LightmapLightWithIdsGroupEntry/GroupLightData";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightGroupSO>,
                    f32,
                    f32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (lightGroup, groupIntensity, groupProbeHighlightsIntensityMultiplier),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_groupIntensity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_groupIntensity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_groupIntensity", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_groupProbeHighlightsIntensityMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                f32,
                0usize,
            >("get_groupProbeHighlightsIntensityMultiplier")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_groupProbeHighlightsIntensityMultiplier", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_lightGroup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightGroupSO>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightGroupSO>,
                0usize,
            >("get_lightGroup")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_lightGroup", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LightGroupSO,
        > = unsafe { method.invoke_unchecked(self, ()) };
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
    const CLASS_NAME: &'static str = "LightmapLightWithIdsGroupEntry/LightIntensitiesWithId";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, f32, f32), quest_hook::libil2cpp::Void, 3usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (lightId, intensity, probeHighlightsIntensityMultiplier),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_intensity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_intensity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_intensity", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_lightId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_lightId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_lightId", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_probeHighlightsIntensityMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_probeHighlightsIntensityMultiplier")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_probeHighlightsIntensityMultiplier", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ()) };
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
