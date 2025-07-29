#[cfg(feature = "cordl_class_BeatmapIdentifierNetSerializableHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapIdentifierNetSerializableHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_BeatmapIdentifierNetSerializableHelper")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapIdentifierNetSerializableHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapIdentifierNetSerializableHelper";
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
#[cfg(feature = "BeatmapIdentifierNetSerializableHelper")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapIdentifierNetSerializableHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapIdentifierNetSerializableHelper")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BeatmapIdentifierNetSerializableHelper {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapIdentifierNetSerializableHelper")]
impl crate::GlobalNamespace::BeatmapIdentifierNetSerializableHelper {
    pub fn ToBeatmapKey(
        beatmapKeySerializable: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapKeyNetSerializable,
        >,
        beatmapCharacteristicCollection: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapKey> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapKeyNetSerializable,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapCharacteristicCollection,
                            >,
                        ),
                        crate::GlobalNamespace::BeatmapKey,
                        2usize,
                    >("ToBeatmapKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToBeatmapKey", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::BeatmapKey = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (beatmapKeySerializable, beatmapCharacteristicCollection),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToIdentifier(
        beatmapKey: crate::GlobalNamespace::BeatmapKey,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapKeyNetSerializable>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::BeatmapKey),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapKeyNetSerializable,
                        >,
                        1usize,
                    >("ToIdentifier")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToIdentifier", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapKeyNetSerializable,
        > = unsafe { cordl_method_info.invoke_unchecked((), (beatmapKey))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatmapIdentifierNetSerializableHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapIdentifierNetSerializableHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
