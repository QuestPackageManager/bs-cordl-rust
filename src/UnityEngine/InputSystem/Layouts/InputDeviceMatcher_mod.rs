#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceMatcher")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputDeviceMatcher {
    pub m_Patterns: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::System::Collections::Generic::KeyValuePair_2<
                crate::UnityEngine::InputSystem::Utilities::InternedString,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceMatcher")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Layouts::InputDeviceMatcher =>
    "UnityEngine.InputSystem.Layouts"."InputDeviceMatcher"
);
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceMatcher")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceMatcher")]
impl crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher {
    #[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceMatcher+MatcherJson")]
    pub type MatcherJson = crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher_MatcherJson;
    pub fn Equals_Gc1(
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
    pub fn Equals_InputDeviceMatcher0(
        &mut self,
        other: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FromDeviceDescription(
        deviceDescription: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromDeviceDescription", (deviceDescription))?;
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
    pub fn GetNumPropertiesIn(
        description: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNumPropertiesIn", (description))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchPercentage(
        &mut self,
        deviceDescription: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MatchPercentage",
            (deviceDescription),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchSingleProperty(
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MatchSingleProperty", (pattern, value))?;
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
    pub fn With(
        &mut self,
        key: crate::UnityEngine::InputSystem::Utilities::InternedString,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        supportRegex: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "With",
            (key, value, supportRegex),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithCapability<TValue>(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: TValue,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithCapability",
            (path, value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithDeviceClass(
        &mut self,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        supportRegex: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithDeviceClass",
            (pattern, supportRegex),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithInterface(
        &mut self,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        supportRegex: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithInterface",
            (pattern, supportRegex),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithManufacturer(
        &mut self,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        supportRegex: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithManufacturer",
            (pattern, supportRegex),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithProduct(
        &mut self,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        supportRegex: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithProduct",
            (pattern, supportRegex),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithVersion(
        &mut self,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        supportRegex: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithVersion",
            (pattern, supportRegex),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_empty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_empty",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_patterns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::KeyValuePair_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::KeyValuePair_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_patterns", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        left: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
        right: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        left: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
        right: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (left, right))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceMatcher")]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    >,
> for crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceMatcher")]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    >,
> for crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceMatcher+MatcherJson")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputDeviceMatcher_MatcherJson {
    pub interface: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub interfaces: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub deviceClass: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub deviceClasses: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub manufacturer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub manufacturers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub product: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub products: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub versions: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub capabilities: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::InputSystem::Layouts::MatcherJson_InputDeviceMatcher_Capability,
        >,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceMatcher+MatcherJson")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Layouts::InputDeviceMatcher_MatcherJson =>
    "UnityEngine.InputSystem.Layouts"."InputDeviceMatcher/MatcherJson"
);
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceMatcher+MatcherJson")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher_MatcherJson {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceMatcher+MatcherJson")]
impl crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher_MatcherJson {
    #[cfg(
        feature = "UnityEngine+InputSystem+Layouts+InputDeviceMatcher+MatcherJson+Capability"
    )]
    pub type Capability = crate::UnityEngine::InputSystem::Layouts::MatcherJson_InputDeviceMatcher_Capability;
    pub fn FromMatcher(
        matcher: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher_MatcherJson,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher_MatcherJson = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromMatcher", (matcher))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToMatcher(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToMatcher",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputDeviceMatcher+MatcherJson+Capability"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct MatcherJson_InputDeviceMatcher_Capability {
    pub path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputDeviceMatcher+MatcherJson+Capability"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Layouts::MatcherJson_InputDeviceMatcher_Capability =>
    "UnityEngine.InputSystem.Layouts"."InputDeviceMatcher/MatcherJson/Capability"
);
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputDeviceMatcher+MatcherJson+Capability"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Layouts::MatcherJson_InputDeviceMatcher_Capability {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputDeviceMatcher+MatcherJson+Capability"
)]
impl crate::UnityEngine::InputSystem::Layouts::MatcherJson_InputDeviceMatcher_Capability {}
