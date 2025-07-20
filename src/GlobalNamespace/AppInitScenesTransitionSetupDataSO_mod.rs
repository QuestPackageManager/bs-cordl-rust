#[cfg(feature = "AppInitScenesTransitionSetupDataSO")]
#[repr(C)]
#[derive(Debug)]
pub struct AppInitScenesTransitionSetupDataSO {
    __cordl_parent: crate::GlobalNamespace::SingleFixedSceneScenesTransitionSetupDataSO,
}
#[cfg(feature = "AppInitScenesTransitionSetupDataSO")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "AppInitScenesTransitionSetupDataSO";
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
#[cfg(feature = "AppInitScenesTransitionSetupDataSO")]
impl std::ops::Deref for crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO {
    type Target = crate::GlobalNamespace::SingleFixedSceneScenesTransitionSetupDataSO;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AppInitScenesTransitionSetupDataSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AppInitScenesTransitionSetupDataSO")]
impl crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO {
    #[cfg(feature = "AppInitScenesTransitionSetupDataSO+AppInitOverrideStartType")]
    pub type AppInitOverrideStartType = crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitOverrideStartType;
    #[cfg(feature = "AppInitScenesTransitionSetupDataSO+AppInitSceneSetupData")]
    pub type AppInitSceneSetupData = crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitSceneSetupData;
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Init")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Init", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitAsAppStart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("InitAsAppStart")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InitAsAppStart", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
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
    pub fn __Init(
        &mut self,
        appInitOverrideStartType: crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitOverrideStartType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitOverrideStartType),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("__Init")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "__Init", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (appInitOverrideStartType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AppInitScenesTransitionSetupDataSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "AppInitScenesTransitionSetupDataSO+AppInitOverrideStartType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AppInitScenesTransitionSetupDataSO_AppInitOverrideStartType {
    #[default]
    AppRestart = 2i32,
    AppStart = 1i32,
    DoNotOverride = 0i32,
    MultiSceneEditor = 3i32,
}
#[cfg(feature = "AppInitScenesTransitionSetupDataSO+AppInitOverrideStartType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitOverrideStartType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "AppInitScenesTransitionSetupDataSO/AppInitOverrideStartType";
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
#[cfg(feature = "AppInitScenesTransitionSetupDataSO+AppInitOverrideStartType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitOverrideStartType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "AppInitScenesTransitionSetupDataSO+AppInitOverrideStartType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitOverrideStartType {
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
#[cfg(feature = "AppInitScenesTransitionSetupDataSO+AppInitOverrideStartType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitOverrideStartType {
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
#[cfg(feature = "AppInitScenesTransitionSetupDataSO+AppInitOverrideStartType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitOverrideStartType {
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
#[cfg(feature = "AppInitScenesTransitionSetupDataSO+AppInitSceneSetupData")]
#[repr(C)]
#[derive(Debug)]
pub struct AppInitScenesTransitionSetupDataSO_AppInitSceneSetupData {
    __cordl_parent: crate::GlobalNamespace::SceneSetupData,
    pub _appInitOverrideStartType_k__BackingField: crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitOverrideStartType,
}
#[cfg(feature = "AppInitScenesTransitionSetupDataSO+AppInitSceneSetupData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitSceneSetupData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "AppInitScenesTransitionSetupDataSO/AppInitSceneSetupData";
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
#[cfg(feature = "AppInitScenesTransitionSetupDataSO+AppInitSceneSetupData")]
impl std::ops::Deref
for crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitSceneSetupData {
    type Target = crate::GlobalNamespace::SceneSetupData;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AppInitScenesTransitionSetupDataSO+AppInitSceneSetupData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitSceneSetupData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AppInitScenesTransitionSetupDataSO+AppInitSceneSetupData")]
impl crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitSceneSetupData {
    pub fn New(
        appInitOverrideStartType: crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitOverrideStartType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (appInitOverrideStartType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        appInitOverrideStartType: crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitOverrideStartType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitOverrideStartType),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (appInitOverrideStartType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_appInitOverrideStartType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitOverrideStartType,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitOverrideStartType,
                        0usize,
                    >("get_appInitOverrideStartType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_appInitOverrideStartType", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitOverrideStartType = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_appInitOverrideStartType(
        &mut self,
        value: crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitOverrideStartType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitOverrideStartType),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_appInitOverrideStartType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "set_appInitOverrideStartType", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AppInitScenesTransitionSetupDataSO+AppInitSceneSetupData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitSceneSetupData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
