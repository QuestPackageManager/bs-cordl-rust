#[cfg(feature = "Unity+Profiling+ProfilerCategory")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ProfilerCategory {
    padding: quest_hook::libil2cpp::ValueTypePadding<2usize>,
}
#[cfg(feature = "Unity+Profiling+ProfilerCategory")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Profiling::ProfilerCategory {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Profiling";
    const CLASS_NAME: &'static str = "ProfilerCategory";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::Profiling::ProfilerCategory {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::Profiling::ProfilerCategory {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::Profiling::ProfilerCategory {
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
unsafe impl quest_hook::libil2cpp::Return for crate::Unity::Profiling::ProfilerCategory {
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
#[cfg(feature = "Unity+Profiling+ProfilerCategory")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Profiling::ProfilerCategory {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Profiling+ProfilerCategory")]
impl crate::Unity::Profiling::ProfilerCategory {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        category: u16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (category),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Internal() -> quest_hook::libil2cpp::Result<
        crate::Unity::Profiling::ProfilerCategory,
    > {
        let __cordl_ret: crate::Unity::Profiling::ProfilerCategory = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Internal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Memory() -> quest_hook::libil2cpp::Result<
        crate::Unity::Profiling::ProfilerCategory,
    > {
        let __cordl_ret: crate::Unity::Profiling::ProfilerCategory = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Memory", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Render() -> quest_hook::libil2cpp::Result<
        crate::Unity::Profiling::ProfilerCategory,
    > {
        let __cordl_ret: crate::Unity::Profiling::ProfilerCategory = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Render", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Scripts() -> quest_hook::libil2cpp::Result<
        crate::Unity::Profiling::ProfilerCategory,
    > {
        let __cordl_ret: crate::Unity::Profiling::ProfilerCategory = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Scripts", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        category: crate::Unity::Profiling::ProfilerCategory,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (category))?;
        Ok(__cordl_ret.into())
    }
}
