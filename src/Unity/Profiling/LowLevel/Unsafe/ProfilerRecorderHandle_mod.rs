#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerRecorderHandle")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ProfilerRecorderHandle {
    padding: quest_hook::libil2cpp::ValueTypePadding<8usize>,
}
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerRecorderHandle")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Profiling.LowLevel.Unsafe";
    const CLASS_NAME: &'static str = "ProfilerRecorderHandle";
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
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerRecorderHandle")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerRecorderHandle")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle {
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
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerRecorderHandle")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle {
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
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerRecorderHandle")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle {
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
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerRecorderHandle")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerRecorderHandle")]
impl crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle {
    pub fn GetAvailable(
        outRecorderHandleList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAvailable", (outRecorderHandleList))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetByName(
        category: crate::Unity::Profiling::ProfilerCategory,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nameLen: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
    > {
        let __cordl_ret: crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetByName", (category, name, nameLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetByName_Unsafe(
        category: crate::Unity::Profiling::ProfilerCategory,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nameLen: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
    > {
        let __cordl_ret: crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetByName_Unsafe", (category, name, nameLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetByName_Unsafe_Injected(
        category: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Profiling::ProfilerCategory,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nameLen: i32,
        ret: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetByName_Unsafe_Injected", (category, name, nameLen, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetByName__Unmanaged(
        category: crate::Unity::Profiling::ProfilerCategory,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nameLen: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
    > {
        let __cordl_ret: crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetByName__Unmanaged", (category, name, nameLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetByName__Unmanaged_Injected(
        category: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Profiling::ProfilerCategory,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nameLen: i32,
        ret: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetByName__Unmanaged_Injected", (category, name, nameLen, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDescription(
        handle: crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderDescription,
    > {
        let __cordl_ret: crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderDescription = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDescription", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDescriptionInternal(
        handle: crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderDescription,
    > {
        let __cordl_ret: crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderDescription = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDescriptionInternal", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDescriptionInternal_Injected(
        handle: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
        >,
        ret: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderDescription,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDescriptionInternal_Injected", (handle, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Valid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Valid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
