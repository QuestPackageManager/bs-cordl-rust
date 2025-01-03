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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::DefaultScenesTransitionsFromInit => ""
    ."DefaultScenesTransitionsFromInit"
);
#[cfg(feature = "DefaultScenesTransitionsFromInit")]
impl std::ops::Deref for crate::GlobalNamespace::DefaultScenesTransitionsFromInit {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DefaultScenesTransitionsFromInit")]
impl std::ops::DerefMut for crate::GlobalNamespace::DefaultScenesTransitionsFromInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "TransitionToNextScene",
                (
                    goStraightToMenu,
                    goStraightToEditor,
                    goToRecordingToolScene,
                    onFinishShaderWarmup,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TransitionToStartupErrorScene(
        &mut self,
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subtitle: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TransitionToStartupErrorScene", (title, subtitle))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mainMenuScenesTransitionSetupData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MenuScenesTransitionSetupDataSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MenuScenesTransitionSetupDataSO,
        > = __cordl_object.invoke("get_mainMenuScenesTransitionSetupData", ())?;
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
#[derive(Debug, Clone, Default)]
pub struct __c__DisplayClass9_0_DefaultScenesTransitionsFromInit___TransitionToNextScene_g__OnFinishCallback_0_d {
    pub __1__state: i32,
    pub __t__builder: crate::System::Runtime::CompilerServices::AsyncVoidMethodBuilder,
    pub __4__this: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub __u__1: crate::System::Runtime::CompilerServices::TaskAwaiter,
}
#[cfg(
    feature = "DefaultScenesTransitionsFromInit+__c__DisplayClass9_0+__TransitionToNextScene_g__OnFinishCallback_0_d"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::__c__DisplayClass9_0_DefaultScenesTransitionsFromInit___TransitionToNextScene_g__OnFinishCallback_0_d
    => ""
    ."DefaultScenesTransitionsFromInit/<>c__DisplayClass9_0/<<TransitionToNextScene>g__OnFinishCallback|0>d"
);
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
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStateMachine(
        &mut self,
        stateMachine: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::IAsyncStateMachine,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetStateMachine",
            (stateMachine),
        )?;
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
