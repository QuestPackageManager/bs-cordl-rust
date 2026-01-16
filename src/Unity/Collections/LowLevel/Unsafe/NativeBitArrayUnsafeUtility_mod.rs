#[cfg(feature = "cordl_class_Unity+Collections+LowLevel+Unsafe+NativeBitArrayUnsafeUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeBitArrayUnsafeUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Unity+Collections+LowLevel+Unsafe+NativeBitArrayUnsafeUtility")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Unity::Collections::LowLevel::Unsafe::NativeBitArrayUnsafeUtility
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Collections.LowLevel.Unsafe";
    const CLASS_NAME: &'static str = "NativeBitArrayUnsafeUtility";
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
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+NativeBitArrayUnsafeUtility")]
impl std::ops::Deref for crate::Unity::Collections::LowLevel::Unsafe::NativeBitArrayUnsafeUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+NativeBitArrayUnsafeUtility")]
impl std::ops::DerefMut
    for crate::Unity::Collections::LowLevel::Unsafe::NativeBitArrayUnsafeUtility
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+NativeBitArrayUnsafeUtility")]
impl crate::Unity::Collections::LowLevel::Unsafe::NativeBitArrayUnsafeUtility {
    pub fn ConvertExistingDataToNativeBitArray(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        sizeInBytes: i32,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeBitArray> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        i32,
                        crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                    ), crate::Unity::Collections::NativeBitArray, 3usize>(
                        "ConvertExistingDataToNativeBitArray",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ConvertExistingDataToNativeBitArray",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeBitArray =
            unsafe { cordl_method_info.invoke_unchecked((), (ptr, sizeInBytes, allocator))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+LowLevel+Unsafe+NativeBitArrayUnsafeUtility")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Unity::Collections::LowLevel::Unsafe::NativeBitArrayUnsafeUtility
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
