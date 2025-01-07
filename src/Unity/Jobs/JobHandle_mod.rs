#[cfg(feature = "Unity+Jobs+JobHandle")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct JobHandle {
    pub jobGroup: u64,
    pub version: i32,
}
#[cfg(feature = "Unity+Jobs+JobHandle")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Jobs::JobHandle {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Jobs";
    const CLASS_NAME: &'static str = "JobHandle";
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
unsafe impl quest_hook::libil2cpp::Argument for crate::Unity::Jobs::JobHandle {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter for crate::Unity::Jobs::JobHandle {
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
unsafe impl quest_hook::libil2cpp::Returned for crate::Unity::Jobs::JobHandle {
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
unsafe impl quest_hook::libil2cpp::Return for crate::Unity::Jobs::JobHandle {
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
#[cfg(feature = "Unity+Jobs+JobHandle")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Unity::Jobs::JobHandle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Jobs+JobHandle")]
impl crate::Unity::Jobs::JobHandle {
    pub fn CombineDependenciesInternal2(
        job0: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
        job1: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        let __cordl_ret: crate::Unity::Jobs::JobHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CombineDependenciesInternal2", (job0, job1))?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineDependenciesInternal2_Injected(
        job0: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
        job1: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CombineDependenciesInternal2_Injected", (job0, job1, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineDependenciesInternalPtr(
        jobs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        let __cordl_ret: crate::Unity::Jobs::JobHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CombineDependenciesInternalPtr", (jobs, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineDependenciesInternalPtr_Injected(
        jobs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        count: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CombineDependenciesInternalPtr_Injected", (jobs, count, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineDependencies_JobHandle_JobHandle0(
        job0: crate::Unity::Jobs::JobHandle,
        job1: crate::Unity::Jobs::JobHandle,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        let __cordl_ret: crate::Unity::Jobs::JobHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CombineDependencies", (job0, job1))?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineDependencies_NativeArray_1_1(
        jobs: crate::Unity::Collections::NativeArray_1<crate::Unity::Jobs::JobHandle>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        let __cordl_ret: crate::Unity::Jobs::JobHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CombineDependencies", (jobs))?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineDependencies_NativeSlice_1_2(
        jobs: crate::Unity::Collections::NativeSlice_1<crate::Unity::Jobs::JobHandle>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        let __cordl_ret: crate::Unity::Jobs::JobHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CombineDependencies", (jobs))?;
        Ok(__cordl_ret.into())
    }
    pub fn Complete(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Complete",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        other: crate::Unity::Jobs::JobHandle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleBatchedJobs() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ScheduleBatchedJobs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleBatchedJobsAndComplete(
        job: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ScheduleBatchedJobsAndComplete", (job))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleBatchedJobsAndIsCompleted(
        job: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ScheduleBatchedJobsAndIsCompleted", (job))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCompleted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsCompleted",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Jobs+JobHandle")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Jobs::JobHandle>>
for crate::Unity::Jobs::JobHandle {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::Unity::Jobs::JobHandle> {
        todo!()
    }
}
#[cfg(feature = "Unity+Jobs+JobHandle")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Jobs::JobHandle>>
for crate::Unity::Jobs::JobHandle {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Jobs::JobHandle> {
        todo!()
    }
}
