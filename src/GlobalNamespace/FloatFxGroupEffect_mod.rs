#[cfg(feature = "FloatFxGroupEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct FloatFxGroupEffect {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _tweeningManager: quest_hook::libil2cpp::Gc<
        crate::Tweening::SongTimeTweeningManager,
    >,
    pub _beatmapCallbacksController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCallbacksController,
    >,
    pub _floatTween: quest_hook::libil2cpp::Gc<crate::Tweening::FloatTween>,
    pub _floatFxBeatmapEventCallbackWrapper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapDataCallbackWrapper,
    >,
    pub _initData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FloatFxGroupEffect_InitData,
    >,
}
#[cfg(feature = "FloatFxGroupEffect")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::FloatFxGroupEffect {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "FloatFxGroupEffect";
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
#[cfg(feature = "FloatFxGroupEffect")]
impl std::ops::Deref for crate::GlobalNamespace::FloatFxGroupEffect {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FloatFxGroupEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::FloatFxGroupEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FloatFxGroupEffect")]
impl crate::GlobalNamespace::FloatFxGroupEffect {
    #[cfg(feature = "FloatFxGroupEffect+InitData")]
    pub type InitData = crate::GlobalNamespace::FloatFxGroupEffect_InitData;
    pub fn Cleanup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Cleanup")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Cleanup", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleFloatFxBeatmapEventCallback(
        &mut self,
        currentEvent: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FloatFxBeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
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
                    self, "HandleFloatFxBeatmapEventCallback", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (currentEvent))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        initData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FloatFxGroupEffect_InitData,
        >,
        tweeningManager: quest_hook::libil2cpp::Gc<
            crate::Tweening::SongTimeTweeningManager,
        >,
        beatmapCallbacksController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCallbacksController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (initData, tweeningManager, beatmapCallbacksController),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn SetValue(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32), quest_hook::libil2cpp::Void, 1usize>("SetValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetValue", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        initData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FloatFxGroupEffect_InitData,
        >,
        tweeningManager: quest_hook::libil2cpp::Gc<
            crate::Tweening::SongTimeTweeningManager,
        >,
        beatmapCallbacksController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCallbacksController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::FloatFxGroupEffect_InitData,
                    >,
                    quest_hook::libil2cpp::Gc<crate::Tweening::SongTimeTweeningManager>,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::BeatmapCallbacksController,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (initData, tweeningManager, beatmapCallbacksController),
                )
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "FloatFxGroupEffect")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::FloatFxGroupEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "FloatFxGroupEffect+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct FloatFxGroupEffect_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub groupId: i32,
    pub elementId: i32,
    pub target: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FloatFxGroupEffectTarget,
    >,
}
#[cfg(feature = "FloatFxGroupEffect+InitData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::FloatFxGroupEffect_InitData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "FloatFxGroupEffect/InitData";
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
#[cfg(feature = "FloatFxGroupEffect+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::FloatFxGroupEffect_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FloatFxGroupEffect+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::FloatFxGroupEffect_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FloatFxGroupEffect+InitData")]
impl crate::GlobalNamespace::FloatFxGroupEffect_InitData {
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
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
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (groupId, elementId, target))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "FloatFxGroupEffect+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FloatFxGroupEffect_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
