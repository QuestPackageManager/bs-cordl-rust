#[cfg(feature = "BeatmapDataLoaderVersion4+IndexFilterConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct IndexFilterConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapDataLoaderVersion4+IndexFilterConverter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion4::IndexFilterConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion4";
    const CLASS_NAME: &'static str = "IndexFilterConverter";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "BeatmapDataLoaderVersion4+IndexFilterConverter")]
impl std::ops::Deref for crate::BeatmapDataLoaderVersion4::IndexFilterConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+IndexFilterConverter")]
impl std::ops::DerefMut for crate::BeatmapDataLoaderVersion4::IndexFilterConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+IndexFilterConverter")]
impl crate::BeatmapDataLoaderVersion4::IndexFilterConverter {
    pub fn Convert(
        indexFilter: crate::BeatmapSaveDataVersion4::IndexFilter,
        groupSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IndexFilter>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion4::IndexFilterConverter as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::BeatmapSaveDataVersion4::IndexFilter, i32),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IndexFilter>,
                2usize,
            >("Convert")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatmapDataLoaderVersion4::IndexFilterConverter as
                    quest_hook::libil2cpp::Type > ::class(), "Convert", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IndexFilter,
        > = unsafe { method.invoke_unchecked((), (indexFilter, groupSize))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsIndexFilterValid(
        indexFilter: crate::BeatmapSaveDataVersion4::IndexFilter,
        groupSize: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion4::IndexFilterConverter as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::BeatmapSaveDataVersion4::IndexFilter, i32),
                bool,
                2usize,
            >("IsIndexFilterValid")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatmapDataLoaderVersion4::IndexFilterConverter as
                    quest_hook::libil2cpp::Type > ::class(), "IsIndexFilterValid", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (indexFilter, groupSize))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+IndexFilterConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion4::IndexFilterConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
