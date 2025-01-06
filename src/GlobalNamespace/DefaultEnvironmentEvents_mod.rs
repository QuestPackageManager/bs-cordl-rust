#[cfg(feature = "DefaultEnvironmentEvents")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultEnvironmentEvents {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _basicBeatmapEvents: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::DefaultEnvironmentEvents_BasicBeatmapEvent,
            >,
        >,
    >,
    pub _lightGroupEvents: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupEvent,
            >,
        >,
    >,
}
#[cfg(feature = "DefaultEnvironmentEvents")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::DefaultEnvironmentEvents => ""
    ."DefaultEnvironmentEvents"
);
#[cfg(feature = "DefaultEnvironmentEvents")]
impl std::ops::Deref for crate::GlobalNamespace::DefaultEnvironmentEvents {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DefaultEnvironmentEvents")]
impl std::ops::DerefMut for crate::GlobalNamespace::DefaultEnvironmentEvents {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DefaultEnvironmentEvents")]
impl crate::GlobalNamespace::DefaultEnvironmentEvents {
    #[cfg(feature = "DefaultEnvironmentEvents+BasicBeatmapEvent")]
    pub type BasicBeatmapEvent = crate::GlobalNamespace::DefaultEnvironmentEvents_BasicBeatmapEvent;
    #[cfg(feature = "DefaultEnvironmentEvents+LightGroupDistribution")]
    pub type LightGroupDistribution = crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution;
    #[cfg(feature = "DefaultEnvironmentEvents+LightGroupEvent")]
    pub type LightGroupEvent = crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupEvent;
    #[cfg(feature = "DefaultEnvironmentEvents+LightGroupFiltering")]
    pub type LightGroupFiltering = crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupFiltering;
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
    pub fn get_basicBeatmapEvents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::DefaultEnvironmentEvents_BasicBeatmapEvent,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::DefaultEnvironmentEvents_BasicBeatmapEvent,
                >,
            >,
        > = __cordl_object.invoke("get_basicBeatmapEvents", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isEmpty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lightGroupEvents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupEvent,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupEvent,
                >,
            >,
        > = __cordl_object.invoke("get_lightGroupEvents", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "DefaultEnvironmentEvents")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DefaultEnvironmentEvents {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "DefaultEnvironmentEvents+BasicBeatmapEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultEnvironmentEvents_BasicBeatmapEvent {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _eventType: crate::GlobalNamespace::BasicBeatmapEventType,
    pub _value: i32,
    pub _floatValue: f32,
}
#[cfg(feature = "DefaultEnvironmentEvents+BasicBeatmapEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::DefaultEnvironmentEvents_BasicBeatmapEvent => ""
    ."DefaultEnvironmentEvents/BasicBeatmapEvent"
);
#[cfg(feature = "DefaultEnvironmentEvents+BasicBeatmapEvent")]
impl std::ops::Deref
for crate::GlobalNamespace::DefaultEnvironmentEvents_BasicBeatmapEvent {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DefaultEnvironmentEvents+BasicBeatmapEvent")]
impl std::ops::DerefMut
for crate::GlobalNamespace::DefaultEnvironmentEvents_BasicBeatmapEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DefaultEnvironmentEvents+BasicBeatmapEvent")]
impl crate::GlobalNamespace::DefaultEnvironmentEvents_BasicBeatmapEvent {
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
    pub fn get_eventType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BasicBeatmapEventType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BasicBeatmapEventType = __cordl_object
            .invoke("get_eventType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_floatValue(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_floatValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_value(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_value", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "DefaultEnvironmentEvents+BasicBeatmapEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DefaultEnvironmentEvents_BasicBeatmapEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "DefaultEnvironmentEvents+LightGroupDistribution")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultEnvironmentEvents_LightGroupDistribution {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _useDistribution: bool,
    pub _distributionParam: f32,
    pub _distributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
}
#[cfg(feature = "DefaultEnvironmentEvents+LightGroupDistribution")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution => ""
    ."DefaultEnvironmentEvents/LightGroupDistribution"
);
#[cfg(feature = "DefaultEnvironmentEvents+LightGroupDistribution")]
impl std::ops::Deref
for crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DefaultEnvironmentEvents+LightGroupDistribution")]
impl std::ops::DerefMut
for crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DefaultEnvironmentEvents+LightGroupDistribution")]
impl crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution {
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
    pub fn get_distributionParam(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_distributionParam", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_distributionParamType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType = __cordl_object
            .invoke("get_distributionParamType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_useDistribution(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_useDistribution", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "DefaultEnvironmentEvents+LightGroupDistribution")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "DefaultEnvironmentEvents+LightGroupEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultEnvironmentEvents_LightGroupEvent {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _lightGroup: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightGroupSO>,
    pub _environmentColorType: crate::GlobalNamespace::EnvironmentColorType,
    pub _brightness: f32,
    pub _brightnessDistribution: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution,
    >,
    pub _brightnessFiltering: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupFiltering,
    >,
    pub _rotationX: f32,
    pub _rotationY: f32,
    pub _rotationZ: f32,
    pub _rotationXDistribution: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution,
    >,
    pub _rotationYDistribution: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution,
    >,
    pub _rotationZDistribution: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution,
    >,
    pub _rotationFiltering: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupFiltering,
    >,
    pub _translationX: f32,
    pub _translationY: f32,
    pub _translationZ: f32,
    pub _translationXDistribution: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution,
    >,
    pub _translationYDistribution: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution,
    >,
    pub _translationZDistribution: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution,
    >,
    pub _translationFiltering: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupFiltering,
    >,
    pub _floatFxValue: f32,
    pub _floatFxDistribution: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution,
    >,
    pub _floatFxFiltering: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupFiltering,
    >,
}
#[cfg(feature = "DefaultEnvironmentEvents+LightGroupEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::DefaultEnvironmentEvents_LightGroupEvent => ""
    ."DefaultEnvironmentEvents/LightGroupEvent"
);
#[cfg(feature = "DefaultEnvironmentEvents+LightGroupEvent")]
impl std::ops::Deref
for crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupEvent {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DefaultEnvironmentEvents+LightGroupEvent")]
impl std::ops::DerefMut
for crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DefaultEnvironmentEvents+LightGroupEvent")]
impl crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupEvent {
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
    pub fn get_brightness(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_brightness", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_brightnessDistribution(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution,
        > = __cordl_object.invoke("get_brightnessDistribution", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_brightnessFiltering(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupFiltering,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupFiltering,
        > = __cordl_object.invoke("get_brightnessFiltering", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_environmentColorType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::EnvironmentColorType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::EnvironmentColorType = __cordl_object
            .invoke("get_environmentColorType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_floatFxDistribution(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution,
        > = __cordl_object.invoke("get_floatFxDistribution", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_floatFxFiltering(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupFiltering,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupFiltering,
        > = __cordl_object.invoke("get_floatFxFiltering", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_floatFxValue(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_floatFxValue", ())?;
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
    pub fn get_rotationFiltering(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupFiltering,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupFiltering,
        > = __cordl_object.invoke("get_rotationFiltering", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rotationX(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_rotationX", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rotationXDistribution(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution,
        > = __cordl_object.invoke("get_rotationXDistribution", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rotationY(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_rotationY", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rotationYDistribution(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution,
        > = __cordl_object.invoke("get_rotationYDistribution", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rotationZ(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_rotationZ", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rotationZDistribution(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution,
        > = __cordl_object.invoke("get_rotationZDistribution", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_translationFiltering(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupFiltering,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupFiltering,
        > = __cordl_object.invoke("get_translationFiltering", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_translationX(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_translationX", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_translationXDistribution(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution,
        > = __cordl_object.invoke("get_translationXDistribution", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_translationY(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_translationY", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_translationYDistribution(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution,
        > = __cordl_object.invoke("get_translationYDistribution", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_translationZ(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_translationZ", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_translationZDistribution(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupDistribution,
        > = __cordl_object.invoke("get_translationZDistribution", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "DefaultEnvironmentEvents+LightGroupEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "DefaultEnvironmentEvents+LightGroupFiltering")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultEnvironmentEvents_LightGroupFiltering {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _useFiltering: bool,
    pub _randomType: crate::GlobalNamespace::IndexFilter_IndexFilterRandomType,
    pub _limit: f32,
    pub _alsoAffectType: crate::GlobalNamespace::IndexFilter_IndexFilterLimitAlsoAffectType,
    pub _seed: i32,
    pub _chunks: i32,
}
#[cfg(feature = "DefaultEnvironmentEvents+LightGroupFiltering")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::DefaultEnvironmentEvents_LightGroupFiltering => ""
    ."DefaultEnvironmentEvents/LightGroupFiltering"
);
#[cfg(feature = "DefaultEnvironmentEvents+LightGroupFiltering")]
impl std::ops::Deref
for crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupFiltering {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DefaultEnvironmentEvents+LightGroupFiltering")]
impl std::ops::DerefMut
for crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupFiltering {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DefaultEnvironmentEvents+LightGroupFiltering")]
impl crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupFiltering {
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
    pub fn get_alsoAffectType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::IndexFilter_IndexFilterLimitAlsoAffectType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::IndexFilter_IndexFilterLimitAlsoAffectType = __cordl_object
            .invoke("get_alsoAffectType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_chunks(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_chunks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_limit(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_limit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_randomType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::IndexFilter_IndexFilterRandomType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::IndexFilter_IndexFilterRandomType = __cordl_object
            .invoke("get_randomType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_seed(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_seed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_useFiltering(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_useFiltering", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "DefaultEnvironmentEvents+LightGroupFiltering")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DefaultEnvironmentEvents_LightGroupFiltering {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
