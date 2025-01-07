#[cfg(feature = "UnityEngine+SceneManagement+SceneManagerAPI")]
#[repr(C)]
#[derive(Debug)]
pub struct SceneManagerAPI {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+SceneManagement+SceneManagerAPI")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::SceneManagement::SceneManagerAPI {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.SceneManagement";
    const CLASS_NAME: &'static str = "SceneManagerAPI";
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
#[cfg(feature = "UnityEngine+SceneManagement+SceneManagerAPI")]
impl std::ops::Deref for crate::UnityEngine::SceneManagement::SceneManagerAPI {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SceneManagement+SceneManagerAPI")]
impl std::ops::DerefMut for crate::UnityEngine::SceneManagement::SceneManagerAPI {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SceneManagement+SceneManagerAPI")]
impl crate::UnityEngine::SceneManagement::SceneManagerAPI {
    pub fn GetNumScenesInBuildSettings(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetNumScenesInBuildSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadFirstScene(
        &mut self,
        mustLoadAsync: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = __cordl_object
            .invoke("LoadFirstScene", (mustLoadAsync))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsyncByNameOrIndex(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sceneBuildIndex: i32,
        parameters: crate::UnityEngine::SceneManagement::LoadSceneParameters,
        mustCompleteNextFrame: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = __cordl_object
            .invoke(
                "LoadSceneAsyncByNameOrIndex",
                (sceneName, sceneBuildIndex, parameters, mustCompleteNextFrame),
            )?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ActiveAPI() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::SceneManagement::SceneManagerAPI>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::SceneManagement::SceneManagerAPI,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_ActiveAPI", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_overrideAPI() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::SceneManagement::SceneManagerAPI>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::SceneManagement::SceneManagerAPI,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_overrideAPI", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+SceneManagement+SceneManagerAPI")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::SceneManagement::SceneManagerAPI {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
