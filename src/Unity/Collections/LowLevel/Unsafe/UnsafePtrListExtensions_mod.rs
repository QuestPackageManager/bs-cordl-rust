#[cfg(feature = "cordl_class_Unity+Collections+LowLevel+Unsafe+UnsafePtrListExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct UnsafePtrListExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Unity+Collections+LowLevel+Unsafe+UnsafePtrListExtensions")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Unity::Collections::LowLevel::Unsafe::UnsafePtrListExtensions
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Collections.LowLevel.Unsafe";
    const CLASS_NAME: &'static str = "UnsafePtrListExtensions";
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
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+UnsafePtrListExtensions")]
impl std::ops::Deref for crate::Unity::Collections::LowLevel::Unsafe::UnsafePtrListExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+UnsafePtrListExtensions")]
impl std::ops::DerefMut for crate::Unity::Collections::LowLevel::Unsafe::UnsafePtrListExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+UnsafePtrListExtensions")]
impl crate::Unity::Collections::LowLevel::Unsafe::UnsafePtrListExtensions {
    pub fn ListData<T>(
        from: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::LowLevel::Unsafe::UnsafePtrList_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::LowLevel::Unsafe::UnsafeList_1<crate::System::IntPtr>,
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
                    .find_static_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::Unity::Collections::LowLevel::Unsafe::UnsafePtrList_1<T>,
                    >), quest_hook::libil2cpp::ByRefMut<
                        crate::Unity::Collections::LowLevel::Unsafe::UnsafeList_1<
                            crate::System::IntPtr,
                        >,
                    >, 1usize>("ListData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ListData",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::LowLevel::Unsafe::UnsafeList_1<crate::System::IntPtr>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (from))? };
        Ok(__cordl_ret.into())
    }
    pub fn ListDataRO<T>(
        from: crate::Unity::Collections::LowLevel::Unsafe::UnsafePtrList_1<T>,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::LowLevel::Unsafe::UnsafeList_1<crate::System::IntPtr>,
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
                    .find_static_method::<
                        (crate::Unity::Collections::LowLevel::Unsafe::UnsafePtrList_1<
                            T,
                        >),
                        crate::Unity::Collections::LowLevel::Unsafe::UnsafeList_1<
                            crate::System::IntPtr,
                        >,
                        1usize,
                    >("ListDataRO")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ListDataRO", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::LowLevel::Unsafe::UnsafeList_1<
            crate::System::IntPtr,
        > = unsafe { cordl_method_info.invoke_unchecked((), (from))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+LowLevel+Unsafe+UnsafePtrListExtensions")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Unity::Collections::LowLevel::Unsafe::UnsafePtrListExtensions
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
