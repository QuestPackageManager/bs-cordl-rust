#[cfg(feature = "RenderingParamsSetupApplicator")]
#[repr(C)]
#[derive(Debug)]
pub struct RenderingParamsSetupApplicator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _settingsManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SettingsManager,
    >,
    pub _settingsApplicator: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SettingsApplicatorSO,
    >,
}
#[cfg(feature = "RenderingParamsSetupApplicator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::RenderingParamsSetupApplicator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "RenderingParamsSetupApplicator";
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
#[cfg(feature = "RenderingParamsSetupApplicator")]
impl std::ops::Deref for crate::GlobalNamespace::RenderingParamsSetupApplicator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RenderingParamsSetupApplicator")]
impl std::ops::DerefMut for crate::GlobalNamespace::RenderingParamsSetupApplicator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RenderingParamsSetupApplicator")]
impl crate::GlobalNamespace::RenderingParamsSetupApplicator {
    pub fn Apply(
        &mut self,
        sceneType: crate::GlobalNamespace::SceneType,
        optionalEnvironmentSerializedName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::RenderingParamsSetupApplicator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::GlobalNamespace::SceneType,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Apply")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::RenderingParamsSetupApplicator as
                    quest_hook::libil2cpp::Type > ::class(), "Apply", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(self, (sceneType, optionalEnvironmentSerializedName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ApplyGraphicsSettings(
        &mut self,
        sceneType: crate::GlobalNamespace::SceneType,
        optionalEnvironmentSerializedName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::RenderingParamsSetupApplicator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::GlobalNamespace::SceneType,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ApplyGraphicsSettings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::RenderingParamsSetupApplicator as
                    quest_hook::libil2cpp::Type > ::class(), "ApplyGraphicsSettings",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(self, (sceneType, optionalEnvironmentSerializedName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ApplyMainSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::RenderingParamsSetupApplicator as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ApplyMainSettings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::RenderingParamsSetupApplicator as
                    quest_hook::libil2cpp::Type > ::class(), "ApplyMainSettings", 0usize
                )
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
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::RenderingParamsSetupApplicator as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::RenderingParamsSetupApplicator as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "RenderingParamsSetupApplicator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::RenderingParamsSetupApplicator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "RenderingParamsSetupApplicator")]
impl AsRef<crate::GlobalNamespace::IRenderingParamsApplicator>
for crate::GlobalNamespace::RenderingParamsSetupApplicator {
    fn as_ref(&self) -> &crate::GlobalNamespace::IRenderingParamsApplicator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "RenderingParamsSetupApplicator")]
impl AsMut<crate::GlobalNamespace::IRenderingParamsApplicator>
for crate::GlobalNamespace::RenderingParamsSetupApplicator {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IRenderingParamsApplicator {
        unsafe { std::mem::transmute(self) }
    }
}
