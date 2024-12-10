#[cfg(feature = "Mono+RuntimeGenericParamInfoHandle")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RuntimeGenericParamInfoHandle {
    pub value: *mut quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+RuntimeGenericParamInfoHandle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::RuntimeGenericParamInfoHandle => "Mono"
    ."RuntimeGenericParamInfoHandle"
);
#[cfg(feature = "Mono+RuntimeGenericParamInfoHandle")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Mono::RuntimeGenericParamInfoHandle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+RuntimeGenericParamInfoHandle")]
impl crate::Mono::RuntimeGenericParamInfoHandle {
    pub fn GetConstraints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "GetConstraints", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetConstraintsCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetConstraintsCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (ptr),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Attributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Reflection::GenericParameterAttributes,
    > {
        let __cordl_ret: crate::System::Reflection::GenericParameterAttributes = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Attributes",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Constraints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Constraints", ())?;
        Ok(__cordl_ret.into())
    }
}
