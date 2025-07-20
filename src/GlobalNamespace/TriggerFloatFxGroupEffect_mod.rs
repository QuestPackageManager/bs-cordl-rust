#[cfg(feature = "TriggerFloatFxGroupEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct TriggerFloatFxGroupEffect {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _beatmapCallbacksController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCallbacksController,
    >,
    pub _floatFxBeatmapEventCallbackWrapper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapDataCallbackWrapper,
    >,
    pub _initData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::TriggerFloatFxGroupEffect_InitData,
    >,
}
#[cfg(feature = "TriggerFloatFxGroupEffect")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::TriggerFloatFxGroupEffect {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TriggerFloatFxGroupEffect";
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
#[cfg(feature = "TriggerFloatFxGroupEffect")]
impl std::ops::Deref for crate::GlobalNamespace::TriggerFloatFxGroupEffect {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TriggerFloatFxGroupEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::TriggerFloatFxGroupEffect {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TriggerFloatFxGroupEffect")]
impl crate::GlobalNamespace::TriggerFloatFxGroupEffect {
    #[cfg(feature = "TriggerFloatFxGroupEffect+InitData")]
    pub type InitData = crate::GlobalNamespace::TriggerFloatFxGroupEffect_InitData;
    pub fn Cleanup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Cleanup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Cleanup", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleFloatFxBeatmapEventCallback(
        &mut self,
        currentEvent: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FloatFxBeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::FloatFxBeatmapEventData,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("HandleFloatFxBeatmapEventCallback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "HandleFloatFxBeatmapEventCallback", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (currentEvent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        initData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::TriggerFloatFxGroupEffect_InitData,
        >,
        beatmapCallbacksController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCallbacksController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initData, beatmapCallbacksController))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        initData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::TriggerFloatFxGroupEffect_InitData,
        >,
        beatmapCallbacksController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCallbacksController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::TriggerFloatFxGroupEffect_InitData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapCallbacksController,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (initData, beatmapCallbacksController))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TriggerFloatFxGroupEffect")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TriggerFloatFxGroupEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TriggerFloatFxGroupEffect+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct TriggerFloatFxGroupEffect_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub groupId: i32,
    pub elementId: i32,
    pub target: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FloatFxGroupEffectTarget,
    >,
}
#[cfg(feature = "TriggerFloatFxGroupEffect+InitData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::TriggerFloatFxGroupEffect_InitData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TriggerFloatFxGroupEffect/InitData";
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
#[cfg(feature = "TriggerFloatFxGroupEffect+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::TriggerFloatFxGroupEffect_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TriggerFloatFxGroupEffect+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::TriggerFloatFxGroupEffect_InitData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TriggerFloatFxGroupEffect+InitData")]
impl crate::GlobalNamespace::TriggerFloatFxGroupEffect_InitData {
    pub fn New(
        groupId: i32,
        elementId: i32,
        target: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FloatFxGroupEffectTarget,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (groupId, elementId, target))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        groupId: i32,
        elementId: i32,
        target: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FloatFxGroupEffectTarget,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::FloatFxGroupEffectTarget,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (groupId, elementId, target))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TriggerFloatFxGroupEffect+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TriggerFloatFxGroupEffect_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
