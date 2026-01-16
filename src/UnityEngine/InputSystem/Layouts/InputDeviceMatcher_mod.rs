#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceMatcher")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceMatcher")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Layouts";
    const CLASS_NAME: &'static str = "InputDeviceMatcher";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceMatcher")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceMatcher")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceMatcher")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceMatcher")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceMatcher")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher
{
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
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (obj))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_InputDeviceMatcher0(
        &mut self,
        other: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromDeviceDescription(
        deviceDescription: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription),
                        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
                        1usize,
                    >("FromDeviceDescription")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromDeviceDescription", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher =
            unsafe { cordl_method_info.invoke_unchecked((), (deviceDescription))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetHashCode",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetNumPropertiesIn(
        description: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription),
                        i32,
                        1usize,
                    >("GetNumPropertiesIn")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetNumPropertiesIn", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (description))? };
        Ok(__cordl_ret.into())
    }
    pub fn MatchPercentage(
        &mut self,
        deviceDescription: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription),
                        f32,
                        1usize,
                    >("MatchPercentage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MatchPercentage", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 =
            unsafe { cordl_method_info.invoke_unchecked(self, (deviceDescription))? };
        Ok(__cordl_ret.into())
    }
    pub fn MatchSingleProperty(
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), bool, 2usize>("MatchSingleProperty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "MatchSingleProperty",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (pattern, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn MatchSinglePropertyContains(
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), bool, 2usize>("MatchSinglePropertyContains")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "MatchSinglePropertyContains",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (pattern, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn With(
        &mut self,
        key: crate::UnityEngine::InputSystem::Utilities::InternedString,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        supportRegex: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        bool,
                    ), crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher, 3usize>(
                        "With"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "With",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher =
            unsafe { cordl_method_info.invoke_unchecked(self, (key, value, supportRegex))? };
        Ok(__cordl_ret.into())
    }
    pub fn WithCapability<TValue>(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: TValue,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher>
    where
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        TValue,
                    ), crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher, 2usize>(
                        "WithCapability",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WithCapability",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher =
            unsafe { cordl_method_info.invoke_unchecked(self, (path, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn WithDeviceClass(
        &mut self,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        supportRegex: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        bool,
                    ), crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher, 2usize>(
                        "WithDeviceClass",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WithDeviceClass",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher =
            unsafe { cordl_method_info.invoke_unchecked(self, (pattern, supportRegex))? };
        Ok(__cordl_ret.into())
    }
    pub fn WithInterface(
        &mut self,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        supportRegex: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        bool,
                    ), crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher, 2usize>(
                        "WithInterface",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WithInterface",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher =
            unsafe { cordl_method_info.invoke_unchecked(self, (pattern, supportRegex))? };
        Ok(__cordl_ret.into())
    }
    pub fn WithManufacturer(
        &mut self,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        supportRegex: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        bool,
                    ), crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher, 2usize>(
                        "WithManufacturer",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WithManufacturer",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher =
            unsafe { cordl_method_info.invoke_unchecked(self, (pattern, supportRegex))? };
        Ok(__cordl_ret.into())
    }
    pub fn WithManufacturerContains(
        &mut self,
        noRegExPattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
                        1usize,
                    >("WithManufacturerContains")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WithManufacturerContains", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher =
            unsafe { cordl_method_info.invoke_unchecked(self, (noRegExPattern))? };
        Ok(__cordl_ret.into())
    }
    pub fn WithProduct(
        &mut self,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        supportRegex: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        bool,
                    ), crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher, 2usize>(
                        "WithProduct",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WithProduct",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher =
            unsafe { cordl_method_info.invoke_unchecked(self, (pattern, supportRegex))? };
        Ok(__cordl_ret.into())
    }
    pub fn WithVersion(
        &mut self,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        supportRegex: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        bool,
                    ), crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher, 2usize>(
                        "WithVersion",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WithVersion",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher =
            unsafe { cordl_method_info.invoke_unchecked(self, (pattern, supportRegex))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_empty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_empty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_empty",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_patterns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::System::Collections::Generic::KeyValuePair_2<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<
                            crate::System::Collections::Generic::KeyValuePair_2<
                                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                            >,
                        >,
                    >, 0usize>("get_patterns")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_patterns",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::System::Collections::Generic::KeyValuePair_2<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        left: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
        right: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
                        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
                    ), bool, 2usize>("op_Equality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Equality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (left, right))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        left: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
        right: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
                        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
                    ), bool, 2usize>("op_Inequality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Inequality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (left, right))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceMatcher")]
impl
    AsRef<crate::System::IEquatable_1<crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher>>
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher
{
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher>
    {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceMatcher")]
impl
    AsMut<crate::System::IEquatable_1<crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher>>
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    > {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceMatcher+MatcherJson")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
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
    pub manufacturerContains: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceMatcher+MatcherJson")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher_MatcherJson
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Layouts";
    const CLASS_NAME: &'static str = "InputDeviceMatcher/MatcherJson";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceMatcher+MatcherJson")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher_MatcherJson
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceMatcher+MatcherJson")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher_MatcherJson
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceMatcher+MatcherJson")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher_MatcherJson
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceMatcher+MatcherJson")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher_MatcherJson
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceMatcher+MatcherJson")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher_MatcherJson
{
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
    #[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceMatcher+MatcherJson+Capability")]
    pub type Capability =
        crate::UnityEngine::InputSystem::Layouts::MatcherJson_InputDeviceMatcher_Capability;
    pub fn FromMatcher(
        matcher: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher_MatcherJson,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher),
                        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher_MatcherJson,
                        1usize,
                    >("FromMatcher")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromMatcher", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher_MatcherJson =
            unsafe { cordl_method_info.invoke_unchecked((), (matcher))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToMatcher(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
                        0usize,
                    >("ToMatcher")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToMatcher", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceMatcher+MatcherJson+Capability"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct MatcherJson_InputDeviceMatcher_Capability {
    pub path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceMatcher+MatcherJson+Capability"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Layouts::MatcherJson_InputDeviceMatcher_Capability
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Layouts";
    const CLASS_NAME: &'static str = "InputDeviceMatcher/MatcherJson/Capability";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceMatcher+MatcherJson+Capability"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::InputSystem::Layouts::MatcherJson_InputDeviceMatcher_Capability
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceMatcher+MatcherJson+Capability"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::InputSystem::Layouts::MatcherJson_InputDeviceMatcher_Capability
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceMatcher+MatcherJson+Capability"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::InputSystem::Layouts::MatcherJson_InputDeviceMatcher_Capability
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceMatcher+MatcherJson+Capability"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::InputSystem::Layouts::MatcherJson_InputDeviceMatcher_Capability
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceMatcher+MatcherJson+Capability"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::InputSystem::Layouts::MatcherJson_InputDeviceMatcher_Capability
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceMatcher+MatcherJson+Capability")]
impl crate::UnityEngine::InputSystem::Layouts::MatcherJson_InputDeviceMatcher_Capability {}
