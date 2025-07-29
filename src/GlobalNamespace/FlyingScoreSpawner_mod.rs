#[cfg(feature = "cordl_class_FlyingScoreSpawner")]
#[repr(C)]
#[derive(Debug)]
pub struct FlyingScoreSpawner {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _flyingScoreEffectPool: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FlyingScoreEffect_Pool,
    >,
    pub _initData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FlyingScoreSpawner_InitData,
    >,
}
#[cfg(feature = "cordl_class_FlyingScoreSpawner")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::FlyingScoreSpawner {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "FlyingScoreSpawner";
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
#[cfg(feature = "FlyingScoreSpawner")]
impl std::ops::Deref for crate::GlobalNamespace::FlyingScoreSpawner {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FlyingScoreSpawner")]
impl std::ops::DerefMut for crate::GlobalNamespace::FlyingScoreSpawner {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FlyingScoreSpawner")]
impl crate::GlobalNamespace::FlyingScoreSpawner {
    #[cfg(feature = "FlyingScoreSpawner+InitData")]
    pub type InitData = crate::GlobalNamespace::FlyingScoreSpawner_InitData;
    #[cfg(feature = "FlyingScoreSpawner+SpawnPosition")]
    pub type SpawnPosition = crate::GlobalNamespace::FlyingScoreSpawner_SpawnPosition;
    pub fn HandleFlyingObjectEffectDidFinish(
        &mut self,
        flyingObjectEffect: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FlyingObjectEffect,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::FlyingObjectEffect,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("HandleFlyingObjectEffectDidFinish")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleFlyingObjectEffectDidFinish", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (flyingObjectEffect))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SpawnFlyingScore(
        &mut self,
        cutScoreBuffer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyCutScoreBuffer,
        >,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IReadonlyCutScoreBuffer,
                            >,
                            crate::UnityEngine::Color,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SpawnFlyingScore")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SpawnFlyingScore", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (cutScoreBuffer, color))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SpawnFlyingScoreNextFrame(
        &mut self,
        cutScoreBuffer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyCutScoreBuffer,
        >,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IReadonlyCutScoreBuffer,
                            >,
                            crate::UnityEngine::Color,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SpawnFlyingScoreNextFrame")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SpawnFlyingScoreNextFrame", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (cutScoreBuffer, color))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SpawnFlyingScoreNextFrameCoroutine(
        &mut self,
        cutScoreBuffer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyCutScoreBuffer,
        >,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IReadonlyCutScoreBuffer,
                            >,
                            crate::UnityEngine::Color,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::IEnumerator,
                        >,
                        2usize,
                    >("SpawnFlyingScoreNextFrameCoroutine")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SpawnFlyingScoreNextFrameCoroutine", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (cutScoreBuffer, color))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_FlyingScoreSpawner")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::FlyingScoreSpawner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "FlyingScoreSpawner")]
impl AsRef<crate::GlobalNamespace::IFlyingObjectEffectDidFinishEvent>
for crate::GlobalNamespace::FlyingScoreSpawner {
    fn as_ref(&self) -> &crate::GlobalNamespace::IFlyingObjectEffectDidFinishEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "FlyingScoreSpawner")]
impl AsMut<crate::GlobalNamespace::IFlyingObjectEffectDidFinishEvent>
for crate::GlobalNamespace::FlyingScoreSpawner {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IFlyingObjectEffectDidFinishEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "cordl_class_FlyingScoreSpawner+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct FlyingScoreSpawner_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub spawnPosition: crate::GlobalNamespace::FlyingScoreSpawner_SpawnPosition,
}
#[cfg(feature = "cordl_class_FlyingScoreSpawner+InitData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::FlyingScoreSpawner_InitData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "FlyingScoreSpawner/InitData";
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
#[cfg(feature = "FlyingScoreSpawner+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::FlyingScoreSpawner_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FlyingScoreSpawner+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::FlyingScoreSpawner_InitData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FlyingScoreSpawner+InitData")]
impl crate::GlobalNamespace::FlyingScoreSpawner_InitData {
    pub fn New(
        spawnPosition: crate::GlobalNamespace::FlyingScoreSpawner_SpawnPosition,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (spawnPosition))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        spawnPosition: crate::GlobalNamespace::FlyingScoreSpawner_SpawnPosition,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::FlyingScoreSpawner_SpawnPosition),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (spawnPosition))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_FlyingScoreSpawner+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FlyingScoreSpawner_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_FlyingScoreSpawner+SpawnPosition")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FlyingScoreSpawner_SpawnPosition {
    #[default]
    AboveGround = 1i32,
    Underground = 0i32,
}
#[cfg(feature = "cordl_class_FlyingScoreSpawner+SpawnPosition")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::FlyingScoreSpawner_SpawnPosition {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "FlyingScoreSpawner/SpawnPosition";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_FlyingScoreSpawner+SpawnPosition")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::FlyingScoreSpawner_SpawnPosition {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_FlyingScoreSpawner+SpawnPosition")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::FlyingScoreSpawner_SpawnPosition {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_FlyingScoreSpawner+SpawnPosition")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::FlyingScoreSpawner_SpawnPosition {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_FlyingScoreSpawner+SpawnPosition")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::FlyingScoreSpawner_SpawnPosition {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
