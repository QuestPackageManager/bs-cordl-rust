#[cfg(feature = "EnvironmentName")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct EnvironmentName {
    pub _environmentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "EnvironmentName")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::EnvironmentName => ""
    ."EnvironmentName"
);
#[cfg(feature = "EnvironmentName")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::EnvironmentName {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "EnvironmentName")]
impl crate::GlobalNamespace::EnvironmentName {
    pub fn Equals_EnvironmentName0(
        &mut self,
        other: crate::GlobalNamespace::EnvironmentName,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
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
        environmentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (environmentName),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        a: crate::GlobalNamespace::EnvironmentName,
        b: crate::GlobalNamespace::EnvironmentName,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_EnvironmentName0(
        e: crate::GlobalNamespace::EnvironmentName,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("op_Implicit", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_Il2CppString1(
        environmentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::EnvironmentName> {
        let __cordl_ret: crate::GlobalNamespace::EnvironmentName = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (environmentName))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        a: crate::GlobalNamespace::EnvironmentName,
        b: crate::GlobalNamespace::EnvironmentName,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (a, b))?;
        Ok(__cordl_ret.into())
    }
}
