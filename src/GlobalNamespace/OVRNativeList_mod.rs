#[cfg(feature = "cordl_class_OVRNativeList")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRNativeList {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRNativeList")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRNativeList {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRNativeList";
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
#[cfg(feature = "OVRNativeList")]
impl std::ops::Deref for crate::GlobalNamespace::OVRNativeList {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRNativeList")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRNativeList {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRNativeList")]
impl crate::GlobalNamespace::OVRNativeList {
    #[cfg(feature = "OVRNativeList+CapacityHelper")]
    pub type CapacityHelper = crate::GlobalNamespace::OVRNativeList_CapacityHelper;
    pub fn ToNativeList<T>(
        collection: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRNativeList_1<T>>
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
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<T>,
                        >,
                        crate::Unity::Collections::Allocator,
                    ), crate::GlobalNamespace::OVRNativeList_1<T>, 2usize>(
                        "ToNativeList"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ToNativeList",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRNativeList_1<T> =
            unsafe { cordl_method_info.invoke_unchecked((), (collection, allocator))? };
        Ok(__cordl_ret.into())
    }
    pub fn WithSuggestedCapacityFrom_ByRefMut1<T>(
        collection: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
        nonAllocatingEnumerable: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVREnumerable_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRNativeList_CapacityHelper>
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
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<T>,
                        >,
                        quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVREnumerable_1<T>>,
                    ), crate::GlobalNamespace::OVRNativeList_CapacityHelper, 2usize>(
                        "WithSuggestedCapacityFrom",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WithSuggestedCapacityFrom",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRNativeList_CapacityHelper = unsafe {
            cordl_method_info.invoke_unchecked((), (collection, nonAllocatingEnumerable))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithSuggestedCapacityFrom_IEnumerable_1_0<T>(
        collection: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRNativeList_CapacityHelper>
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
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<T>,
                    >), crate::GlobalNamespace::OVRNativeList_CapacityHelper, 1usize>(
                        "WithSuggestedCapacityFrom",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WithSuggestedCapacityFrom",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRNativeList_CapacityHelper =
            unsafe { cordl_method_info.invoke_unchecked((), (collection))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRNativeList")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRNativeList {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRNativeList+CapacityHelper")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct OVRNativeList_CapacityHelper {
    pub _count: crate::System::Nullable_1<i32>,
}
#[cfg(feature = "cordl_class_OVRNativeList+CapacityHelper")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRNativeList_CapacityHelper {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRNativeList/CapacityHelper";
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
#[cfg(feature = "cordl_class_OVRNativeList+CapacityHelper")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::OVRNativeList_CapacityHelper
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRNativeList+CapacityHelper")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::OVRNativeList_CapacityHelper
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
#[cfg(feature = "cordl_class_OVRNativeList+CapacityHelper")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::GlobalNamespace::OVRNativeList_CapacityHelper
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
#[cfg(feature = "cordl_class_OVRNativeList+CapacityHelper")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::OVRNativeList_CapacityHelper {
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
#[cfg(feature = "cordl_class_OVRNativeList+CapacityHelper")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::GlobalNamespace::OVRNativeList_CapacityHelper
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRNativeList+CapacityHelper")]
impl crate::GlobalNamespace::OVRNativeList_CapacityHelper {
    pub fn AllocateEmpty<T>(
        &mut self,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRNativeList_1<T>>
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
                    .find_method::<
                        (crate::Unity::Collections::Allocator),
                        crate::GlobalNamespace::OVRNativeList_1<T>,
                        1usize,
                    >("AllocateEmpty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AllocateEmpty", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRNativeList_1<T> =
            unsafe { cordl_method_info.invoke_unchecked(self, (allocator))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        count: crate::System::Nullable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Nullable_1<i32>),
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
            unsafe { cordl_method_info.invoke_unchecked(self, (count))? };
        Ok(__cordl_ret.into())
    }
}
