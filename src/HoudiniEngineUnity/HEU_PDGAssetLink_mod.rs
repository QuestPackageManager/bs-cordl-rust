#[cfg(feature = "cordl_class_HoudiniEngineUnity+HEU_PDGAssetLink")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_PDGAssetLink {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _assetPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _assetGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _assetName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _assetID: i32,
    pub _heu: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
    pub _topNetworks: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNetworkData>,
        >,
    >,
    pub _topNetworkNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub _selectedTOPNetwork: i32,
    pub _linkState: crate::HoudiniEngineUnity::HEU_PDGAssetLink_LinkState,
    pub _autoCook: bool,
    pub _useHEngineData: bool,
    pub _repaintUIDelegate: quest_hook::libil2cpp::Gc<
        crate::HoudiniEngineUnity::HEU_PDGAssetLink_UpdateUIDelegate,
    >,
    pub _numWorkItems: i32,
    pub _workItemTally: quest_hook::libil2cpp::Gc<
        crate::HoudiniEngineUnity::HEU_WorkItemTally,
    >,
    pub _loadRootGameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _outputCachePathRoot: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "cordl_class_HoudiniEngineUnity+HEU_PDGAssetLink")]
unsafe impl quest_hook::libil2cpp::Type for crate::HoudiniEngineUnity::HEU_PDGAssetLink {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_PDGAssetLink";
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
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGAssetLink")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_PDGAssetLink {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGAssetLink")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_PDGAssetLink {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGAssetLink")]
impl crate::HoudiniEngineUnity::HEU_PDGAssetLink {
    #[cfg(feature = "HoudiniEngineUnity+HEU_PDGAssetLink+LinkState")]
    pub type LinkState = crate::HoudiniEngineUnity::HEU_PDGAssetLink_LinkState;
    #[cfg(feature = "HoudiniEngineUnity+HEU_PDGAssetLink+UpdateUIDelegate")]
    pub type UpdateUIDelegate = crate::HoudiniEngineUnity::HEU_PDGAssetLink_UpdateUIDelegate;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Awake")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Awake",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CancelCook(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("CancelCook")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CancelCook", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearAllTOPData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("ClearAllTOPData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ClearAllTOPData", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearTOPNetworkWorkItemResults(
        topNetwork: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_TOPNetworkData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::HoudiniEngineUnity::HEU_TOPNetworkData,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ClearTOPNetworkWorkItemResults")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ClearTOPNetworkWorkItemResults", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (topNetwork))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearTOPNodeWorkItemResults(
        topNode: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNodeData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::HoudiniEngineUnity::HEU_TOPNodeData,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ClearTOPNodeWorkItemResults")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ClearTOPNodeWorkItemResults", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (topNode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearWorkItemResult(
        topNode: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNodeData>,
        result: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPWorkResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_TOPNodeData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_TOPWorkResult,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ClearWorkItemResult")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ClearWorkItemResult", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (topNode, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearWorkItemResultByID(
        topNode: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNodeData>,
        workItemID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_TOPNodeData,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ClearWorkItemResultByID")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ClearWorkItemResultByID", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (topNode, workItemID))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CookOutput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("CookOutput")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CookOutput", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CookTOPNode(
        &mut self,
        topNode: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNodeData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::HoudiniEngineUnity::HEU_TOPNodeData,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CookTOPNode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CookTOPNode", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (topNode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DestroyWorkItemResultData(
        topNode: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNodeData>,
        result: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPWorkResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_TOPNodeData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_TOPWorkResult,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("DestroyWorkItemResultData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DestroyWorkItemResultData", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (topNode, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DirtyAll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("DirtyAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DirtyAll", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DirtyTOPNode(
        &mut self,
        topNode: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNodeData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::HoudiniEngineUnity::HEU_TOPNodeData,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("DirtyTOPNode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DirtyTOPNode", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (topNode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetHAPISession(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::HoudiniEngineUnity::HEU_SessionBase,
                        >,
                        0usize,
                    >("GetHAPISession")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetHAPISession", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_SessionBase,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetLoadRootTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                        0usize,
                    >("GetLoadRootTransform")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetLoadRootTransform", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSelectedTOPNetwork(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNetworkData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::HoudiniEngineUnity::HEU_TOPNetworkData,
                        >,
                        0usize,
                    >("GetSelectedTOPNetwork")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSelectedTOPNetwork", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_TOPNetworkData,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSelectedTOPNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNodeData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::HoudiniEngineUnity::HEU_TOPNodeData,
                        >,
                        0usize,
                    >("GetSelectedTOPNode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSelectedTOPNode", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_TOPNodeData,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTOPNetwork(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNetworkData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Gc<
                            crate::HoudiniEngineUnity::HEU_TOPNetworkData,
                        >,
                        1usize,
                    >("GetTOPNetwork")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTOPNetwork", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_TOPNetworkData,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (index))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTOPNetworkByName(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        topNetworks: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNetworkData>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNetworkData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::HoudiniEngineUnity::HEU_TOPNetworkData,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::HoudiniEngineUnity::HEU_TOPNetworkData,
                        >,
                        2usize,
                    >("GetTOPNetworkByName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTOPNetworkByName", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_TOPNetworkData,
        > = unsafe { cordl_method_info.invoke_unchecked((), (name, topNetworks))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTOPNode(
        &mut self,
        nodeID: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNodeData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Gc<
                            crate::HoudiniEngineUnity::HEU_TOPNodeData,
                        >,
                        1usize,
                    >("GetTOPNode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTOPNode", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_TOPNodeData,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (nodeID))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTOPNodeByName(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        topNodes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNodeData>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNodeData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::HoudiniEngineUnity::HEU_TOPNodeData,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::HoudiniEngineUnity::HEU_TOPNodeData,
                        >,
                        2usize,
                    >("GetTOPNodeByName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTOPNodeByName", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_TOPNodeData,
        > = unsafe { cordl_method_info.invoke_unchecked((), (name, topNodes))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTOPNodeStatus(
        &mut self,
        topNode: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNodeData>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::HoudiniEngineUnity::HEU_TOPNodeData,
                        >),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("GetTOPNodeStatus")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTOPNodeStatus", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (topNode))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetWorkResultByID(
        topNode: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNodeData>,
        workItemID: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPWorkResult>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_TOPNodeData,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::HoudiniEngineUnity::HEU_TOPWorkResult,
                        >,
                        2usize,
                    >("GetWorkResultByID")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetWorkResultByID", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_TOPWorkResult,
        > = unsafe { cordl_method_info.invoke_unchecked((), (topNode, workItemID))? };
        Ok(__cordl_ret.into())
    }
    pub fn HandleInitialLoad(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("HandleInitialLoad")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleInitialLoad", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadResults(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        topNode: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNodeData>,
        workItemInfo: crate::HoudiniEngineUnity::HAPI_PDG_WorkitemInfo,
        resultInfos: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::HoudiniEngineUnity::HAPI_PDG_WorkitemResultInfo,
            >,
        >,
        workItemID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_SessionBase,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_TOPNodeData,
                            >,
                            crate::HoudiniEngineUnity::HAPI_PDG_WorkitemInfo,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::HoudiniEngineUnity::HAPI_PDG_WorkitemResultInfo,
                                >,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("LoadResults")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadResults", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (session, topNode, workItemInfo, resultInfos, workItemID),
                )?
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
    pub fn NotifyAssetCooked_HEU_CookedEventData1(
        &mut self,
        cookedEventData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_CookedEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::HoudiniEngineUnity::HEU_CookedEventData,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("NotifyAssetCooked")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NotifyAssetCooked", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (cookedEventData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NotifyAssetCooked_HEU_HoudiniAsset__cordl_bool_List_1_0(
        &mut self,
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
        bSuccess: bool,
        generatedOutputs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_HoudiniAsset,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("NotifyAssetCooked")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NotifyAssetCooked", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (asset, bSuccess, generatedOutputs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NotifyAssetCooked_HEU_ReloadEventData2(
        &mut self,
        reloadEventData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_ReloadEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::HoudiniEngineUnity::HEU_ReloadEventData,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("NotifyAssetCooked")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NotifyAssetCooked", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (reloadEventData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnAfterDeserialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("OnAfterDeserialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnAfterDeserialize", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnBeforeSerialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("OnBeforeSerialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnBeforeSerialize", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDestroy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnDestroy", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseHEngineData(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        topNodeID: i32,
        nodeInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_NodeInfo,
        >,
        nodeTags: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::TOPNodeTags>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_SessionBase,
                            >,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::HoudiniEngineUnity::HAPI_NodeInfo,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::HoudiniEngineUnity::TOPNodeTags,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("ParseHEngineData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ParseHEngineData", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (session, topNodeID, nodeInfo, nodeTags))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PauseCook(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("PauseCook")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PauseCook", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PopulateFromHDA(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("PopulateFromHDA")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PopulateFromHDA", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PopulateTOPNetworks(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("PopulateTOPNetworks")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PopulateTOPNetworks", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn PopulateTOPNodes(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        topNetwork: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_TOPNetworkData,
        >,
        topNodeIDs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        useHEngineData: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_SessionBase,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_TOPNetworkData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<i32>,
                            >,
                            bool,
                        ),
                        bool,
                        4usize,
                    >("PopulateTOPNodes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PopulateTOPNodes", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (session, topNetwork, topNodeIDs, useHEngineData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Refresh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Refresh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Refresh",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RepaintUI(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("RepaintUI")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RepaintUI", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Reset",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResetTOPNetworkWorkItemTally(
        &mut self,
        topNetwork: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_TOPNetworkData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::HoudiniEngineUnity::HEU_TOPNetworkData,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ResetTOPNetworkWorkItemTally")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ResetTOPNetworkWorkItemTally", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (topNetwork))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SelectTOPNetwork(
        &mut self,
        newIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SelectTOPNetwork")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SelectTOPNetwork", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (newIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SelectTOPNode(
        &mut self,
        network: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_TOPNetworkData,
        >,
        newIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_TOPNetworkData,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SelectTOPNode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SelectTOPNode", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (network, newIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Setup(
        &mut self,
        hdaAsset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::HoudiniEngineUnity::HEU_HoudiniAsset,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Setup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Setup",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (hdaAsset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateTOPNodeResultsVisibility(
        &mut self,
        topNode: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNodeData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::HoudiniEngineUnity::HEU_TOPNodeData,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UpdateTOPNodeResultsVisibility")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateTOPNodeResultsVisibility", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (topNode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateWorkItemTally(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("UpdateWorkItemTally")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateWorkItemTally", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
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
    pub fn get_AssetLinkState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::HoudiniEngineUnity::HEU_PDGAssetLink_LinkState,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::HoudiniEngineUnity::HEU_PDGAssetLink_LinkState,
                        0usize,
                    >("get_AssetLinkState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_AssetLinkState", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::HoudiniEngineUnity::HEU_PDGAssetLink_LinkState = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_AssetName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_AssetName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_AssetName", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_SelectedTOPNetwork(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_SelectedTOPNetwork")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_SelectedTOPNetwork", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_HoudiniEngineUnity+HEU_PDGAssetLink")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_PDGAssetLink {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGAssetLink")]
impl AsRef<crate::UnityEngine::ISerializationCallbackReceiver>
for crate::HoudiniEngineUnity::HEU_PDGAssetLink {
    fn as_ref(&self) -> &crate::UnityEngine::ISerializationCallbackReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGAssetLink")]
impl AsMut<crate::UnityEngine::ISerializationCallbackReceiver>
for crate::HoudiniEngineUnity::HEU_PDGAssetLink {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::ISerializationCallbackReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "cordl_class_HoudiniEngineUnity+HEU_PDGAssetLink+LinkState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HEU_PDGAssetLink_LinkState {
    #[default]
    ERROR_NOT_LINKED = 3i32,
    INACTIVE = 0i32,
    LINKED = 2i32,
    LINKING = 1i32,
}
#[cfg(feature = "cordl_class_HoudiniEngineUnity+HEU_PDGAssetLink+LinkState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::HEU_PDGAssetLink_LinkState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_PDGAssetLink/LinkState";
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
#[cfg(feature = "cordl_class_HoudiniEngineUnity+HEU_PDGAssetLink+LinkState")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::HoudiniEngineUnity::HEU_PDGAssetLink_LinkState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_HoudiniEngineUnity+HEU_PDGAssetLink+LinkState")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::HoudiniEngineUnity::HEU_PDGAssetLink_LinkState {
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
#[cfg(feature = "cordl_class_HoudiniEngineUnity+HEU_PDGAssetLink+LinkState")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::HoudiniEngineUnity::HEU_PDGAssetLink_LinkState {
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
#[cfg(feature = "cordl_class_HoudiniEngineUnity+HEU_PDGAssetLink+LinkState")]
unsafe impl quest_hook::libil2cpp::Return
for crate::HoudiniEngineUnity::HEU_PDGAssetLink_LinkState {
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
#[cfg(feature = "cordl_class_HoudiniEngineUnity+HEU_PDGAssetLink+UpdateUIDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_PDGAssetLink_UpdateUIDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "cordl_class_HoudiniEngineUnity+HEU_PDGAssetLink+UpdateUIDelegate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::HEU_PDGAssetLink_UpdateUIDelegate {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_PDGAssetLink/UpdateUIDelegate";
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
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGAssetLink+UpdateUIDelegate")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_PDGAssetLink_UpdateUIDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGAssetLink+UpdateUIDelegate")]
impl std::ops::DerefMut
for crate::HoudiniEngineUnity::HEU_PDGAssetLink_UpdateUIDelegate {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGAssetLink+UpdateUIDelegate")]
impl crate::HoudiniEngineUnity::HEU_PDGAssetLink_UpdateUIDelegate {
    pub fn BeginInvoke(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        2usize,
                    >("BeginInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginInvoke", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            cordl_method_info.invoke_unchecked(self, (callback, object))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndInvoke", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Invoke",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_HoudiniEngineUnity+HEU_PDGAssetLink+UpdateUIDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_PDGAssetLink_UpdateUIDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
