#[cfg(feature = "cordl_class_Unity+Collections+LowLevel+Unsafe+UnsafeTextExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct UnsafeTextExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Unity+Collections+LowLevel+Unsafe+UnsafeTextExtensions")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Unity::Collections::LowLevel::Unsafe::UnsafeTextExtensions
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Collections.LowLevel.Unsafe";
    const CLASS_NAME: &'static str = "UnsafeTextExtensions";
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
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+UnsafeTextExtensions")]
impl std::ops::Deref for crate::Unity::Collections::LowLevel::Unsafe::UnsafeTextExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+UnsafeTextExtensions")]
impl std::ops::DerefMut for crate::Unity::Collections::LowLevel::Unsafe::UnsafeTextExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+UnsafeTextExtensions")]
impl crate::Unity::Collections::LowLevel::Unsafe::UnsafeTextExtensions {
    pub fn AsUnsafeListOfBytes(
        text: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::LowLevel::Unsafe::UnsafeList_1<u8>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
                    >), quest_hook::libil2cpp::ByRefMut<
                        crate::Unity::Collections::LowLevel::Unsafe::UnsafeList_1<u8>,
                    >, 1usize>("AsUnsafeListOfBytes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AsUnsafeListOfBytes",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::LowLevel::Unsafe::UnsafeList_1<u8>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (text))? };
        Ok(__cordl_ret.into())
    }
    pub fn AsUnsafeListOfBytesRO(
        text: crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::LowLevel::Unsafe::UnsafeList_1<u8>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::LowLevel::Unsafe::UnsafeText),
                        crate::Unity::Collections::LowLevel::Unsafe::UnsafeList_1<u8>,
                        1usize,
                    >("AsUnsafeListOfBytesRO")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AsUnsafeListOfBytesRO", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::LowLevel::Unsafe::UnsafeList_1<u8> =
            unsafe { cordl_method_info.invoke_unchecked((), (text))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+LowLevel+Unsafe+UnsafeTextExtensions")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Unity::Collections::LowLevel::Unsafe::UnsafeTextExtensions
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
