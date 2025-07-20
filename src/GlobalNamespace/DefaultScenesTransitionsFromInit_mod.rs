#[cfg(feature = "DefaultScenesTransitionsFromInit")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultScenesTransitionsFromInit {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _healthWarningScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::HealthWarningScenesTransitionSetupDataSO,
    >,
    pub _recordingToolScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::RecordingToolScenesTransitionSetupDataSO,
    >,
    pub _mainMenuScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MenuScenesTransitionSetupDataSO,
    >,
    pub _beatmapEditorScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapEditorScenesTransitionSetupDataSO,
    >,
    pub _shaderWarmupScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ShaderWarmupScenesTransitionSetupDataSO,
    >,
    pub _startupErrorScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::StartupErrorScenesTransitionSetupDataSO,
    >,
    pub _gameScenesManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameScenesManager,
    >,
}
#[cfg(feature = "DefaultScenesTransitionsFromInit")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::DefaultScenesTransitionsFromInit {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "DefaultScenesTransitionsFromInit";
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
#[cfg(feature = "DefaultScenesTransitionsFromInit")]
impl std::ops::Deref for crate::GlobalNamespace::DefaultScenesTransitionsFromInit {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DefaultScenesTransitionsFromInit")]
impl std::ops::DerefMut for crate::GlobalNamespace::DefaultScenesTransitionsFromInit {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DefaultScenesTransitionsFromInit")]
impl crate::GlobalNamespace::DefaultScenesTransitionsFromInit {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn TransitionToNextScene(
        &mut self,
        goStraightToMenu: bool,
        goStraightToEditor: bool,
        goToRecordingToolScene: bool,
        onFinishShaderWarmup: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            bool,
                            bool,
                            bool,
                            quest_hook::libil2cpp::Gc<crate::System::Action>,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("TransitionToNextScene")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "TransitionToNextScene", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        goStraightToMenu,
                        goStraightToEditor,
                        goToRecordingToolScene,
                        onFinishShaderWarmup,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TransitionToStartupErrorScene(
        &mut self,
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subtitle: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("TransitionToStartupErrorScene")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "TransitionToStartupErrorScene", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (title, subtitle))?
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
    pub fn get_mainMenuScenesTransitionSetupData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MenuScenesTransitionSetupDataSO,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::MenuScenesTransitionSetupDataSO,
                        >,
                        0usize,
                    >("get_mainMenuScenesTransitionSetupData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_mainMenuScenesTransitionSetupData",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MenuScenesTransitionSetupDataSO,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "DefaultScenesTransitionsFromInit")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DefaultScenesTransitionsFromInit {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "DefaultScenesTransitionsFromInit+__c__DisplayClass9_0+__TransitionToNextScene_g__OnFinishCallback_0_d"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct __c__DisplayClass9_0_DefaultScenesTransitionsFromInit___TransitionToNextScene_g__OnFinishCallback_0_d {
    pub __1__state: i32,
    pub __t__builder: crate::System::Runtime::CompilerServices::AsyncVoidMethodBuilder,
    pub __4__this: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub __u__1: crate::System::Runtime::CompilerServices::TaskAwaiter,
}
#[cfg(
    feature = "DefaultScenesTransitionsFromInit+__c__DisplayClass9_0+__TransitionToNextScene_g__OnFinishCallback_0_d"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::__c__DisplayClass9_0_DefaultScenesTransitionsFromInit___TransitionToNextScene_g__OnFinishCallback_0_d {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "DefaultScenesTransitionsFromInit/<>c__DisplayClass9_0/<<TransitionToNextScene>g__OnFinishCallback|0>d";
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
#[cfg(
    feature = "DefaultScenesTransitionsFromInit+__c__DisplayClass9_0+__TransitionToNextScene_g__OnFinishCallback_0_d"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::__c__DisplayClass9_0_DefaultScenesTransitionsFromInit___TransitionToNextScene_g__OnFinishCallback_0_d {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "DefaultScenesTransitionsFromInit+__c__DisplayClass9_0+__TransitionToNextScene_g__OnFinishCallback_0_d"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::__c__DisplayClass9_0_DefaultScenesTransitionsFromInit___TransitionToNextScene_g__OnFinishCallback_0_d {
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
#[cfg(
    feature = "DefaultScenesTransitionsFromInit+__c__DisplayClass9_0+__TransitionToNextScene_g__OnFinishCallback_0_d"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::__c__DisplayClass9_0_DefaultScenesTransitionsFromInit___TransitionToNextScene_g__OnFinishCallback_0_d {
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
#[cfg(
    feature = "DefaultScenesTransitionsFromInit+__c__DisplayClass9_0+__TransitionToNextScene_g__OnFinishCallback_0_d"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::__c__DisplayClass9_0_DefaultScenesTransitionsFromInit___TransitionToNextScene_g__OnFinishCallback_0_d {
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
#[cfg(
    feature = "DefaultScenesTransitionsFromInit+__c__DisplayClass9_0+__TransitionToNextScene_g__OnFinishCallback_0_d"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::__c__DisplayClass9_0_DefaultScenesTransitionsFromInit___TransitionToNextScene_g__OnFinishCallback_0_d {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "DefaultScenesTransitionsFromInit+__c__DisplayClass9_0+__TransitionToNextScene_g__OnFinishCallback_0_d"
)]
impl crate::GlobalNamespace::__c__DisplayClass9_0_DefaultScenesTransitionsFromInit___TransitionToNextScene_g__OnFinishCallback_0_d {
    pub fn MoveNext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("MoveNext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "MoveNext", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetStateMachine(
        &mut self,
        stateMachine: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::IAsyncStateMachine,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::CompilerServices::IAsyncStateMachine,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetStateMachine")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SetStateMachine", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (stateMachine))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "DefaultScenesTransitionsFromInit+__c__DisplayClass9_0+__TransitionToNextScene_g__OnFinishCallback_0_d"
)]
impl AsRef<crate::System::Runtime::CompilerServices::IAsyncStateMachine>
for crate::GlobalNamespace::__c__DisplayClass9_0_DefaultScenesTransitionsFromInit___TransitionToNextScene_g__OnFinishCallback_0_d {
    fn as_ref(&self) -> &crate::System::Runtime::CompilerServices::IAsyncStateMachine {
        todo!()
    }
}
#[cfg(
    feature = "DefaultScenesTransitionsFromInit+__c__DisplayClass9_0+__TransitionToNextScene_g__OnFinishCallback_0_d"
)]
impl AsMut<crate::System::Runtime::CompilerServices::IAsyncStateMachine>
for crate::GlobalNamespace::__c__DisplayClass9_0_DefaultScenesTransitionsFromInit___TransitionToNextScene_g__OnFinishCallback_0_d {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::CompilerServices::IAsyncStateMachine {
        todo!()
    }
}
