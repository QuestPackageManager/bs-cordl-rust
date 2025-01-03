#[cfg(feature = "HoudiniEngineUnity+HEU_AssetSync")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_AssetSync {
    __cordl_parent: crate::HoudiniEngineUnity::HEU_BaseSync,
    pub _onAssetLoaded: quest_hook::libil2cpp::Gc<
        crate::HoudiniEngineUnity::HEU_AssetSync_AssetSyncCallback,
    >,
    pub _assetPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetSync")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_AssetSync =>
    "HoudiniEngineUnity"."HEU_AssetSync"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetSync")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_AssetSync {
    type Target = crate::HoudiniEngineUnity::HEU_BaseSync;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetSync")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_AssetSync {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetSync")]
impl crate::HoudiniEngineUnity::HEU_AssetSync {
    #[cfg(feature = "HoudiniEngineUnity+HEU_AssetSync+AssetSyncCallback")]
    pub type AssetSyncCallback = crate::HoudiniEngineUnity::HEU_AssetSync_AssetSyncCallback;
    pub fn CreateThreadedTask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo,
        > = __cordl_object.invoke("CreateThreadedTask", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeAsset(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        assetPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nodeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        startPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InitializeAsset",
                (session, assetPath, nodeName, parent, startPosition),
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
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnLoadComplete(
        &mut self,
        loadData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnLoadComplete", (loadData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Resync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Resync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLoadCallback(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadCallback,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLoadCallback", (callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetupLoadTask(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupLoadTask", (session))?;
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
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetSync")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_AssetSync {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetSync+AssetSyncCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_AssetSync_AssetSyncCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetSync+AssetSyncCallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_AssetSync_AssetSyncCallback => "HoudiniEngineUnity"
    ."HEU_AssetSync/AssetSyncCallback"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetSync+AssetSyncCallback")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_AssetSync_AssetSyncCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetSync+AssetSyncCallback")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_AssetSync_AssetSyncCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetSync+AssetSyncCallback")]
impl crate::HoudiniEngineUnity::HEU_AssetSync_AssetSyncCallback {
    pub fn BeginInvoke(
        &mut self,
        assetSync: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_AssetSync>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (assetSync, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        assetSync: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_AssetSync>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (assetSync))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetSync+AssetSyncCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_AssetSync_AssetSyncCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
