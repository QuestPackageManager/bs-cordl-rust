#[cfg(feature = "BTSCharacterSpawnEventValueParser")]
#[repr(C)]
#[derive(Debug)]
pub struct BTSCharacterSpawnEventValueParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BTSCharacterSpawnEventValueParser")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BTSCharacterSpawnEventValueParser {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BTSCharacterSpawnEventValueParser";
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
#[cfg(feature = "BTSCharacterSpawnEventValueParser")]
impl std::ops::Deref for crate::GlobalNamespace::BTSCharacterSpawnEventValueParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BTSCharacterSpawnEventValueParser")]
impl std::ops::DerefMut for crate::GlobalNamespace::BTSCharacterSpawnEventValueParser {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BTSCharacterSpawnEventValueParser")]
impl crate::GlobalNamespace::BTSCharacterSpawnEventValueParser {
    pub const kAlternativeMaterialMask: i32 = 65536i32;
    pub const kAlternativeMaterialOffset: i32 = 16i32;
    pub const kAnimationBitOffset: i32 = 8i32;
    pub const kAnimationMask: i32 = 65280i32;
    pub const kPrefabBitOffset: i32 = 0i32;
    pub const kPrefabMask: i32 = 255i32;
    pub fn GetAnimationId(value: i32) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), i32, 1usize>("GetAnimationId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAnimationId", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetIsAlternativeMaterial(value: i32) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32),
                        bool,
                        1usize,
                    >("GetIsAlternativeMaterial")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetIsAlternativeMaterial", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPrefabId(value: i32) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), i32, 1usize>("GetPrefabId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPrefabId", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MergeValuesIntoOneInt(
        prefabId: i32,
        animationId: i32,
        isAlternativeMaterial: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32, i32, bool),
                        i32,
                        3usize,
                    >("MergeValuesIntoOneInt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MergeValuesIntoOneInt", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked((), (prefabId, animationId, isAlternativeMaterial))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BTSCharacterSpawnEventValueParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BTSCharacterSpawnEventValueParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
