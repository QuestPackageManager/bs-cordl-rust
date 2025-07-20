#[cfg(feature = "BeatSaber+BeatAvatarSDK+RandomizeAvatarColorMap")]
#[repr(C)]
#[derive(Debug)]
pub struct RandomizeAvatarColorMap {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _totalIndices_k__BackingField: i32,
    pub _colorIndices_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+RandomizeAvatarColorMap")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::BeatAvatarSDK::RandomizeAvatarColorMap {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.BeatAvatarSDK";
    const CLASS_NAME: &'static str = "RandomizeAvatarColorMap";
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
#[cfg(feature = "BeatSaber+BeatAvatarSDK+RandomizeAvatarColorMap")]
impl std::ops::Deref for crate::BeatSaber::BeatAvatarSDK::RandomizeAvatarColorMap {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+RandomizeAvatarColorMap")]
impl std::ops::DerefMut for crate::BeatSaber::BeatAvatarSDK::RandomizeAvatarColorMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+RandomizeAvatarColorMap")]
impl crate::BeatSaber::BeatAvatarSDK::RandomizeAvatarColorMap {
    pub fn New(
        headTopPrimaryColorIndex: i32,
        headTopSecondaryColorIndex: i32,
        glassesColorIndex: i32,
        facialHairColorIndex: i32,
        handsColorIndex: i32,
        clothesPrimaryColorIndex: i32,
        clothesSecondaryColorIndex: i32,
        clothesDetailColorIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    headTopPrimaryColorIndex,
                    headTopSecondaryColorIndex,
                    glassesColorIndex,
                    facialHairColorIndex,
                    handsColorIndex,
                    clothesPrimaryColorIndex,
                    clothesSecondaryColorIndex,
                    clothesDetailColorIndex,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        headTopPrimaryColorIndex: i32,
        headTopSecondaryColorIndex: i32,
        glassesColorIndex: i32,
        facialHairColorIndex: i32,
        handsColorIndex: i32,
        clothesPrimaryColorIndex: i32,
        clothesSecondaryColorIndex: i32,
        clothesDetailColorIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (i32, i32, i32, i32, i32, i32, i32, i32),
                        quest_hook::libil2cpp::Void,
                        8usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        headTopPrimaryColorIndex,
                        headTopSecondaryColorIndex,
                        glassesColorIndex,
                        facialHairColorIndex,
                        handsColorIndex,
                        clothesPrimaryColorIndex,
                        clothesSecondaryColorIndex,
                        clothesDetailColorIndex,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_colorIndices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<i32>,
                        >,
                        0usize,
                    >("get_colorIndices")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_colorIndices", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_totalIndices(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), i32, 0usize>("get_totalIndices")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_totalIndices", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+RandomizeAvatarColorMap")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::BeatAvatarSDK::RandomizeAvatarColorMap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
