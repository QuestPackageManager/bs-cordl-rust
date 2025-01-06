#[cfg(feature = "Unity+Profiling+ProfilerMarker")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ProfilerMarker {
    pub m_Ptr: crate::System::IntPtr,
}
#[cfg(feature = "Unity+Profiling+ProfilerMarker")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Profiling::ProfilerMarker =>
    "Unity.Profiling"."ProfilerMarker"
);
#[cfg(feature = "Unity+Profiling+ProfilerMarker")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Profiling::ProfilerMarker {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Profiling+ProfilerMarker")]
impl crate::Unity::Profiling::ProfilerMarker {
    #[cfg(feature = "Unity+Profiling+ProfilerMarker+AutoScope")]
    pub type AutoScope = crate::Unity::Profiling::ProfilerMarker_AutoScope;
    pub fn Auto(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Profiling::ProfilerMarker_AutoScope,
    > {
        let __cordl_ret: crate::Unity::Profiling::ProfilerMarker_AutoScope = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Auto",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString0(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (name),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ProfilerCategory_Il2CppString1(
        &mut self,
        category: crate::Unity::Profiling::ProfilerCategory,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (category, name),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Profiling+ProfilerMarker+AutoScope")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ProfilerMarker_AutoScope {
    pub m_Ptr: crate::System::IntPtr,
}
#[cfg(feature = "Unity+Profiling+ProfilerMarker+AutoScope")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Profiling::ProfilerMarker_AutoScope =>
    "Unity.Profiling"."ProfilerMarker/AutoScope"
);
#[cfg(feature = "Unity+Profiling+ProfilerMarker+AutoScope")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Profiling::ProfilerMarker_AutoScope {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Profiling+ProfilerMarker+AutoScope")]
impl crate::Unity::Profiling::ProfilerMarker_AutoScope {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        markerPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (markerPtr),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Profiling+ProfilerMarker+AutoScope")]
impl AsRef<crate::System::IDisposable>
for crate::Unity::Profiling::ProfilerMarker_AutoScope {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "Unity+Profiling+ProfilerMarker+AutoScope")]
impl AsMut<crate::System::IDisposable>
for crate::Unity::Profiling::ProfilerMarker_AutoScope {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
