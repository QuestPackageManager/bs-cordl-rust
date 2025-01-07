#[cfg(feature = "ShaderWarmupSceneSetupData")]
#[repr(C)]
#[derive(Debug)]
pub struct ShaderWarmupSceneSetupData {
    __cordl_parent: crate::GlobalNamespace::SceneSetupData,
    pub _nextScenesTransitionSetupData_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ScenesTransitionSetupDataSO,
    >,
}
#[cfg(feature = "ShaderWarmupSceneSetupData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ShaderWarmupSceneSetupData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ShaderWarmupSceneSetupData";
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
#[cfg(feature = "ShaderWarmupSceneSetupData")]
impl std::ops::Deref for crate::GlobalNamespace::ShaderWarmupSceneSetupData {
    type Target = crate::GlobalNamespace::SceneSetupData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ShaderWarmupSceneSetupData")]
impl std::ops::DerefMut for crate::GlobalNamespace::ShaderWarmupSceneSetupData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ShaderWarmupSceneSetupData")]
impl crate::GlobalNamespace::ShaderWarmupSceneSetupData {
    pub fn New(
        nextScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScenesTransitionSetupDataSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nextScenesTransitionSetupData))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        nextScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScenesTransitionSetupDataSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nextScenesTransitionSetupData))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_nextScenesTransitionSetupData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ScenesTransitionSetupDataSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScenesTransitionSetupDataSO,
        > = __cordl_object.invoke("get_nextScenesTransitionSetupData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_nextScenesTransitionSetupData(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScenesTransitionSetupDataSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_nextScenesTransitionSetupData", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ShaderWarmupSceneSetupData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ShaderWarmupSceneSetupData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
