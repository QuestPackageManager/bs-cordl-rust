#[cfg(feature = "HoudiniEngineUnity+HEU_PDGSession")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_PDGSession {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _pdgAssets: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_PDGAssetLink>,
        >,
    >,
    pub _pdgMaxProcessEvents: i32,
    pub _pdgQueryEvents: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::HoudiniEngineUnity::HAPI_PDG_EventInfo>,
    >,
    pub _pdgContextSize: i32,
    pub _pdgContextIDs: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub _errored: bool,
    pub _errorMsg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _pdgState: crate::HoudiniEngineUnity::HAPI_PDG_State,
    pub _pdgEventMessages: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    pub _eventMessageColorCode: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGSession")]
unsafe impl quest_hook::libil2cpp::Type for crate::HoudiniEngineUnity::HEU_PDGSession {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_PDGSession";
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
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGSession")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_PDGSession {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGSession")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_PDGSession {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGSession")]
impl crate::HoudiniEngineUnity::HEU_PDGSession {
    #[cfg(feature = "HoudiniEngineUnity+HEU_PDGSession+EventMessageColor")]
    pub type EventMessageColor = crate::HoudiniEngineUnity::HEU_PDGSession_EventMessageColor;
    pub fn AddAsset(
        &mut self,
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_PDGAssetLink>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_PDGAssetLink>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("AddAsset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddAsset", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (asset))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddEventMessage(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("AddEventMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddEventMessage", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (msg))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CancelCook(
        &mut self,
        topNetwork: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_TOPNetworkData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::HoudiniEngineUnity::HEU_TOPNetworkData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CancelCook")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CancelCook", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (topNetwork))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CleanUp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("CleanUp")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CleanUp", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearErrorState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ClearErrorState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ClearErrorState", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearEventMessages(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ClearEventMessages")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ClearEventMessages", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearWorkItemResult(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        contextID: i32,
        eventInfo: crate::HoudiniEngineUnity::HAPI_PDG_EventInfo,
        topNode: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNodeData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_SessionBase,
                    >,
                    i32,
                    crate::HoudiniEngineUnity::HAPI_PDG_EventInfo,
                    quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNodeData>,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("ClearWorkItemResult")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ClearWorkItemResult", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (session, contextID, eventInfo, topNode))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CookTOPNetworkOutputNode(
        &mut self,
        topNetwork: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_TOPNetworkData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::HoudiniEngineUnity::HEU_TOPNetworkData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CookTOPNetworkOutputNode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CookTOPNetworkOutputNode", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (topNetwork))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CookTOPNode(&mut self, nodeID: i32) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), bool, 1usize>("CookTOPNode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CookTOPNode", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (nodeID)) };
        Ok(__cordl_ret.into())
    }
    pub fn DirtyAll(&mut self, nodeID: i32) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), bool, 1usize>("DirtyAll")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DirtyAll", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (nodeID)) };
        Ok(__cordl_ret.into())
    }
    pub fn DirtyTOPNode(&mut self, nodeID: i32) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), bool, 1usize>("DirtyTOPNode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DirtyTOPNode", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (nodeID)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetEventMessages(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("GetEventMessages")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetEventMessages", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetHAPIPDGSession(
        &mut self,
        bCreate: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
                1usize,
            >("GetHAPIPDGSession")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetHAPIPDGSession", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_SessionBase,
        > = unsafe { method.invoke_unchecked(self, (bCreate)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetPDGSession() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_PDGSession>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_PDGSession>,
                0usize,
            >("GetPDGSession")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetPDGSession", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_PDGSession,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetTOPAssetLinkAndNode(
        &mut self,
        nodeID: i32,
        assetLink: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_PDGAssetLink>,
        >,
        topNode: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNodeData>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            crate::HoudiniEngineUnity::HEU_PDGAssetLink,
                        >,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            crate::HoudiniEngineUnity::HEU_TOPNodeData,
                        >,
                    >,
                ),
                bool,
                3usize,
            >("GetTOPAssetLinkAndNode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTOPAssetLinkAndNode", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (nodeID, assetLink, topNode))
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
    pub fn NotifyTOPNodeCookedWorkItem(
        &mut self,
        assetLink: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_PDGAssetLink,
        >,
        topNode: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNodeData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_PDGAssetLink,
                    >,
                    quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNodeData>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("NotifyTOPNodeCookedWorkItem")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NotifyTOPNodeCookedWorkItem", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (assetLink, topNode))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NotifyTOPNodeCookingWorkItem(
        &mut self,
        assetLink: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_PDGAssetLink,
        >,
        topNode: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNodeData>,
        inc: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_PDGAssetLink,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_TOPNodeData,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("NotifyTOPNodeCookingWorkItem")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NotifyTOPNodeCookingWorkItem", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (assetLink, topNode, inc))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NotifyTOPNodeErrorWorkItem(
        &mut self,
        assetLink: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_PDGAssetLink,
        >,
        topNode: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNodeData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_PDGAssetLink,
                    >,
                    quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNodeData>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("NotifyTOPNodeErrorWorkItem")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NotifyTOPNodeErrorWorkItem", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (assetLink, topNode))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NotifyTOPNodePDGStateClear(
        &mut self,
        assetLink: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_PDGAssetLink,
        >,
        topNode: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNodeData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_PDGAssetLink,
                    >,
                    quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNodeData>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("NotifyTOPNodePDGStateClear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NotifyTOPNodePDGStateClear", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (assetLink, topNode))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NotifyTOPNodeScheduledWorkItem(
        &mut self,
        assetLink: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_PDGAssetLink,
        >,
        topNode: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNodeData>,
        inc: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_PDGAssetLink,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_TOPNodeData,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("NotifyTOPNodeScheduledWorkItem")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NotifyTOPNodeScheduledWorkItem", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (assetLink, topNode, inc))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NotifyTOPNodeTotalWorkItem(
        &mut self,
        assetLink: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_PDGAssetLink,
        >,
        topNode: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNodeData>,
        inc: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_PDGAssetLink,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_TOPNodeData,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("NotifyTOPNodeTotalWorkItem")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NotifyTOPNodeTotalWorkItem", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (assetLink, topNode, inc))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NotifyTOPNodeWaitingWorkItem(
        &mut self,
        assetLink: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_PDGAssetLink,
        >,
        topNode: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNodeData>,
        inc: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_PDGAssetLink,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_TOPNodeData,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("NotifyTOPNodeWaitingWorkItem")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NotifyTOPNodeWaitingWorkItem", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (assetLink, topNode, inc))
        };
        Ok(__cordl_ret.into())
    }
    pub fn PauseCook(
        &mut self,
        topNetwork: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_TOPNetworkData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::HoudiniEngineUnity::HEU_TOPNetworkData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("PauseCook")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PauseCook", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (topNetwork))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessPDGEvent(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        contextID: i32,
        eventInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_PDG_EventInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_SessionBase,
                    >,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::HoudiniEngineUnity::HAPI_PDG_EventInfo,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("ProcessPDGEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessPDGEvent", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (session, contextID, eventInfo))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReinitializePDGContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ReinitializePDGContext")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReinitializePDGContext", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAsset(
        &mut self,
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_PDGAssetLink>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_PDGAssetLink>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RemoveAsset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RemoveAsset", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (asset))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResetPDGEventInfo(
        eventInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_PDG_EventInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    crate::HoudiniEngineUnity::HAPI_PDG_EventInfo,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ResetPDGEventInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ResetPDGEventInfo", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (eventInfo))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetErrorState(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bLogIt: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetErrorState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetErrorState", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (msg, bLogIt))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetTOPNodePDGState(
        &mut self,
        assetLink: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_PDGAssetLink,
        >,
        topNode: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPNodeData>,
        pdgState: crate::HoudiniEngineUnity::HEU_TOPNodeData_PDGState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_PDGAssetLink,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_TOPNodeData,
                    >,
                    crate::HoudiniEngineUnity::HEU_TOPNodeData_PDGState,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetTOPNodePDGState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetTOPNodePDGState", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (assetLink, topNode, pdgState))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Update")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Update", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdatePDGContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("UpdatePDGContext")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UpdatePDGContext", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGSession")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_PDGSession {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGSession+EventMessageColor")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HEU_PDGSession_EventMessageColor {
    #[default]
    DEFAULT = 0i32,
    ERROR = 2i32,
    WARNING = 1i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGSession+EventMessageColor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::HEU_PDGSession_EventMessageColor {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_PDGSession/EventMessageColor";
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
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGSession+EventMessageColor")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::HoudiniEngineUnity::HEU_PDGSession_EventMessageColor {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGSession+EventMessageColor")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::HoudiniEngineUnity::HEU_PDGSession_EventMessageColor {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGSession+EventMessageColor")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::HoudiniEngineUnity::HEU_PDGSession_EventMessageColor {
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
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGSession+EventMessageColor")]
unsafe impl quest_hook::libil2cpp::Return
for crate::HoudiniEngineUnity::HEU_PDGSession_EventMessageColor {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
