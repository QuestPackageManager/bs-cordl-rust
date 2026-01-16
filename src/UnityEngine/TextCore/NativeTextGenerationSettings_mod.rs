#[cfg(feature = "cordl_class_UnityEngine+TextCore+NativeTextGenerationSettings")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct NativeTextGenerationSettings {
    pub fontAsset: crate::System::IntPtr,
    pub globalFontAssetFallbacks:
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<crate::System::IntPtr>>,
    pub text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub screenWidth: i32,
    pub screenHeight: i32,
    pub wordWrap: crate::UnityEngine::TextCore::WhiteSpace,
    pub overflow: crate::UnityEngine::TextCore::TextOverflow,
    pub languageDirection: crate::UnityEngine::TextCore::LanguageDirection,
    pub vertexPadding: i32,
    pub horizontalAlignment: crate::UnityEngine::TextCore::HorizontalAlignment,
    pub verticalAlignment: crate::UnityEngine::TextCore::VerticalAlignment,
    pub fontSize: i32,
    pub fontStyle: crate::UnityEngine::TextCore::Text::FontStyles,
    pub fontWeight: crate::UnityEngine::TextCore::Text::TextFontWeight,
    pub textSpans: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::TextCore::TextSpan>,
    >,
    pub color: crate::UnityEngine::Color32,
    pub characterSpacing: i32,
    pub wordSpacing: i32,
    pub paragraphSpacing: i32,
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+NativeTextGenerationSettings")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::TextCore::NativeTextGenerationSettings
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore";
    const CLASS_NAME: &'static str = "NativeTextGenerationSettings";
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+NativeTextGenerationSettings")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::TextCore::NativeTextGenerationSettings
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+NativeTextGenerationSettings")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::TextCore::NativeTextGenerationSettings
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+NativeTextGenerationSettings")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::TextCore::NativeTextGenerationSettings
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+NativeTextGenerationSettings")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::TextCore::NativeTextGenerationSettings
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+NativeTextGenerationSettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::TextCore::NativeTextGenerationSettings
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+NativeTextGenerationSettings")]
impl crate::UnityEngine::TextCore::NativeTextGenerationSettings {
    pub fn CreateTextSpan(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextCore::TextSpan> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::TextCore::TextSpan, 0usize>(
                        "CreateTextSpan",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateTextSpan",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TextCore::TextSpan =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
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
    pub fn get_Default(
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextCore::NativeTextGenerationSettings>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::UnityEngine::TextCore::NativeTextGenerationSettings,
                        0usize,
                    >("get_Default")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Default", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TextCore::NativeTextGenerationSettings =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_hasLink(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_hasLink")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_hasLink",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
