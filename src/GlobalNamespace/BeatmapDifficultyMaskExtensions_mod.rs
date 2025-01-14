#[cfg(feature = "BeatmapDifficultyMaskExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDifficultyMaskExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapDifficultyMaskExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapDifficultyMaskExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapDifficultyMaskExtensions";
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
#[cfg(feature = "BeatmapDifficultyMaskExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDifficultyMaskExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDifficultyMaskExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapDifficultyMaskExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDifficultyMaskExtensions")]
impl crate::GlobalNamespace::BeatmapDifficultyMaskExtensions {
    pub fn Contains_BeatmapDifficulty0(
        mask: crate::GlobalNamespace::BeatmapDifficultyMask,
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::GlobalNamespace::BeatmapDifficultyMask,
                    crate::GlobalNamespace::BeatmapDifficulty,
                ),
                bool,
                2usize,
            >("Contains")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Contains", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (mask, difficulty))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Contains_BeatmapDifficultyMask1(
        mask: crate::GlobalNamespace::BeatmapDifficultyMask,
        other: crate::GlobalNamespace::BeatmapDifficultyMask,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::GlobalNamespace::BeatmapDifficultyMask,
                    crate::GlobalNamespace::BeatmapDifficultyMask,
                ),
                bool,
                2usize,
            >("Contains")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Contains", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (mask, other)) };
        Ok(__cordl_ret.into())
    }
    pub fn DifferenceFrom(
        mask: crate::GlobalNamespace::BeatmapDifficultyMask,
        other: crate::GlobalNamespace::BeatmapDifficultyMask,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::GlobalNamespace::BeatmapDifficultyMask,
                    crate::GlobalNamespace::BeatmapDifficultyMask,
                ),
                i32,
                2usize,
            >("DifferenceFrom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DifferenceFrom", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (mask, other)) };
        Ok(__cordl_ret.into())
    }
    pub fn FromMask(
        mask: crate::GlobalNamespace::BeatmapDifficultyMask,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapDifficulty> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::GlobalNamespace::BeatmapDifficultyMask),
                crate::GlobalNamespace::BeatmapDifficulty,
                1usize,
            >("FromMask")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromMask", 1usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::BeatmapDifficulty = unsafe {
            method.invoke_unchecked((), (mask))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromMaskMaybe(
        mask: crate::GlobalNamespace::BeatmapDifficultyMask,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::GlobalNamespace::BeatmapDifficulty>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::GlobalNamespace::BeatmapDifficultyMask),
                crate::System::Nullable_1<crate::GlobalNamespace::BeatmapDifficulty>,
                1usize,
            >("FromMaskMaybe")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromMaskMaybe", 1usize
                )
            });
        let __cordl_ret: crate::System::Nullable_1<
            crate::GlobalNamespace::BeatmapDifficulty,
        > = unsafe { method.invoke_unchecked((), (mask)) };
        Ok(__cordl_ret.into())
    }
    pub fn LocalizedKey(
        mask: crate::GlobalNamespace::BeatmapDifficultyMask,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::GlobalNamespace::BeatmapDifficultyMask),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("LocalizedKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LocalizedKey", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (mask)) };
        Ok(__cordl_ret.into())
    }
    pub fn ShortLocalizedKey(
        mask: crate::GlobalNamespace::BeatmapDifficultyMask,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::GlobalNamespace::BeatmapDifficultyMask),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ShortLocalizedKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ShortLocalizedKey", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (mask)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToHexString(
        mask: crate::GlobalNamespace::BeatmapDifficultyMask,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::GlobalNamespace::BeatmapDifficultyMask),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToHexString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToHexString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (mask)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToMask(
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapDifficultyMask> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::GlobalNamespace::BeatmapDifficulty),
                crate::GlobalNamespace::BeatmapDifficultyMask,
                1usize,
            >("ToMask")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToMask", 1usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::BeatmapDifficultyMask = unsafe {
            method.invoke_unchecked((), (difficulty))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDifficultyMaskExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapDifficultyMaskExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
