#[cfg(feature = "cordl_class_TMPro+TMP_DynamicFontAssetUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_DynamicFontAssetUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub s_SystemFontLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            u64,
            crate::TMPro::TMP_DynamicFontAssetUtilities_FontReference,
        >,
    >,
    pub s_SystemFontPaths: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub s_RegularStyleNameHashCode: u32,
}
#[cfg(feature = "cordl_class_TMPro+TMP_DynamicFontAssetUtilities")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::TMP_DynamicFontAssetUtilities {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "TMP_DynamicFontAssetUtilities";
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
#[cfg(feature = "TMPro+TMP_DynamicFontAssetUtilities")]
impl std::ops::Deref for crate::TMPro::TMP_DynamicFontAssetUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_DynamicFontAssetUtilities")]
impl std::ops::DerefMut for crate::TMPro::TMP_DynamicFontAssetUtilities {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_DynamicFontAssetUtilities")]
impl crate::TMPro::TMP_DynamicFontAssetUtilities {
    #[cfg(feature = "TMPro+TMP_DynamicFontAssetUtilities+FontReference")]
    pub type FontReference = crate::TMPro::TMP_DynamicFontAssetUtilities_FontReference;
    pub fn InitializeSystemFontReferenceCache(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "InitializeSystemFontReferenceCache",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InitializeSystemFontReferenceCache",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn TryGetSystemFontReferenceInternal(
        &mut self,
        familyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        styleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fontRef: quest_hook::libil2cpp::ByRefMut<
            crate::TMPro::TMP_DynamicFontAssetUtilities_FontReference,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::TMPro::TMP_DynamicFontAssetUtilities_FontReference,
                        >,
                    ), bool, 3usize>("TryGetSystemFontReferenceInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryGetSystemFontReferenceInternal",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (familyName, styleName, fontRef))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetSystemFontReference_ByRefMut0(
        familyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fontRef: quest_hook::libil2cpp::ByRefMut<
            crate::TMPro::TMP_DynamicFontAssetUtilities_FontReference,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::TMPro::TMP_DynamicFontAssetUtilities_FontReference,
                        >,
                    ), bool, 2usize>("TryGetSystemFontReference")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryGetSystemFontReference",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (familyName, fontRef))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetSystemFontReference_Il2CppString_ByRefMut1(
        familyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        styleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fontRef: quest_hook::libil2cpp::ByRefMut<
            crate::TMPro::TMP_DynamicFontAssetUtilities_FontReference,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::TMPro::TMP_DynamicFontAssetUtilities_FontReference,
                        >,
                    ), bool, 3usize>("TryGetSystemFontReference")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryGetSystemFontReference",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (familyName, styleName, fontRef))? };
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
#[cfg(feature = "cordl_class_TMPro+TMP_DynamicFontAssetUtilities")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_DynamicFontAssetUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_TMPro+TMP_DynamicFontAssetUtilities+FontReference")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct TMP_DynamicFontAssetUtilities_FontReference {
    pub familyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub styleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub faceIndex: i32,
    pub filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub hashCode: u64,
}
#[cfg(feature = "cordl_class_TMPro+TMP_DynamicFontAssetUtilities+FontReference")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::TMPro::TMP_DynamicFontAssetUtilities_FontReference
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "TMP_DynamicFontAssetUtilities/FontReference";
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
#[cfg(feature = "cordl_class_TMPro+TMP_DynamicFontAssetUtilities+FontReference")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::TMPro::TMP_DynamicFontAssetUtilities_FontReference
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_TMPro+TMP_DynamicFontAssetUtilities+FontReference")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::TMPro::TMP_DynamicFontAssetUtilities_FontReference
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
#[cfg(feature = "cordl_class_TMPro+TMP_DynamicFontAssetUtilities+FontReference")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::TMPro::TMP_DynamicFontAssetUtilities_FontReference
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
#[cfg(feature = "cordl_class_TMPro+TMP_DynamicFontAssetUtilities+FontReference")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::TMPro::TMP_DynamicFontAssetUtilities_FontReference
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
#[cfg(feature = "cordl_class_TMPro+TMP_DynamicFontAssetUtilities+FontReference")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::TMPro::TMP_DynamicFontAssetUtilities_FontReference
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+TMP_DynamicFontAssetUtilities+FontReference")]
impl crate::TMPro::TMP_DynamicFontAssetUtilities_FontReference {
    pub fn _ctor(
        &mut self,
        fontFilePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        faceNameAndStyle: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (fontFilePath, faceNameAndStyle, index))?
        };
        Ok(__cordl_ret.into())
    }
}
