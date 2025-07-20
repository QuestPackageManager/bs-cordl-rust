#[cfg(feature = "ProfilerMetrics")]
#[repr(C)]
#[derive(Debug)]
pub struct ProfilerMetrics {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _metrics: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::ProfilerMetrics_ProfilerMetric,
        >,
    >,
    pub _profilerRecorders: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::Unity::Profiling::ProfilerRecorder>,
    >,
    pub _samples: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<i64>,
    >,
}
#[cfg(feature = "ProfilerMetrics")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::ProfilerMetrics {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ProfilerMetrics";
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
#[cfg(feature = "ProfilerMetrics")]
impl std::ops::Deref for crate::GlobalNamespace::ProfilerMetrics {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ProfilerMetrics")]
impl std::ops::DerefMut for crate::GlobalNamespace::ProfilerMetrics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ProfilerMetrics")]
impl crate::GlobalNamespace::ProfilerMetrics {
    pub const kExposedMetricsCap: i32 = 256i32;
    #[cfg(feature = "ProfilerMetrics+ProfilerMetric")]
    pub type ProfilerMetric = crate::GlobalNamespace::ProfilerMetrics_ProfilerMetric;
    pub fn AddExposedMetrics(
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::GlobalNamespace::ProfilerMetrics_ProfilerMetric,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ProfilerMetrics as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        crate::GlobalNamespace::ProfilerMetrics_ProfilerMetric,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("AddExposedMetrics")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ProfilerMetrics as
                    quest_hook::libil2cpp::Type > ::class(), "AddExposedMetrics", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (list))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CaptureFrame(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ProfilerMetrics as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("CaptureFrame")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ProfilerMetrics as
                    quest_hook::libil2cpp::Type > ::class(), "CaptureFrame", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ProfilerMetrics as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ProfilerMetrics as
                    quest_hook::libil2cpp::Type > ::class(), "Dispose", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateListReport(
        &mut self,
        units: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ProfilerMetrics as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GenerateListReport")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ProfilerMetrics as
                    quest_hook::libil2cpp::Type > ::class(), "GenerateListReport", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (units))? };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateTableReport(
        &mut self,
        units: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ProfilerMetrics as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GenerateTableReport")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ProfilerMetrics as
                    quest_hook::libil2cpp::Type > ::class(), "GenerateTableReport",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (units))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        metrics: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::ProfilerMetrics_ProfilerMetric,
            >,
        >,
        profilerRecorders: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::Unity::Profiling::ProfilerRecorder>,
        >,
        initialFrameCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (metrics, profilerRecorders, initialFrameCapacity))?;
        Ok(__cordl_object.into())
    }
    pub fn Record(
        metrics: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::ProfilerMetrics_ProfilerMetric,
            >,
        >,
        initialFrameCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ProfilerMetrics>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ProfilerMetrics as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::GlobalNamespace::ProfilerMetrics_ProfilerMetric,
                        >,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ProfilerMetrics>,
                2usize,
            >("Record")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ProfilerMetrics as
                    quest_hook::libil2cpp::Type > ::class(), "Record", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ProfilerMetrics,
        > = unsafe { method.invoke_unchecked((), (metrics, initialFrameCapacity))? };
        Ok(__cordl_ret.into())
    }
    pub fn RecordAllMetrics(
        initialFrameCapacity: i32,
        listed: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ProfilerMetrics>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ProfilerMetrics as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, bool),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ProfilerMetrics>,
                2usize,
            >("RecordAllMetrics")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ProfilerMetrics as
                    quest_hook::libil2cpp::Type > ::class(), "RecordAllMetrics", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ProfilerMetrics,
        > = unsafe { method.invoke_unchecked((), (initialFrameCapacity, listed))? };
        Ok(__cordl_ret.into())
    }
    pub fn RecordFrameTimingMetrics(
        initialFrameCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ProfilerMetrics>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ProfilerMetrics as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ProfilerMetrics>,
                1usize,
            >("RecordFrameTimingMetrics")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ProfilerMetrics as
                    quest_hook::libil2cpp::Type > ::class(), "RecordFrameTimingMetrics",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ProfilerMetrics,
        > = unsafe { method.invoke_unchecked((), (initialFrameCapacity))? };
        Ok(__cordl_ret.into())
    }
    pub fn RecordListedMetrics(
        initialFrameCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ProfilerMetrics>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ProfilerMetrics as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ProfilerMetrics>,
                1usize,
            >("RecordListedMetrics")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ProfilerMetrics as
                    quest_hook::libil2cpp::Type > ::class(), "RecordListedMetrics",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ProfilerMetrics,
        > = unsafe { method.invoke_unchecked((), (initialFrameCapacity))? };
        Ok(__cordl_ret.into())
    }
    pub fn RecordMemoryMetrics(
        initialFrameCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ProfilerMetrics>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ProfilerMetrics as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ProfilerMetrics>,
                1usize,
            >("RecordMemoryMetrics")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ProfilerMetrics as
                    quest_hook::libil2cpp::Type > ::class(), "RecordMemoryMetrics",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ProfilerMetrics,
        > = unsafe { method.invoke_unchecked((), (initialFrameCapacity))? };
        Ok(__cordl_ret.into())
    }
    pub fn RecordRenderingMetrics(
        initialFrameCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ProfilerMetrics>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ProfilerMetrics as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ProfilerMetrics>,
                1usize,
            >("RecordRenderingMetrics")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ProfilerMetrics as
                    quest_hook::libil2cpp::Type > ::class(), "RecordRenderingMetrics",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ProfilerMetrics,
        > = unsafe { method.invoke_unchecked((), (initialFrameCapacity))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        metrics: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::ProfilerMetrics_ProfilerMetric,
            >,
        >,
        profilerRecorders: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::Unity::Profiling::ProfilerRecorder>,
        >,
        initialFrameCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ProfilerMetrics as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::GlobalNamespace::ProfilerMetrics_ProfilerMetric,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::Unity::Profiling::ProfilerRecorder,
                        >,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ProfilerMetrics as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (metrics, profilerRecorders, initialFrameCapacity),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ProfilerMetrics")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ProfilerMetrics {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ProfilerMetrics")]
impl AsRef<crate::System::IDisposable> for crate::GlobalNamespace::ProfilerMetrics {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ProfilerMetrics")]
impl AsMut<crate::System::IDisposable> for crate::GlobalNamespace::ProfilerMetrics {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ProfilerMetrics+ProfilerMetric")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ProfilerMetrics_ProfilerMetric {
    pub category: crate::Unity::Profiling::ProfilerCategory,
    pub name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "ProfilerMetrics+ProfilerMetric")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ProfilerMetrics_ProfilerMetric {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ProfilerMetrics/ProfilerMetric";
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
#[cfg(feature = "ProfilerMetrics+ProfilerMetric")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::ProfilerMetrics_ProfilerMetric {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "ProfilerMetrics+ProfilerMetric")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::ProfilerMetrics_ProfilerMetric {
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
#[cfg(feature = "ProfilerMetrics+ProfilerMetric")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::ProfilerMetrics_ProfilerMetric {
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
#[cfg(feature = "ProfilerMetrics+ProfilerMetric")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::ProfilerMetrics_ProfilerMetric {
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
#[cfg(feature = "ProfilerMetrics+ProfilerMetric")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::ProfilerMetrics_ProfilerMetric {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "ProfilerMetrics+ProfilerMetric")]
impl crate::GlobalNamespace::ProfilerMetrics_ProfilerMetric {}
