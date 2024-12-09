#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceMatcher")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputDeviceMatcher {
    pub m_Patterns: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::System::Collections::Generic::KeyValuePair_2<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
            *mut crate::System::Object,
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
    #[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceMatcher+__c")]
    pub type __c = crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher___c;
    #[cfg(
        feature = "UnityEngine+InputSystem+Layouts+InputDeviceMatcher+_get_patterns_d__4"
    )]
    pub type _get_patterns_d__4 = crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher__get_patterns_d__4;
    pub fn Equals_InputDeviceMatcher0(
        &mut self,
        other: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object1(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn With(
        &mut self,
        key: crate::UnityEngine::InputSystem::Utilities::InternedString,
        value: *mut crate::System::Object,
        supportRegex: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "With",
            (key, value, supportRegex),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithCapability<TValue>(
        &mut self,
        path: *mut crate::System::String,
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
        Ok(__cordl_ret)
    }
    pub fn WithDeviceClass(
        &mut self,
        pattern: *mut crate::System::String,
        supportRegex: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithDeviceClass",
            (pattern, supportRegex),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithInterface(
        &mut self,
        pattern: *mut crate::System::String,
        supportRegex: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithInterface",
            (pattern, supportRegex),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithManufacturer(
        &mut self,
        pattern: *mut crate::System::String,
        supportRegex: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithManufacturer",
            (pattern, supportRegex),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithProduct(
        &mut self,
        pattern: *mut crate::System::String,
        supportRegex: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithProduct",
            (pattern, supportRegex),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithVersion(
        &mut self,
        pattern: *mut crate::System::String,
        supportRegex: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithVersion",
            (pattern, supportRegex),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_empty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_empty",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_patterns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::System::Collections::Generic::KeyValuePair_2<
                *mut crate::System::String,
                *mut crate::System::Object,
            >,
        >,
    > {
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::System::Collections::Generic::KeyValuePair_2<
                *mut crate::System::String,
                *mut crate::System::Object,
            >,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_patterns", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceMatcher+MatcherJson")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputDeviceMatcher_MatcherJson {
    pub interface: *mut crate::System::String,
    pub interfaces: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub deviceClass: *mut crate::System::String,
    pub deviceClasses: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub manufacturer: *mut crate::System::String,
    pub manufacturers: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub product: *mut crate::System::String,
    pub products: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub version: *mut crate::System::String,
    pub versions: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub capabilities: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::Layouts::MatcherJson_InputDeviceMatcher_Capability,
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
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputDeviceMatcher+MatcherJson+Capability"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MatcherJson_InputDeviceMatcher_Capability {
    pub path: *mut crate::System::String,
    pub value: *mut crate::System::String,
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
