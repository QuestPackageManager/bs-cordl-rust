#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Builder+ControlBuilder"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct Builder_InputControlLayout_ControlBuilder {
    pub builder: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
    >,
    pub index: i32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Builder+ControlBuilder"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Layouts";
    const CLASS_NAME: &'static str = "InputControlLayout/Builder/ControlBuilder";
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
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Builder+ControlBuilder"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder
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
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Builder+ControlBuilder"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder
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
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Builder+ControlBuilder"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder
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
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Builder+ControlBuilder"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder
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
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Builder+ControlBuilder"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Builder+ControlBuilder")]
impl crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder {
    pub fn AsArrayOfControlsWithSize(
        &mut self,
        arraySize: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
                        1usize,
                    >("AsArrayOfControlsWithSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AsArrayOfControlsWithSize", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = unsafe {
            cordl_method_info.invoke_unchecked(self, (arraySize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DontReset(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool),
                        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
                        1usize,
                    >("DontReset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DontReset", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsNoisy(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool),
                        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
                        1usize,
                    >("IsNoisy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "IsNoisy",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsSynthetic(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool),
                        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
                        1usize,
                    >("IsSynthetic")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsSynthetic", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UsingStateFrom(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
                        1usize,
                    >("UsingStateFrom")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UsingStateFrom", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = unsafe {
            cordl_method_info.invoke_unchecked(self, (path))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithBitOffset(
        &mut self,
        bit: u32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u32),
                        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
                        1usize,
                    >("WithBitOffset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WithBitOffset", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = unsafe {
            cordl_method_info.invoke_unchecked(self, (bit))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithByteOffset(
        &mut self,
        offset: u32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u32),
                        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
                        1usize,
                    >("WithByteOffset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WithByteOffset", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = unsafe {
            cordl_method_info.invoke_unchecked(self, (offset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithDefaultState(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::Utilities::PrimitiveValue),
                        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
                        1usize,
                    >("WithDefaultState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WithDefaultState", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithDisplayName(
        &mut self,
        displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
                        1usize,
                    >("WithDisplayName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WithDisplayName", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = unsafe {
            cordl_method_info.invoke_unchecked(self, (displayName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithFormat_FourCC0(
        &mut self,
        format: crate::UnityEngine::InputSystem::Utilities::FourCC,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::Utilities::FourCC),
                        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
                        1usize,
                    >("WithFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WithFormat", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = unsafe {
            cordl_method_info.invoke_unchecked(self, (format))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithFormat_Il2CppString1(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
                        1usize,
                    >("WithFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WithFormat", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = unsafe {
            cordl_method_info.invoke_unchecked(self, (format))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithLayout(
        &mut self,
        layout: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
                        1usize,
                    >("WithLayout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WithLayout", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = unsafe {
            cordl_method_info.invoke_unchecked(self, (layout))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithParameters(
        &mut self,
        parameters: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
                        1usize,
                    >("WithParameters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WithParameters", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = unsafe {
            cordl_method_info.invoke_unchecked(self, (parameters))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithProcessors(
        &mut self,
        processors: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
                        1usize,
                    >("WithProcessors")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WithProcessors", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = unsafe {
            cordl_method_info.invoke_unchecked(self, (processors))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithRange(
        &mut self,
        minValue: f32,
        maxValue: f32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (f32, f32),
                        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
                        2usize,
                    >("WithRange")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WithRange", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = unsafe {
            cordl_method_info.invoke_unchecked(self, (minValue, maxValue))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithSizeInBits(
        &mut self,
        sizeInBits: u32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u32),
                        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
                        1usize,
                    >("WithSizeInBits")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WithSizeInBits", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = unsafe {
            cordl_method_info.invoke_unchecked(self, (sizeInBits))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithUsages_IEnumerable_1_1(
        &mut self,
        usages: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppString,
                                >,
                            >,
                        >),
                        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
                        1usize,
                    >("WithUsages")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WithUsages", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = unsafe {
            cordl_method_info.invoke_unchecked(self, (usages))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithUsages_Il2CppArray0(
        &mut self,
        usages: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::InputSystem::Utilities::InternedString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::InputSystem::Utilities::InternedString,
                            >,
                        >),
                        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
                        1usize,
                    >("WithUsages")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WithUsages", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = unsafe {
            cordl_method_info.invoke_unchecked(self, (usages))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithUsages_Il2CppArray2(
        &mut self,
        usages: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppString,
                                >,
                            >,
                        >),
                        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
                        1usize,
                    >("WithUsages")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WithUsages", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = unsafe {
            cordl_method_info.invoke_unchecked(self, (usages))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Collection+LayoutMatcher"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct Collection_InputControlLayout_LayoutMatcher {
    pub layoutName: crate::UnityEngine::InputSystem::Utilities::InternedString,
    pub deviceMatcher: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
}
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Collection+LayoutMatcher"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Layouts::Collection_InputControlLayout_LayoutMatcher
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Layouts";
    const CLASS_NAME: &'static str = "InputControlLayout/Collection/LayoutMatcher";
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
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Collection+LayoutMatcher"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::InputSystem::Layouts::Collection_InputControlLayout_LayoutMatcher
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
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Collection+LayoutMatcher"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::InputSystem::Layouts::Collection_InputControlLayout_LayoutMatcher
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
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Collection+LayoutMatcher"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::InputSystem::Layouts::Collection_InputControlLayout_LayoutMatcher
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
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Collection+LayoutMatcher"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::InputSystem::Layouts::Collection_InputControlLayout_LayoutMatcher
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
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Collection+LayoutMatcher"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::InputSystem::Layouts::Collection_InputControlLayout_LayoutMatcher
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Collection+LayoutMatcher")]
impl crate::UnityEngine::InputSystem::Layouts::Collection_InputControlLayout_LayoutMatcher {}
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Collection+PrecompiledLayout"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct Collection_InputControlLayout_PrecompiledLayout {
    pub factoryMethod: quest_hook::libil2cpp::Gc<
        crate::System::Func_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        >,
    >,
    pub metadata: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Collection+PrecompiledLayout"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Layouts::Collection_InputControlLayout_PrecompiledLayout
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Layouts";
    const CLASS_NAME: &'static str = "InputControlLayout/Collection/PrecompiledLayout";
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
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Collection+PrecompiledLayout"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::InputSystem::Layouts::Collection_InputControlLayout_PrecompiledLayout
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
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Collection+PrecompiledLayout"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::InputSystem::Layouts::Collection_InputControlLayout_PrecompiledLayout
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
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Collection+PrecompiledLayout"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::InputSystem::Layouts::Collection_InputControlLayout_PrecompiledLayout
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
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Collection+PrecompiledLayout"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::InputSystem::Layouts::Collection_InputControlLayout_PrecompiledLayout
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
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Collection+PrecompiledLayout"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::InputSystem::Layouts::Collection_InputControlLayout_PrecompiledLayout
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Collection+PrecompiledLayout")]
impl crate::UnityEngine::InputSystem::Layouts::Collection_InputControlLayout_PrecompiledLayout {}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItem+Flags")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum ControlItem_InputControlLayout_Flags {
    #[default]
    DontReset = 16i32,
    IsFirstDefinedInThisLayout = 8i32,
    IsNoisy = 2i32,
    IsSynthetic = 4i32,
    isModifyingExistingControl = 1i32,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItem+Flags")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Layouts::ControlItem_InputControlLayout_Flags
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Layouts";
    const CLASS_NAME: &'static str = "InputControlLayout/ControlItem/Flags";
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItem+Flags")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::InputSystem::Layouts::ControlItem_InputControlLayout_Flags
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItem+Flags")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::InputSystem::Layouts::ControlItem_InputControlLayout_Flags
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItem+Flags")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::InputSystem::Layouts::ControlItem_InputControlLayout_Flags
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItem+Flags")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::InputSystem::Layouts::ControlItem_InputControlLayout_Flags
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout")]
#[repr(C)]
#[derive(Debug)]
pub struct InputControlLayout {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Name: crate::UnityEngine::InputSystem::Utilities::InternedString,
    pub m_Type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub m_Variants: crate::UnityEngine::InputSystem::Utilities::InternedString,
    pub m_StateFormat: crate::UnityEngine::InputSystem::Utilities::FourCC,
    pub m_StateSizeInBytes: i32,
    pub m_UpdateBeforeRender: crate::System::Nullable_1<bool>,
    pub m_BaseLayouts: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    >,
    pub m_AppliedOverrides: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    >,
    pub m_CommonUsages: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        >,
    >,
    pub m_Controls: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        >,
    >,
    pub m_DisplayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_Description: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_Flags: crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Flags,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Layouts";
    const CLASS_NAME: &'static str = "InputControlLayout";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Layouts::InputControlLayout {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::Layouts::InputControlLayout {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout")]
impl crate::UnityEngine::InputSystem::Layouts::InputControlLayout {
    pub const VariantSeparator: &'static str = ";";
    #[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Builder")]
    pub type Builder = crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder;
    #[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Cache")]
    pub type Cache = crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Cache;
    #[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+CacheRefInstance")]
    pub type CacheRefInstance =
        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_CacheRefInstance;
    #[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Collection")]
    pub type Collection = crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Collection;
    #[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItem")]
    pub type ControlItem = crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem;
    #[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItemJson")]
    pub type ControlItemJson =
        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItemJson;
    #[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Flags")]
    pub type Flags = crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Flags;
    #[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutJson")]
    pub type LayoutJson = crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJson;
    #[cfg(
        feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutJsonNameAndDescriptorOnly"
    )]
    pub type LayoutJsonNameAndDescriptorOnly = crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJsonNameAndDescriptorOnly;
    #[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutNotFoundException")]
    pub type LayoutNotFoundException =
        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutNotFoundException;
    pub fn AddControlItems(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        controlLayouts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
            >,
        >,
        layoutName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("AddControlItems")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddControlItems", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_cordl_type, controlLayouts, layoutName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddControlItemsFromFields(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        controlLayouts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
            >,
        >,
        layoutName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("AddControlItemsFromFields")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddControlItemsFromFields", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_cordl_type, controlLayouts, layoutName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddControlItemsFromMember(
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Layouts::InputControlAttribute,
                >,
            >,
        >,
        controlItems: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Reflection::MemberInfo,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::InputSystem::Layouts::InputControlAttribute,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("AddControlItemsFromMember")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddControlItemsFromMember", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (member, attributes, controlItems))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddControlItemsFromMembers(
        members: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
            >,
        >,
        controlItems: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
            >,
        >,
        layoutName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Reflection::MemberInfo,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("AddControlItemsFromMembers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddControlItemsFromMembers", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (members, controlItems, layoutName))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddControlItemsFromProperties(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        controlLayouts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
            >,
        >,
        layoutName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("AddControlItemsFromProperties")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddControlItemsFromProperties", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_cordl_type, controlLayouts, layoutName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CacheRef() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_CacheRefInstance,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_CacheRefInstance,
                        0usize,
                    >("CacheRef")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CacheRef", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputControlLayout_CacheRefInstance = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateControlItemFromMember(
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        attribute: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlAttribute,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Reflection::MemberInfo,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::Layouts::InputControlAttribute,
                            >,
                        ),
                        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
                        2usize,
                    >("CreateControlItemFromMember")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateControlItemFromMember", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem =
            unsafe { cordl_method_info.invoke_unchecked((), (member, attribute))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateLookupTableForControls(
        controlItems: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
            >,
        >,
        variants: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppString,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::Dictionary_2<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppString,
                                >,
                                crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
                            >,
                        >,
                        2usize,
                    >("CreateLookupTableForControls")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateLookupTableForControls", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (controlItems, variants))? };
        Ok(__cordl_ret.into())
    }
    pub fn FindControl(
        &mut self,
        path: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::Utilities::InternedString),
                        crate::System::Nullable_1<
                            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
                        >,
                        1usize,
                    >("FindControl")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FindControl", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (path))? };
        Ok(__cordl_ret.into())
    }
    pub fn FindControlIncludingArrayElements(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arrayIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), crate::System::Nullable_1<
                        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
                    >, 2usize>("FindControlIncludingArrayElements")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FindControlIncludingArrayElements",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (path, arrayIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromJson(
        json: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Layouts::InputControlLayout>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
                        >,
                        1usize,
                    >("FromJson")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromJson", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        > = unsafe { cordl_method_info.invoke_unchecked((), (json))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromType(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Layouts::InputControlLayout>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                    ), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
                    >, 2usize>("FromType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FromType",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        > = unsafe { cordl_method_info.invoke_unchecked((), (name, _cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetValueType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<crate::System::Type>, 0usize>(
                        "GetValueType",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetValueType",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn InferLayoutFromValueType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("InferLayoutFromValueType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InferLayoutFromValueType", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn MergeLayout(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
                    >), quest_hook::libil2cpp::Void, 1usize>("MergeLayout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "MergeLayout",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, _cordl_type))?;
        Ok(__cordl_object.into())
    }
    pub fn ParseHeaderFieldsFromJson(
        json: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        name: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        >,
        baseLayouts: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<
                crate::UnityEngine::InputSystem::Utilities::InternedString,
            >,
        >,
        deviceMatcher: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<
                                crate::UnityEngine::InputSystem::Utilities::InternedString,
                            >,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "ParseHeaderFieldsFromJson"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ParseHeaderFieldsFromJson",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (json, name, baseLayouts, deviceMatcher))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToJson(
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
                    >("ToJson")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "ToJson",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn VariantsMatch_Il2CppString_Il2CppString1(
        expected: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        actual: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), bool, 2usize>("VariantsMatch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "VariantsMatch",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (expected, actual))? };
        Ok(__cordl_ret.into())
    }
    pub fn VariantsMatch_InternedString_InternedString0(
        expected: crate::UnityEngine::InputSystem::Utilities::InternedString,
        actual: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                    ), bool, 2usize>("VariantsMatch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "VariantsMatch",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (expected, actual))? };
        Ok(__cordl_ret.into())
    }
    pub fn _MergeLayout_b__77_0(
        &mut self,
        x: crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem),
                        bool,
                        1usize,
                    >("<MergeLayout>b__77_0")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "<MergeLayout>b__77_0", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (x))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (name, _cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultVariant(
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Utilities::InternedString>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                        0usize,
                    >("get_DefaultVariant")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_DefaultVariant", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::InternedString =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
                        1usize,
                    >("get_Item")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Item", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem =
            unsafe { cordl_method_info.invoke_unchecked(self, (path))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_appliedOverrides(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::InputSystem::Utilities::InternedString,
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
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                        >,
                    >, 0usize>("get_appliedOverrides")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_appliedOverrides",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::InputSystem::Utilities::InternedString,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_baseLayouts(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::InputSystem::Utilities::InternedString,
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
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                        >,
                    >, 0usize>("get_baseLayouts")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_baseLayouts",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::InputSystem::Utilities::InternedString,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_cache() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Cache,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Cache,
                    >, 0usize>("get_cache")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_cache",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Cache,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_canRunInBackground(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<bool>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::System::Nullable_1<bool>, 0usize>(
                        "get_canRunInBackground",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_canRunInBackground",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<bool> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_commonUsages(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                    >, 0usize>("get_commonUsages")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_commonUsages",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_controls(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
                    >, 0usize>("get_controls")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_controls",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_displayName(
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
                    >("get_displayName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_displayName", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_hideInUI(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_hideInUI")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_hideInUI",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isControlLayout(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_isControlLayout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_isControlLayout",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isDeviceLayout(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_isDeviceLayout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_isDeviceLayout",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isGenericTypeOfDevice(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_isGenericTypeOfDevice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_isGenericTypeOfDevice",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isNoisy(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_isNoisy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_isNoisy",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isOverride(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_isOverride")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_isOverride",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Utilities::InternedString>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                        0usize,
                    >("get_name")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_name", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::InternedString =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_stateFormat(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Utilities::FourCC> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::InputSystem::Utilities::FourCC, 0usize>(
                        "get_stateFormat",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_stateFormat",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_stateSizeInBytes(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_stateSizeInBytes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_stateSizeInBytes",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<crate::System::Type>, 0usize>(
                        "get_type",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_type",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_updateBeforeRender(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_updateBeforeRender")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_updateBeforeRender",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_variants(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Utilities::InternedString>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                        0usize,
                    >("get_variants")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_variants", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::InternedString =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_canRunInBackground(
        &mut self,
        value: crate::System::Nullable_1<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Nullable_1<bool>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_canRunInBackground")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_canRunInBackground", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_hideInUI(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("set_hideInUI")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_hideInUI",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_isGenericTypeOfDevice(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "set_isGenericTypeOfDevice",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_isGenericTypeOfDevice",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_isNoisy(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("set_isNoisy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_isNoisy",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_isOverride(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("set_isOverride")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_isOverride",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Builder")]
#[repr(C)]
#[derive(Debug)]
pub struct InputControlLayout_Builder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _name_k__BackingField: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _displayName_k__BackingField:
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _type_k__BackingField: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub _stateFormat_k__BackingField: crate::UnityEngine::InputSystem::Utilities::FourCC,
    pub _stateSizeInBytes_k__BackingField: i32,
    pub m_ExtendsLayout: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _updateBeforeRender_k__BackingField: crate::System::Nullable_1<bool>,
    pub m_ControlCount: i32,
    pub m_Controls: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        >,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Builder")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Layouts";
    const CLASS_NAME: &'static str = "InputControlLayout/Builder";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Builder")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Builder")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Builder")]
impl crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder {
    #[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Builder+ControlBuilder")]
    pub type ControlBuilder =
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder;
    pub fn AddControl(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
                        1usize,
                    >("AddControl")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddControl", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = unsafe {
            cordl_method_info.invoke_unchecked(self, (name))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Build(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Layouts::InputControlLayout>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
                    >, 0usize>("Build")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Build",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Extend(
        &mut self,
        baseLayoutName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
                        >,
                        1usize,
                    >("Extend")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Extend",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (baseLayoutName))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn WithDisplayName(
        &mut self,
        displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
                        >,
                        1usize,
                    >("WithDisplayName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WithDisplayName", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (displayName))? };
        Ok(__cordl_ret.into())
    }
    pub fn WithFormat_FourCC0(
        &mut self,
        format: crate::UnityEngine::InputSystem::Utilities::FourCC,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::Utilities::FourCC),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
                        >,
                        1usize,
                    >("WithFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WithFormat", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn WithFormat_Il2CppString1(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
                        >,
                        1usize,
                    >("WithFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WithFormat", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn WithName(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
                        >,
                        1usize,
                    >("WithName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WithName", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn WithSizeInBytes(
        &mut self,
        sizeInBytes: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
                    >, 1usize>("WithSizeInBytes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WithSizeInBytes",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (sizeInBytes))? };
        Ok(__cordl_ret.into())
    }
    pub fn WithType<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
                    >, 0usize>("WithType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WithType",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_controls(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
                    >, 0usize>("get_controls")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_controls",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_displayName(
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
                    >("get_displayName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_displayName", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_extendsLayout(
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
                    >("get_extendsLayout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_extendsLayout", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_name(
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
                    >("get_name")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_name", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_stateFormat(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Utilities::FourCC> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::InputSystem::Utilities::FourCC, 0usize>(
                        "get_stateFormat",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_stateFormat",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_stateSizeInBytes(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_stateSizeInBytes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_stateSizeInBytes",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<crate::System::Type>, 0usize>(
                        "get_type",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_type",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_updateBeforeRender(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<bool>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::System::Nullable_1<bool>, 0usize>(
                        "get_updateBeforeRender",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_updateBeforeRender",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<bool> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_displayName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_displayName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_displayName", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_extendsLayout(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_extendsLayout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_extendsLayout", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_name(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_name")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_name", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_stateFormat(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::FourCC,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::Utilities::FourCC),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_stateFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_stateFormat", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_stateSizeInBytes(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(
                        "set_stateSizeInBytes",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_stateSizeInBytes",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_type(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_type")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_type", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_updateBeforeRender(
        &mut self,
        value: crate::System::Nullable_1<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Nullable_1<bool>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_updateBeforeRender")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_updateBeforeRender", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Builder")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Cache")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct InputControlLayout_Cache {
    pub table: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Layouts::InputControlLayout>,
        >,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Cache")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Cache
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Layouts";
    const CLASS_NAME: &'static str = "InputControlLayout/Cache";
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Cache")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Cache
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Cache")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Cache
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Cache")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Cache
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Cache")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Cache
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Cache")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Cache
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Cache")]
impl crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Cache {
    pub fn Clear(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Clear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Clear",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn FindOrLoadLayout(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        throwIfNotFound: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Layouts::InputControlLayout>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        bool,
                    ), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
                    >, 2usize>("FindOrLoadLayout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FindOrLoadLayout",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (name, throwIfNotFound))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+CacheRefInstance")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct InputControlLayout_CacheRefInstance {
    pub valid: bool,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+CacheRefInstance")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_CacheRefInstance
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Layouts";
    const CLASS_NAME: &'static str = "InputControlLayout/CacheRefInstance";
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+CacheRefInstance")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_CacheRefInstance
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+CacheRefInstance")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_CacheRefInstance
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+CacheRefInstance")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_CacheRefInstance
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+CacheRefInstance")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_CacheRefInstance
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+CacheRefInstance")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_CacheRefInstance
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+CacheRefInstance")]
impl crate::UnityEngine::InputSystem::Layouts::InputControlLayout_CacheRefInstance {
    pub fn Dispose(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+CacheRefInstance")]
impl AsRef<crate::System::IDisposable>
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_CacheRefInstance
{
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+CacheRefInstance")]
impl AsMut<crate::System::IDisposable>
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_CacheRefInstance
{
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Collection")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct InputControlLayout_Collection {
    pub layoutTypes: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
            quest_hook::libil2cpp::Gc<crate::System::Type>,
        >,
    >,
    pub layoutStrings: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub layoutBuilders: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
            quest_hook::libil2cpp::Gc<
                crate::System::Func_1<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
                    >,
                >,
            >,
        >,
    >,
    pub baseLayoutTable: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        >,
    >,
    pub layoutOverrides: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                >,
            >,
        >,
    >,
    pub layoutOverrideNames: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        >,
    >,
    pub precompiledLayouts: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
            crate::UnityEngine::InputSystem::Layouts::Collection_InputControlLayout_PrecompiledLayout,
        >,
    >,
    pub layoutMatchers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::InputSystem::Layouts::Collection_InputControlLayout_LayoutMatcher,
        >,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Collection")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Collection
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Layouts";
    const CLASS_NAME: &'static str = "InputControlLayout/Collection";
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Collection")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Collection
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Collection")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Collection
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Collection")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Collection
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Collection")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Collection
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Collection")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Collection
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Collection")]
impl crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Collection {
    pub const kBaseScoreForNonGeneratedLayouts: f32 = 1f32;
    #[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Collection+LayoutMatcher")]
    pub type LayoutMatcher =
        crate::UnityEngine::InputSystem::Layouts::Collection_InputControlLayout_LayoutMatcher;
    #[cfg(
        feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Collection+PrecompiledLayout"
    )]
    pub type PrecompiledLayout =
        crate::UnityEngine::InputSystem::Layouts::Collection_InputControlLayout_PrecompiledLayout;
    pub fn AddMatcher(
        &mut self,
        layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        matcher: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                        crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
                    ), quest_hook::libil2cpp::Void, 2usize>("AddMatcher")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddMatcher",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (layout, matcher))? };
        Ok(__cordl_ret.into())
    }
    pub fn Allocate(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Allocate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Allocate",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeDistanceInInheritanceHierarchy(
        &mut self,
        firstLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        secondLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        distance: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), bool, 3usize>("ComputeDistanceInInheritanceHierarchy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ComputeDistanceInInheritanceHierarchy",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (firstLayout, secondLayout, distance))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindLayoutThatIntroducesControl(
        &mut self,
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
        cache: crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Cache,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Utilities::InternedString>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
                        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Cache,
                    ), crate::UnityEngine::InputSystem::Utilities::InternedString, 2usize>(
                        "FindLayoutThatIntroducesControl",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FindLayoutThatIntroducesControl",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::InternedString =
            unsafe { cordl_method_info.invoke_unchecked(self, (control, cache))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetBaseLayoutName(
        &mut self,
        layoutName: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Utilities::InternedString>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::Utilities::InternedString),
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                        1usize,
                    >("GetBaseLayoutName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetBaseLayoutName", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::InternedString =
            unsafe { cordl_method_info.invoke_unchecked(self, (layoutName))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetBaseLayouts(
        &mut self,
        layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        includeSelf: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::InputSystem::Utilities::InternedString,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                        bool,
                    ), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                        >,
                    >, 2usize>("GetBaseLayouts")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetBaseLayouts",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::InputSystem::Utilities::InternedString,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (layout, includeSelf))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetControlTypeForLayout(
        &mut self,
        layoutName: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::Utilities::InternedString),
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        1usize,
                    >("GetControlTypeForLayout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetControlTypeForLayout", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> =
            unsafe { cordl_method_info.invoke_unchecked(self, (layoutName))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRootLayoutName(
        &mut self,
        layoutName: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Utilities::InternedString>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::Utilities::InternedString),
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                        1usize,
                    >("GetRootLayoutName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRootLayoutName", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::InternedString =
            unsafe { cordl_method_info.invoke_unchecked(self, (layoutName))? };
        Ok(__cordl_ret.into())
    }
    pub fn HasLayout(
        &mut self,
        name: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::Utilities::InternedString),
                        bool,
                        1usize,
                    >("HasLayout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HasLayout", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsBasedOn(
        &mut self,
        parentLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        childLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                    ), bool, 2usize>("IsBasedOn")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsBasedOn",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (parentLayout, childLayout))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsGeneratedLayout(
        &mut self,
        layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::Utilities::InternedString),
                        bool,
                        1usize,
                    >("IsGeneratedLayout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsGeneratedLayout", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (layout))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryFindLayoutForType(
        &mut self,
        layoutType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Utilities::InternedString>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                        1usize,
                    >("TryFindLayoutForType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryFindLayoutForType", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::InternedString =
            unsafe { cordl_method_info.invoke_unchecked(self, (layoutType))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryFindMatchingLayout(
        &mut self,
        deviceDescription: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Utilities::InternedString>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription),
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                        1usize,
                    >("TryFindMatchingLayout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryFindMatchingLayout", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::InternedString =
            unsafe { cordl_method_info.invoke_unchecked(self, (deviceDescription))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryLoadLayout(
        &mut self,
        name: crate::UnityEngine::InputSystem::Utilities::InternedString,
        table: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::UnityEngine::InputSystem::Utilities::InternedString,
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Layouts::InputControlLayout>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::Dictionary_2<
                                crate::UnityEngine::InputSystem::Utilities::InternedString,
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
                                >,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
                    >, 2usize>("TryLoadLayout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryLoadLayout",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (name, table))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryLoadLayoutInternal(
        &mut self,
        name: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Layouts::InputControlLayout>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::Utilities::InternedString),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
                        >,
                        1usize,
                    >("TryLoadLayoutInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryLoadLayoutInternal", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn ValueTypeIsAssignableFrom(
        &mut self,
        layoutName: crate::UnityEngine::InputSystem::Utilities::InternedString,
        valueType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                    ), bool, 2usize>("ValueTypeIsAssignableFrom")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ValueTypeIsAssignableFrom",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (layoutName, valueType))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItem")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct InputControlLayout_ControlItem {
    pub _name_k__BackingField: crate::UnityEngine::InputSystem::Utilities::InternedString,
    pub _layout_k__BackingField: crate::UnityEngine::InputSystem::Utilities::InternedString,
    pub _variants_k__BackingField: crate::UnityEngine::InputSystem::Utilities::InternedString,
    pub _useStateFrom_k__BackingField:
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _displayName_k__BackingField:
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _shortDisplayName_k__BackingField:
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _usages_k__BackingField: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    >,
    pub _aliases_k__BackingField: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    >,
    pub _parameters_k__BackingField: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
        crate::UnityEngine::InputSystem::Utilities::NamedValue,
    >,
    pub _processors_k__BackingField: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
        crate::UnityEngine::InputSystem::Utilities::NameAndParameters,
    >,
    pub _offset_k__BackingField: u32,
    pub _bit_k__BackingField: u32,
    pub _sizeInBits_k__BackingField: u32,
    pub _format_k__BackingField: crate::UnityEngine::InputSystem::Utilities::FourCC,
    pub _flags_k__BackingField:
        crate::UnityEngine::InputSystem::Layouts::ControlItem_InputControlLayout_Flags,
    pub _arraySize_k__BackingField: i32,
    pub _defaultState_k__BackingField: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    pub _minValue_k__BackingField: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    pub _maxValue_k__BackingField: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItem")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Layouts";
    const CLASS_NAME: &'static str = "InputControlLayout/ControlItem";
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItem")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItem")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItem")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItem")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItem")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItem")]
impl crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem {
    #[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItem+Flags")]
    pub type Flags = crate::UnityEngine::InputSystem::Layouts::ControlItem_InputControlLayout_Flags;
    pub fn Merge(
        &mut self,
        other: crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem),
                        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
                        1usize,
                    >("Merge")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Merge",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem =
            unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_aliases(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                    >, 0usize>("get_aliases")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_aliases",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_arraySize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_arraySize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_arraySize",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_bit(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), u32, 0usize>("get_bit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_bit",
                            0usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Utilities::PrimitiveValue>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
                        0usize,
                    >("get_defaultState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_defaultState", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_displayName(
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
                    >("get_displayName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_displayName", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_dontReset(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_dontReset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_dontReset",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_flags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::ControlItem_InputControlLayout_Flags,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputSystem::Layouts::ControlItem_InputControlLayout_Flags,
                        0usize,
                    >("get_flags")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_flags", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::ControlItem_InputControlLayout_Flags = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_format(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Utilities::FourCC> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::InputSystem::Utilities::FourCC, 0usize>(
                        "get_format",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_format",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isArray(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_isArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_isArray",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isFirstDefinedInThisLayout(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_isFirstDefinedInThisLayout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_isFirstDefinedInThisLayout",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isModifyingExistingControl(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_isModifyingExistingControl")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_isModifyingExistingControl",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isNoisy(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_isNoisy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_isNoisy",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isSynthetic(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_isSynthetic")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_isSynthetic",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_layout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Utilities::InternedString>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                        0usize,
                    >("get_layout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_layout", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::InternedString =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_maxValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Utilities::PrimitiveValue>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
                        0usize,
                    >("get_maxValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_maxValue", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_minValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Utilities::PrimitiveValue>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
                        0usize,
                    >("get_minValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_minValue", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Utilities::InternedString>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                        0usize,
                    >("get_name")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_name", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::InternedString =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_offset(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), u32, 0usize>("get_offset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_offset",
                            0usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_parameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::NamedValue,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                        crate::UnityEngine::InputSystem::Utilities::NamedValue,
                    >, 0usize>("get_parameters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_parameters",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::NamedValue,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_processors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::NameAndParameters,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                        crate::UnityEngine::InputSystem::Utilities::NameAndParameters,
                    >, 0usize>("get_processors")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_processors",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::NameAndParameters,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_shortDisplayName(
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
                    >("get_shortDisplayName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_shortDisplayName", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_sizeInBits(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), u32, 0usize>("get_sizeInBits")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_sizeInBits",
                            0usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_usages(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                    >, 0usize>("get_usages")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_usages",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_useStateFrom(
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
                    >("get_useStateFrom")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_useStateFrom", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_variants(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Utilities::InternedString>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                        0usize,
                    >("get_variants")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_variants", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::InternedString =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_aliases(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                    >), quest_hook::libil2cpp::Void, 1usize>("set_aliases")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_aliases",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_arraySize(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_arraySize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_arraySize",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_bit(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u32), quest_hook::libil2cpp::Void, 1usize>("set_bit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_bit",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultState(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::Utilities::PrimitiveValue),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_defaultState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_defaultState", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_displayName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_displayName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_displayName", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_dontReset(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("set_dontReset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_dontReset",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_flags(
        &mut self,
        value: crate::UnityEngine::InputSystem::Layouts::ControlItem_InputControlLayout_Flags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::Layouts::ControlItem_InputControlLayout_Flags),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_flags")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_flags", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_format(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::FourCC,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::Utilities::FourCC),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_format", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_isFirstDefinedInThisLayout(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "set_isFirstDefinedInThisLayout",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_isFirstDefinedInThisLayout",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_isModifyingExistingControl(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "set_isModifyingExistingControl",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_isModifyingExistingControl",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_isNoisy(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("set_isNoisy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_isNoisy",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_isSynthetic(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("set_isSynthetic")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_isSynthetic",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_layout(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::Utilities::InternedString),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_layout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_layout", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_maxValue(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::Utilities::PrimitiveValue),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_maxValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_maxValue", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_minValue(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::Utilities::PrimitiveValue),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_minValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_minValue", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_name(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::Utilities::InternedString),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_name")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_name", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_offset(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u32), quest_hook::libil2cpp::Void, 1usize>("set_offset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_offset",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_parameters(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::NamedValue,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                        crate::UnityEngine::InputSystem::Utilities::NamedValue,
                    >), quest_hook::libil2cpp::Void, 1usize>("set_parameters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_parameters",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_processors(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::NameAndParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                        crate::UnityEngine::InputSystem::Utilities::NameAndParameters,
                    >), quest_hook::libil2cpp::Void, 1usize>("set_processors")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_processors",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_shortDisplayName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_shortDisplayName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_shortDisplayName", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_sizeInBits(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u32), quest_hook::libil2cpp::Void, 1usize>("set_sizeInBits")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_sizeInBits",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_usages(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                    >), quest_hook::libil2cpp::Void, 1usize>("set_usages")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_usages",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_useStateFrom(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_useStateFrom")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_useStateFrom", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_variants(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::Utilities::InternedString),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_variants")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_variants", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItemJson")]
#[repr(C)]
#[derive(Debug)]
pub struct InputControlLayout_ControlItemJson {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub layout: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub variants: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub usage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub alias: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub useStateFrom: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub offset: u32,
    pub bit: u32,
    pub sizeInBits: u32,
    pub format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub arraySize: i32,
    pub usages: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub aliases: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub parameters: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub processors: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub shortDisplayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub noisy: bool,
    pub dontReset: bool,
    pub synthetic: bool,
    pub defaultState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub minValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub maxValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItemJson")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItemJson
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Layouts";
    const CLASS_NAME: &'static str = "InputControlLayout/ControlItemJson";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItemJson")]
impl std::ops::Deref
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItemJson
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItemJson")]
impl std::ops::DerefMut
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItemJson
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItemJson")]
impl crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItemJson {
    pub fn FromControlItems(
        items: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItemJson,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
                            >,
                        >),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItemJson,
                                >,
                            >,
                        >,
                        1usize,
                    >("FromControlItems")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromControlItems", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItemJson,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (items))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ToLayout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
                        0usize,
                    >("ToLayout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToLayout", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItemJson")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItemJson
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Flags")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum InputControlLayout_Flags {
    #[default]
    CanRunInBackground = 8i32,
    CanRunInBackgroundIsSet = 16i32,
    HideInUI = 2i32,
    IsGenericTypeOfDevice = 1i32,
    IsNoisy = 32i32,
    IsOverride = 4i32,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Flags")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Flags
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Layouts";
    const CLASS_NAME: &'static str = "InputControlLayout/Flags";
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Flags")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Flags
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Flags")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Flags
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Flags")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Flags
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+Flags")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Flags
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutJson")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct InputControlLayout_LayoutJson {
    pub name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub extend: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub extendMultiple: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub beforeRender: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub runInBackground: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub commonUsages: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub description: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _cordl_type: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub variant: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub isGenericTypeOfDevice: bool,
    pub hideInUI: bool,
    pub controls: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItemJson,
            >,
        >,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutJson")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJson
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Layouts";
    const CLASS_NAME: &'static str = "InputControlLayout/LayoutJson";
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutJson")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJson
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutJson")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJson
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutJson")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJson
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutJson")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJson
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutJson")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJson
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutJson")]
impl crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJson {
    pub fn FromLayout(
        layout: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJson,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
                        >),
                        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJson,
                        1usize,
                    >("FromLayout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromLayout", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJson =
            unsafe { cordl_method_info.invoke_unchecked((), (layout))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToLayout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Layouts::InputControlLayout>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
                    >, 0usize>("ToLayout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ToLayout",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutJsonNameAndDescriptorOnly"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct InputControlLayout_LayoutJsonNameAndDescriptorOnly {
    pub name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub extend: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub extendMultiple: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub device: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher_MatcherJson,
}
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutJsonNameAndDescriptorOnly"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJsonNameAndDescriptorOnly
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Layouts";
    const CLASS_NAME: &'static str = "InputControlLayout/LayoutJsonNameAndDescriptorOnly";
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
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutJsonNameAndDescriptorOnly"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJsonNameAndDescriptorOnly
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
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutJsonNameAndDescriptorOnly"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJsonNameAndDescriptorOnly
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
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutJsonNameAndDescriptorOnly"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJsonNameAndDescriptorOnly
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
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutJsonNameAndDescriptorOnly"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJsonNameAndDescriptorOnly
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
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutJsonNameAndDescriptorOnly"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJsonNameAndDescriptorOnly
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutJsonNameAndDescriptorOnly"
)]
impl crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJsonNameAndDescriptorOnly {}
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutNotFoundException"
)]
#[repr(C)]
#[derive(Debug)]
pub struct InputControlLayout_LayoutNotFoundException {
    __cordl_parent: crate::System::Exception,
    pub _layout_k__BackingField: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutNotFoundException"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutNotFoundException
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Layouts";
    const CLASS_NAME: &'static str = "InputControlLayout/LayoutNotFoundException";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutNotFoundException")]
impl std::ops::Deref
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutNotFoundException
{
    type Target = crate::System::Exception;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutNotFoundException")]
impl std::ops::DerefMut
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutNotFoundException
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutNotFoundException")]
impl crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutNotFoundException {
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString2(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_Exception3(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        innerException: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message, innerException))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_Il2CppString1(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, message))?;
        Ok(__cordl_object.into())
    }
    pub fn New_SerializationInfo_StreamingContext4(
        info: quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::SerializationInfo>,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, context))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString2(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_Exception3(
        &mut self,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        innerException: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::System::Exception>,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (message, innerException))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_Il2CppString1(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (name, message))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SerializationInfo_StreamingContext4(
        &mut self,
        info: quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::SerializationInfo>,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::SerializationInfo,
                        >,
                        crate::System::Runtime::Serialization::StreamingContext,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (info, context))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_layout(
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
                    >("get_layout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_layout", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutNotFoundException"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutNotFoundException
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
