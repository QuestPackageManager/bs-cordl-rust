#[cfg(feature = "cordl_class_BeatSaber+Destinations+SimpleBeatmapKey")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct SimpleBeatmapKey {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub characteristic: crate::BeatSaber::Destinations::BeatmapCharacteristic,
    pub difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    pub levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "cordl_class_BeatSaber+Destinations+SimpleBeatmapKey")]
unsafe impl quest_hook::libil2cpp::Type for crate::BeatSaber::Destinations::SimpleBeatmapKey {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Destinations";
    const CLASS_NAME: &'static str = "SimpleBeatmapKey";
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
#[cfg(feature = "BeatSaber+Destinations+SimpleBeatmapKey")]
impl std::ops::Deref for crate::BeatSaber::Destinations::SimpleBeatmapKey {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Destinations+SimpleBeatmapKey")]
impl std::ops::DerefMut for crate::BeatSaber::Destinations::SimpleBeatmapKey {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Destinations+SimpleBeatmapKey")]
impl crate::BeatSaber::Destinations::SimpleBeatmapKey {
    pub fn New(
        characteristic: crate::BeatSaber::Destinations::BeatmapCharacteristic,
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (characteristic, difficulty, levelId))?;
        Ok(__cordl_object.into())
    }
    pub fn ToStruct(
        &mut self,
        characteristicCollection: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapKey> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::BeatmapCharacteristicCollection,
                    >), crate::GlobalNamespace::BeatmapKey, 1usize>("ToStruct")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ToStruct",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::BeatmapKey =
            unsafe { cordl_method_info.invoke_unchecked(self, (characteristicCollection))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        characteristic: crate::BeatSaber::Destinations::BeatmapCharacteristic,
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::BeatSaber::Destinations::BeatmapCharacteristic,
                        crate::GlobalNamespace::BeatmapDifficulty,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Void, 3usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (characteristic, difficulty, levelId))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Destinations+SimpleBeatmapKey")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::Destinations::SimpleBeatmapKey {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
