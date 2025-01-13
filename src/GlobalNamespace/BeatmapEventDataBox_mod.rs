#[cfg(feature = "BeatmapEventDataBox")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapEventDataBox {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _indexFilter_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IndexFilter,
    >,
    pub _beatDistributionParam: f32,
    pub _beatDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
    pub _eventDistributionCount: i32,
    pub _eventDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
    pub _eventDistributionParam: f32,
    pub _eventDistributionShouldAffectFirstBaseEvent: bool,
    pub _eventDistributionEaseType: crate::GlobalNamespace::EaseType,
}
#[cfg(feature = "BeatmapEventDataBox")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::BeatmapEventDataBox {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapEventDataBox";
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
#[cfg(feature = "BeatmapEventDataBox")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapEventDataBox {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventDataBox")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapEventDataBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventDataBox")]
impl crate::GlobalNamespace::BeatmapEventDataBox {
    #[cfg(feature = "BeatmapEventDataBox+DistributionParamType")]
    pub type DistributionParamType = crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType;
    pub fn BeatDistributionParamToStep(
        distributionParam: f32,
        distributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "BeatDistributionParamToStep",
                (distributionParam, distributionParamType, count),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EventDistributionParamToStep(
        index: i32,
        distributionParam: f32,
        distributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        count: i32,
        easeType: crate::GlobalNamespace::EaseType,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "EventDistributionParamToStep",
                (index, distributionParam, distributionParamType, count, easeType),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatStep_IndexFilter_BeatmapEventDataBox_DistributionParamType_f32_f32_1(
        indexFilter: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IndexFilter>,
        distributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        distributionParam: f32,
        lastBaseEventRelativeBeat: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetBeatStep",
                (
                    indexFilter,
                    distributionParamType,
                    distributionParam,
                    lastBaseEventRelativeBeat,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatStep_f32_0(
        &mut self,
        lastBaseEventRelativeBeat: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetBeatStep", (lastBaseEventRelativeBeat))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDistribution(
        &mut self,
        isFirstBaseDataEvent: bool,
        distributionOrderIndex: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetDistribution", (isFirstBaseDataEvent, distributionOrderIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        indexFilter: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IndexFilter>,
        beatDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        beatDistributionParam: f32,
        eventDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        eventDistributionParam: f32,
        eventDistributionShouldAffectFirstBaseEvent: bool,
        eventDistributionEaseType: crate::GlobalNamespace::EaseType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    indexFilter,
                    beatDistributionParamType,
                    beatDistributionParam,
                    eventDistributionParamType,
                    eventDistributionParam,
                    eventDistributionShouldAffectFirstBaseEvent,
                    eventDistributionEaseType,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn Unpack(
        &mut self,
        groupBoxBeat: f32,
        groupId: i32,
        elementId: i32,
        durationOrderIndex: i32,
        distributionOrderIndex: i32,
        maxBeat: f32,
        beatToTimeConverter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatToTimeConverter,
        >,
        lightEventConverter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLightEventConverter,
        >,
        output: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Unpack",
                (
                    groupBoxBeat,
                    groupId,
                    elementId,
                    durationOrderIndex,
                    distributionOrderIndex,
                    maxBeat,
                    beatToTimeConverter,
                    lightEventConverter,
                    output,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        indexFilter: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IndexFilter>,
        beatDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        beatDistributionParam: f32,
        eventDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        eventDistributionParam: f32,
        eventDistributionShouldAffectFirstBaseEvent: bool,
        eventDistributionEaseType: crate::GlobalNamespace::EaseType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    indexFilter,
                    beatDistributionParamType,
                    beatDistributionParam,
                    eventDistributionParamType,
                    eventDistributionParam,
                    eventDistributionShouldAffectFirstBaseEvent,
                    eventDistributionEaseType,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beatStep(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_beatStep", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_indexFilter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IndexFilter>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IndexFilter,
        > = __cordl_object.invoke("get_indexFilter", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_subtypeIdentifier(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_subtypeIdentifier", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapEventDataBox")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatmapEventDataBox {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapEventDataBox+DistributionParamType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BeatmapEventDataBox_DistributionParamType {
    #[default]
    Step = 2i32,
    Wave = 1i32,
}
#[cfg(feature = "BeatmapEventDataBox+DistributionParamType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapEventDataBox/DistributionParamType";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "BeatmapEventDataBox+DistributionParamType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "BeatmapEventDataBox+DistributionParamType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "BeatmapEventDataBox+DistributionParamType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "BeatmapEventDataBox+DistributionParamType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
