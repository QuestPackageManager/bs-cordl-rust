#[cfg(feature = "cordl_class_AYellowpaper+SerializedCollections+SerializedKeyValuePair_2")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct SerializedKeyValuePair_2<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> {
    pub Key: TKey,
    pub Value: TValue,
    __cordl_phantom_TKey: std::marker::PhantomData<TKey>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "cordl_class_AYellowpaper+SerializedCollections+SerializedKeyValuePair_2")]
unsafe impl<TKey: quest_hook::libil2cpp::Type, TValue: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Type
    for crate::AYellowpaper::SerializedCollections::SerializedKeyValuePair_2<TKey, TValue>
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "AYellowpaper.SerializedCollections";
    const CLASS_NAME: &'static str = "SerializedKeyValuePair`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find(
                "AYellowpaper.SerializedCollections",
                "SerializedKeyValuePair`2",
            )
            .unwrap()
            .make_generic::<(TKey, TValue)>()
            .unwrap()
            .unwrap()
        })
    }
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
#[cfg(feature = "cordl_class_AYellowpaper+SerializedCollections+SerializedKeyValuePair_2")]
unsafe impl<TKey: quest_hook::libil2cpp::Type, TValue: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Argument
    for crate::AYellowpaper::SerializedCollections::SerializedKeyValuePair_2<TKey, TValue>
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_AYellowpaper+SerializedCollections+SerializedKeyValuePair_2")]
unsafe impl<TKey: quest_hook::libil2cpp::Type, TValue: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Parameter
    for crate::AYellowpaper::SerializedCollections::SerializedKeyValuePair_2<TKey, TValue>
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
#[cfg(feature = "cordl_class_AYellowpaper+SerializedCollections+SerializedKeyValuePair_2")]
unsafe impl<TKey: quest_hook::libil2cpp::Type, TValue: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Returned
    for crate::AYellowpaper::SerializedCollections::SerializedKeyValuePair_2<TKey, TValue>
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
#[cfg(feature = "cordl_class_AYellowpaper+SerializedCollections+SerializedKeyValuePair_2")]
unsafe impl<TKey: quest_hook::libil2cpp::Type, TValue: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Return
    for crate::AYellowpaper::SerializedCollections::SerializedKeyValuePair_2<TKey, TValue>
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
#[cfg(feature = "cordl_class_AYellowpaper+SerializedCollections+SerializedKeyValuePair_2")]
unsafe impl<TKey: quest_hook::libil2cpp::Type, TValue: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::ThisArgument
    for crate::AYellowpaper::SerializedCollections::SerializedKeyValuePair_2<TKey, TValue>
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "AYellowpaper+SerializedCollections+SerializedKeyValuePair_2")]
impl<TKey: quest_hook::libil2cpp::Type, TValue: quest_hook::libil2cpp::Type>
    crate::AYellowpaper::SerializedCollections::SerializedKeyValuePair_2<TKey, TValue>
{
    pub fn _ctor(
        &mut self,
        key: TKey,
        value: TValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(TKey, TValue), quest_hook::libil2cpp::Void, 2usize>(".ctor")
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
            unsafe { cordl_method_info.invoke_unchecked(self, (key, value))? };
        Ok(__cordl_ret.into())
    }
}
