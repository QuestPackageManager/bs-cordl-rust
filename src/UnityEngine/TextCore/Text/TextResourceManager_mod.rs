#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+TextResourceManager")]
#[repr(C)]
#[derive(Debug)]
pub struct TextResourceManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+TextResourceManager")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::TextCore::Text::TextResourceManager
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.Text";
    const CLASS_NAME: &'static str = "TextResourceManager";
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
#[cfg(feature = "UnityEngine+TextCore+Text+TextResourceManager")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::TextResourceManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextResourceManager")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::Text::TextResourceManager {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextResourceManager")]
impl crate::UnityEngine::TextCore::Text::TextResourceManager {
    #[cfg(feature = "UnityEngine+TextCore+Text+TextResourceManager+FontAssetRef")]
    pub type FontAssetRef = crate::UnityEngine::TextCore::Text::TextResourceManager_FontAssetRef;
    pub fn AddFontAsset(
        fontAsset: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::FontAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::TextCore::Text::FontAsset,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("AddFontAsset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddFontAsset", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (fontAsset))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+TextResourceManager")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::TextCore::Text::TextResourceManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+TextResourceManager+FontAssetRef")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct TextResourceManager_FontAssetRef {
    pub nameHashCode: i32,
    pub familyNameHashCode: i32,
    pub styleNameHashCode: i32,
    pub familyNameAndStyleHashCode: i64,
    pub fontAsset: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::FontAsset>,
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+TextResourceManager+FontAssetRef")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::TextCore::Text::TextResourceManager_FontAssetRef
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.Text";
    const CLASS_NAME: &'static str = "TextResourceManager/FontAssetRef";
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+TextResourceManager+FontAssetRef")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::TextCore::Text::TextResourceManager_FontAssetRef
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+TextResourceManager+FontAssetRef")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::TextCore::Text::TextResourceManager_FontAssetRef
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+TextResourceManager+FontAssetRef")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::TextCore::Text::TextResourceManager_FontAssetRef
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+TextResourceManager+FontAssetRef")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::TextCore::Text::TextResourceManager_FontAssetRef
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+TextResourceManager+FontAssetRef")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::TextCore::Text::TextResourceManager_FontAssetRef
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextResourceManager+FontAssetRef")]
impl crate::UnityEngine::TextCore::Text::TextResourceManager_FontAssetRef {
    pub fn _ctor(
        &mut self,
        nameHashCode: i32,
        familyNameHashCode: i32,
        styleNameHashCode: i32,
        fontAsset: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::FontAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::FontAsset>,
                    ), quest_hook::libil2cpp::Void, 4usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    nameHashCode,
                    familyNameHashCode,
                    styleNameHashCode,
                    fontAsset,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
}
