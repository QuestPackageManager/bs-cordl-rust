#[cfg(feature = "Unity+Collections+NativeSliceExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeSliceExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Collections+NativeSliceExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Collections::NativeSliceExtensions =>
    "Unity.Collections"."NativeSliceExtensions"
);
#[cfg(feature = "Unity+Collections+NativeSliceExtensions")]
impl std::ops::Deref for crate::Unity::Collections::NativeSliceExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+NativeSliceExtensions")]
impl std::ops::DerefMut for crate::Unity::Collections::NativeSliceExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+NativeSliceExtensions")]
impl crate::Unity::Collections::NativeSliceExtensions {}
#[cfg(feature = "Unity+Collections+NativeSliceExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Collections::NativeSliceExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
