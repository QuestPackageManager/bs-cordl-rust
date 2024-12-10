#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct SceneProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider =>
    "UnityEngine.ResourceManagement.ResourceProviders"."SceneProvider"
);
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider")]
impl crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider {
    #[cfg(
        feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+SceneOp"
    )]
    pub type SceneOp = crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider_SceneOp;
    #[cfg(
        feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+UnloadSceneOp"
    )]
    pub type UnloadSceneOp = crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider_UnloadSceneOp;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ProvideScene_LoadSceneMode0(
        &mut self,
        resourceManager: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceManager,
        >,
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        loadSceneMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        activateOnLoad: bool,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = __cordl_object
            .invoke(
                "ProvideScene",
                (resourceManager, location, loadSceneMode, activateOnLoad, priority),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ProvideScene_LoadSceneParameters1(
        &mut self,
        resourceManager: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceManager,
        >,
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        loadSceneParameters: crate::UnityEngine::SceneManagement::LoadSceneParameters,
        activateOnLoad: bool,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = __cordl_object
            .invoke(
                "ProvideScene",
                (
                    resourceManager,
                    location,
                    loadSceneParameters,
                    activateOnLoad,
                    priority,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseScene(
        &mut self,
        resourceManager: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceManager,
        >,
        sceneLoadHandle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = __cordl_object.invoke("ReleaseScene", (resourceManager, sceneLoadHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ResourceManagement_ResourceProviders_ISceneProvider2_ReleaseScene(
        &mut self,
        resourceManager: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceManager,
        >,
        sceneLoadHandle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
        unloadOptions: crate::UnityEngine::SceneManagement::UnloadSceneOptions,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = __cordl_object
            .invoke(
                "UnityEngine.ResourceManagement.ResourceProviders.ISceneProvider2.ReleaseScene",
                (resourceManager, sceneLoadHandle, unloadOptions),
            )?;
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
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider")]
impl AsRef<crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider>
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider")]
impl AsMut<crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider>
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider")]
impl AsRef<crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider2>
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider2 {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider")]
impl AsMut<crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider2>
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider2 {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+SceneOp"
)]
#[repr(C)]
#[derive(Debug)]
pub struct SceneProvider_SceneOp {
    __cordl_parent: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
    >,
    pub m_ActivateOnLoad: bool,
    pub m_Inst: crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
    pub m_Location: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    pub m_LoadSceneParameters: crate::UnityEngine::SceneManagement::LoadSceneParameters,
    pub m_Priority: i32,
    pub m_DepOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
        *mut crate::System::Collections::Generic::IList_1<
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        >,
    >,
    pub m_ResourceManager: *mut crate::UnityEngine::ResourceManagement::ResourceManager,
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+SceneOp"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider_SceneOp =>
    "UnityEngine.ResourceManagement.ResourceProviders"."SceneProvider/SceneOp"
);
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+SceneOp"
)]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider_SceneOp {
    type Target = crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+SceneOp"
)]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider_SceneOp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+SceneOp"
)]
impl crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider_SceneOp {
    pub fn Destroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Destroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Execute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Execute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDependencies(
        &mut self,
        deps: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetDependencies", (deps))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDownloadStatus(
        &mut self,
        visited: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::DownloadStatus,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::DownloadStatus = __cordl_object
            .invoke("GetDownloadStatus", (visited))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init_LoadSceneMode0(
        &mut self,
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        loadSceneMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        activateOnLoad: bool,
        priority: i32,
        depOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (location, loadSceneMode, activateOnLoad, priority, depOp))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init_LoadSceneParameters1(
        &mut self,
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        loadSceneParameters: crate::UnityEngine::SceneManagement::LoadSceneParameters,
        activateOnLoad: bool,
        priority: i32,
        depOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (location, loadSceneParameters, activateOnLoad, priority, depOp),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalLoad(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loadingFromBundle: bool,
        loadSceneParameters: crate::UnityEngine::SceneManagement::LoadSceneParameters,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = __cordl_object
            .invoke("InternalLoad", (path, loadingFromBundle, loadSceneParameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalLoadScene(
        &mut self,
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        loadingFromBundle: bool,
        loadSceneParameters: crate::UnityEngine::SceneManagement::LoadSceneParameters,
        activateOnLoad: bool,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance = __cordl_object
            .invoke(
                "InternalLoadScene",
                (
                    location,
                    loadingFromBundle,
                    loadSceneParameters,
                    activateOnLoad,
                    priority,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeWaitForCompletion(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("InvokeWaitForCompletion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        rm: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rm))?;
        Ok(__cordl_object.into())
    }
    pub fn UnityEngine_ResourceManagement_IUpdateReceiver_Update(
        &mut self,
        unscaledDeltaTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.ResourceManagement.IUpdateReceiver.Update",
                (unscaledDeltaTime),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        rm: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (rm))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DebugName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_DebugName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Progress(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_Progress", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+SceneOp"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider_SceneOp {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+SceneOp"
)]
impl AsRef<crate::UnityEngine::ResourceManagement::IUpdateReceiver>
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider_SceneOp {
    fn as_ref(&self) -> &crate::UnityEngine::ResourceManagement::IUpdateReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+SceneOp"
)]
impl AsMut<crate::UnityEngine::ResourceManagement::IUpdateReceiver>
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider_SceneOp {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ResourceManagement::IUpdateReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+UnloadSceneOp"
)]
#[repr(C)]
#[derive(Debug)]
pub struct SceneProvider_UnloadSceneOp {
    __cordl_parent: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
    >,
    pub m_Instance: crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
    pub m_sceneLoadHandle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
        crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
    >,
    pub m_UnloadOptions: crate::UnityEngine::SceneManagement::UnloadSceneOptions,
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+UnloadSceneOp"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider_UnloadSceneOp =>
    "UnityEngine.ResourceManagement.ResourceProviders"."SceneProvider/UnloadSceneOp"
);
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+UnloadSceneOp"
)]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider_UnloadSceneOp {
    type Target = crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+UnloadSceneOp"
)]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider_UnloadSceneOp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+UnloadSceneOp"
)]
impl crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider_UnloadSceneOp {
    pub fn Execute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Execute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        sceneLoadHandle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
        options: crate::UnityEngine::SceneManagement::UnloadSceneOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (sceneLoadHandle, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeWaitForCompletion(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("InvokeWaitForCompletion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UnloadSceneCompleted(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnloadSceneCompleted", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnloadSceneCompletedNoRelease(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnloadSceneCompletedNoRelease", (obj))?;
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
    pub fn get_Progress(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_Progress", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+UnloadSceneOp"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider_UnloadSceneOp {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
