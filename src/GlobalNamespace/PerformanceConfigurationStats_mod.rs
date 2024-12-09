#[cfg(feature = "PerformanceConfigurationStats")]
#[repr(C)]
#[derive(Debug)]
pub struct PerformanceConfigurationStats {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub processorFrequency: crate::GlobalNamespace::PerformanceConfigurationStats_IntStats,
    pub batteryStatus: crate::GlobalNamespace::PerformanceConfigurationStats_EnumStats_1<
        crate::UnityEngine::BatteryStatus,
    >,
    pub batteryLevel: crate::GlobalNamespace::PerformanceConfigurationStats_FloatStats,
    pub internetReachability: crate::GlobalNamespace::PerformanceConfigurationStats_EnumStats_1<
        crate::UnityEngine::NetworkReachability,
    >,
    pub gpuUtilLevel: crate::GlobalNamespace::PerformanceConfigurationStats_FloatStats,
    pub powerSaving: crate::GlobalNamespace::PerformanceConfigurationStats_BoolStats,
    pub boundaryVisible: crate::GlobalNamespace::PerformanceConfigurationStats_BoolStats,
}
#[cfg(feature = "PerformanceConfigurationStats")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PerformanceConfigurationStats
    => ""."PerformanceConfigurationStats"
);
#[cfg(feature = "PerformanceConfigurationStats")]
impl std::ops::Deref for crate::GlobalNamespace::PerformanceConfigurationStats {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PerformanceConfigurationStats")]
impl std::ops::DerefMut for crate::GlobalNamespace::PerformanceConfigurationStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PerformanceConfigurationStats")]
impl crate::GlobalNamespace::PerformanceConfigurationStats {
    #[cfg(feature = "PerformanceConfigurationStats+BoolStats")]
    pub type BoolStats = crate::GlobalNamespace::PerformanceConfigurationStats_BoolStats;
    #[cfg(feature = "PerformanceConfigurationStats+EnumStats_1")]
    pub type EnumStats_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::PerformanceConfigurationStats_EnumStats_1<
        T,
    >;
    #[cfg(feature = "PerformanceConfigurationStats+FloatStats")]
    pub type FloatStats = crate::GlobalNamespace::PerformanceConfigurationStats_FloatStats;
    #[cfg(feature = "PerformanceConfigurationStats+IntStats")]
    pub type IntStats = crate::GlobalNamespace::PerformanceConfigurationStats_IntStats;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PerformanceConfigurationStats")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PerformanceConfigurationStats {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PerformanceConfigurationStats+BoolStats")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PerformanceConfigurationStats_BoolStats {
    pub off: i32,
    pub on: i32,
}
#[cfg(feature = "PerformanceConfigurationStats+BoolStats")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PerformanceConfigurationStats_BoolStats => ""
    ."PerformanceConfigurationStats/BoolStats"
);
#[cfg(feature = "PerformanceConfigurationStats+BoolStats")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PerformanceConfigurationStats_BoolStats {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PerformanceConfigurationStats+BoolStats")]
impl crate::GlobalNamespace::PerformanceConfigurationStats_BoolStats {
    pub fn CreateLogValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CreateLogValue",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Update",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PerformanceConfigurationStats+EnumStats_1")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PerformanceConfigurationStats_EnumStats_1<T: quest_hook::libil2cpp::Type> {
    pub flags: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "PerformanceConfigurationStats+EnumStats_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PerformanceConfigurationStats_EnumStats_1 < T > => ""
    ."PerformanceConfigurationStats/EnumStats`1<T>" < T >
);
#[cfg(feature = "PerformanceConfigurationStats+EnumStats_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PerformanceConfigurationStats_EnumStats_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PerformanceConfigurationStats+EnumStats_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::PerformanceConfigurationStats_EnumStats_1<T> {
    pub fn CreateLogValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CreateLogValue",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Update",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PerformanceConfigurationStats+FloatStats")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PerformanceConfigurationStats_FloatStats {
    pub min: f32,
    pub max: f32,
}
#[cfg(feature = "PerformanceConfigurationStats+FloatStats")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PerformanceConfigurationStats_FloatStats => ""
    ."PerformanceConfigurationStats/FloatStats"
);
#[cfg(feature = "PerformanceConfigurationStats+FloatStats")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PerformanceConfigurationStats_FloatStats {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PerformanceConfigurationStats+FloatStats")]
impl crate::GlobalNamespace::PerformanceConfigurationStats_FloatStats {
    pub fn CreateLogValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CreateLogValue",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Update",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PerformanceConfigurationStats+IntStats")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PerformanceConfigurationStats_IntStats {
    pub min: i32,
    pub max: i32,
}
#[cfg(feature = "PerformanceConfigurationStats+IntStats")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PerformanceConfigurationStats_IntStats => ""
    ."PerformanceConfigurationStats/IntStats"
);
#[cfg(feature = "PerformanceConfigurationStats+IntStats")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PerformanceConfigurationStats_IntStats {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PerformanceConfigurationStats+IntStats")]
impl crate::GlobalNamespace::PerformanceConfigurationStats_IntStats {
    pub fn CreateLogValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CreateLogValue",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Update",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
