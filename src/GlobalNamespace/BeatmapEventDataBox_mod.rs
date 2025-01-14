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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    f32,
                    crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
                    i32,
                ),
                f32,
                3usize,
            >("BeatDistributionParamToStep")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "BeatDistributionParamToStep", 3usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method
                .invoke_unchecked((), (distributionParam, distributionParamType, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn EventDistributionParamToStep(
        index: i32,
        distributionParam: f32,
        distributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        count: i32,
        easeType: crate::GlobalNamespace::EaseType,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    f32,
                    crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
                    i32,
                    crate::GlobalNamespace::EaseType,
                ),
                f32,
                5usize,
            >("EventDistributionParamToStep")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EventDistributionParamToStep", 5usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (index, distributionParam, distributionParamType, count, easeType),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatStep_IndexFilter_BeatmapEventDataBox_DistributionParamType_f32_f32_1(
        indexFilter: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IndexFilter>,
        distributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        distributionParam: f32,
        lastBaseEventRelativeBeat: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IndexFilter>,
                    crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
                    f32,
                    f32,
                ),
                f32,
                4usize,
            >("GetBeatStep")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetBeatStep", 4usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        indexFilter,
                        distributionParamType,
                        distributionParam,
                        lastBaseEventRelativeBeat,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatStep_f32_0(
        &mut self,
        lastBaseEventRelativeBeat: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32), f32, 1usize>("GetBeatStep")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetBeatStep", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked(self, (lastBaseEventRelativeBeat))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDistribution(
        &mut self,
        isFirstBaseDataEvent: bool,
        distributionOrderIndex: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool, i32), f32, 2usize>("GetDistribution")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetDistribution", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked(self, (isFirstBaseDataEvent, distributionOrderIndex))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    f32,
                    i32,
                    i32,
                    i32,
                    i32,
                    f32,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IBeatToTimeConverter,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IBeatmapLightEventConverter,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapEventData,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                9usize,
            >("Unpack")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Unpack", 9usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
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
                )
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IndexFilter>,
                    crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
                    f32,
                    crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
                    f32,
                    bool,
                    crate::GlobalNamespace::EaseType,
                ),
                quest_hook::libil2cpp::Void,
                7usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        indexFilter,
                        beatDistributionParamType,
                        beatDistributionParam,
                        eventDistributionParamType,
                        eventDistributionParam,
                        eventDistributionShouldAffectFirstBaseEvent,
                        eventDistributionEaseType,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_beatStep(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_beatStep")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_beatStep", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_indexFilter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IndexFilter>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IndexFilter>,
                0usize,
            >("get_indexFilter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_indexFilter", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IndexFilter,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_subtypeIdentifier(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_subtypeIdentifier")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_subtypeIdentifier", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
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
