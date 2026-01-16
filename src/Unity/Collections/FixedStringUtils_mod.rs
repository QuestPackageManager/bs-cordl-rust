#[cfg(feature = "cordl_class_Unity+Collections+FixedStringUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct FixedStringUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Unity+Collections+FixedStringUtils")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Collections::FixedStringUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "FixedStringUtils";
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
#[cfg(feature = "Unity+Collections+FixedStringUtils")]
impl std::ops::Deref for crate::Unity::Collections::FixedStringUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+FixedStringUtils")]
impl std::ops::DerefMut for crate::Unity::Collections::FixedStringUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+FixedStringUtils")]
impl crate::Unity::Collections::FixedStringUtils {
    #[cfg(feature = "Unity+Collections+FixedStringUtils+UintFloatUnion")]
    pub type UintFloatUnion = crate::Unity::Collections::FixedStringUtils_UintFloatUnion;
    pub fn Base10ToBase2(
        output: quest_hook::libil2cpp::ByRefMut<f32>,
        mantissa10: u64,
        exponent10: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::ParseError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<f32>, u64, i32),
                        crate::Unity::Collections::ParseError,
                        3usize,
                    >("Base10ToBase2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Base10ToBase2", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::ParseError =
            unsafe { cordl_method_info.invoke_unchecked((), (output, mantissa10, exponent10))? };
        Ok(__cordl_ret.into())
    }
    pub fn Base2ToBase10(
        mantissa10: quest_hook::libil2cpp::ByRefMut<u64>,
        exponent10: quest_hook::libil2cpp::ByRefMut<i32>,
        input: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<u64>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                        f32,
                    ), quest_hook::libil2cpp::Void, 3usize>("Base2ToBase10")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Base2ToBase10",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (mantissa10, exponent10, input))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+FixedStringUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Collections::FixedStringUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+FixedStringUtils+UintFloatUnion")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct FixedStringUtils_UintFloatUnion {
    padding: quest_hook::libil2cpp::ValueTypePadding<4usize>,
}
#[cfg(feature = "cordl_class_Unity+Collections+FixedStringUtils+UintFloatUnion")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Unity::Collections::FixedStringUtils_UintFloatUnion
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "FixedStringUtils/UintFloatUnion";
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
#[cfg(feature = "cordl_class_Unity+Collections+FixedStringUtils+UintFloatUnion")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::Unity::Collections::FixedStringUtils_UintFloatUnion
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+FixedStringUtils+UintFloatUnion")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::Unity::Collections::FixedStringUtils_UintFloatUnion
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
#[cfg(feature = "cordl_class_Unity+Collections+FixedStringUtils+UintFloatUnion")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::Unity::Collections::FixedStringUtils_UintFloatUnion
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
#[cfg(feature = "cordl_class_Unity+Collections+FixedStringUtils+UintFloatUnion")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::Unity::Collections::FixedStringUtils_UintFloatUnion
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
#[cfg(feature = "cordl_class_Unity+Collections+FixedStringUtils+UintFloatUnion")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::Unity::Collections::FixedStringUtils_UintFloatUnion
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Collections+FixedStringUtils+UintFloatUnion")]
impl crate::Unity::Collections::FixedStringUtils_UintFloatUnion {}
