#[cfg(feature = "HoudiniEngineUnity+HEU_ObjectNode")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_ObjectNode {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _objName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _parentAsset: quest_hook::libil2cpp::Gc<
        crate::HoudiniEngineUnity::HEU_HoudiniAsset,
    >,
    pub _objectInfo: crate::HoudiniEngineUnity::HAPI_ObjectInfo,
    pub _geoNodes: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_GeoNode>,
        >,
    >,
    pub _objectTransform: crate::HoudiniEngineUnity::HAPI_Transform,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ObjectNode")]
unsafe impl quest_hook::libil2cpp::Type for crate::HoudiniEngineUnity::HEU_ObjectNode {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_ObjectNode";
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
#[cfg(feature = "HoudiniEngineUnity+HEU_ObjectNode")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_ObjectNode {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ObjectNode")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_ObjectNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ObjectNode")]
impl crate::HoudiniEngineUnity::HEU_ObjectNode {
    pub fn ApplyObjectTransformToGeoNodes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ApplyObjectTransformToGeoNodes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(),
                    "ApplyObjectTransformToGeoNodes", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateColliderState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("CalculateColliderState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "CalculateColliderState",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateVisibility(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("CalculateVisibility")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "CalculateVisibility",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearObjectInstances(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ClearObjectInstances")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "ClearObjectInstances",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (session))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateGeoNode(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoInfo: crate::HoudiniEngineUnity::HAPI_GeoInfo,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_GeoNode>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_SessionBase,
                    >,
                    crate::HoudiniEngineUnity::HAPI_GeoInfo,
                ),
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_GeoNode>,
                2usize,
            >("CreateGeoNode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "CreateGeoNode", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GeoNode,
        > = unsafe { method.invoke_unchecked(self, (session, geoInfo))? };
        Ok(__cordl_ret.into())
    }
    pub fn DestroyAllData(
        &mut self,
        bIsRebuild: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("DestroyAllData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "DestroyAllData", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bIsRebuild))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DisableAllColliders(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("DisableAllColliders")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "DisableAllColliders",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateAttributesStore(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("GenerateAttributesStore")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "GenerateAttributesStore",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (session))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateGeometry(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        bRebuild: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_SessionBase,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("GenerateGeometry")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "GenerateGeometry", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (session, bRebuild))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateObjectInstances(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("GenerateObjectInstances")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "GenerateObjectInstances",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (session))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GeneratePartInstances(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("GeneratePartInstances")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "GeneratePartInstances",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (session))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetClonableParts(
        &mut self,
        clonableParts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_PartData>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<
                            crate::HoudiniEngineUnity::HEU_PartData,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("GetClonableParts")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "GetClonableParts", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (clonableParts))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCurves(
        &mut self,
        curves: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_Curve>,
            >,
        >,
        bEditableOnly: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<
                                crate::HoudiniEngineUnity::HEU_Curve,
                            >,
                        >,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("GetCurves")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "GetCurves", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (curves, bEditableOnly))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDebugInfo(
        &mut self,
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("GetDebugInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "GetDebugInfo", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (sb))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetGeoNode(
        &mut self,
        geoName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_GeoNode>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_GeoNode>,
                1usize,
            >("GetGeoNode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "GetGeoNode", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GeoNode,
        > = unsafe { method.invoke_unchecked(self, (geoName))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHDAPartWithGameObject(
        &mut self,
        outputGameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_PartData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>),
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_PartData>,
                1usize,
            >("GetHDAPartWithGameObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "GetHDAPartWithGameObject",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_PartData,
        > = unsafe { method.invoke_unchecked(self, (outputGameObject))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetOutput(
        &mut self,
        outputs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_GeneratedOutput>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<
                            crate::HoudiniEngineUnity::HEU_GeneratedOutput,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("GetOutput")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "GetOutput", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (outputs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOutputGameObjects(
        &mut self,
        outputObjects: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("GetOutputGameObjects")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "GetOutputGameObjects",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (outputObjects))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOutputGeoNodes(
        &mut self,
        outGeoNodes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_GeoNode>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_GeoNode>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("GetOutputGeoNodes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "GetOutputGeoNodes", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (outGeoNodes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HideAllGeometry(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("HideAllGeometry")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "HideAllGeometry", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        objectInfo: crate::HoudiniEngineUnity::HAPI_ObjectInfo,
        objectTranform: crate::HoudiniEngineUnity::HAPI_Transform,
        parentAsset: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_HoudiniAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_SessionBase,
                    >,
                    crate::HoudiniEngineUnity::HAPI_ObjectInfo,
                    crate::HoudiniEngineUnity::HAPI_Transform,
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_HoudiniAsset,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("Initialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "Initialize", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (session, objectInfo, objectTranform, parentAsset),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsEquivalentTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ObjectNode>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ObjectNode>),
                bool,
                1usize,
            >("IsEquivalentTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "IsEquivalentTo", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsInstanced(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("IsInstanced")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "IsInstanced", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn IsInstancer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("IsInstancer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "IsInstancer", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn IsUsingMaterial(
        &mut self,
        materialData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_MaterialData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_MaterialData>),
                bool,
                1usize,
            >("IsUsingMaterial")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "IsUsingMaterial", 1usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (materialData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsVisible(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("IsVisible")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "IsVisible", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PopulateObjectInstanceInfos(
        &mut self,
        objInstanceInfos: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<
                            crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("PopulateObjectInstanceInfos")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(),
                    "PopulateObjectInstanceInfos", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (objInstanceInfos))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessUnityScriptAttributes(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ProcessUnityScriptAttributes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(),
                    "ProcessUnityScriptAttributes", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (session))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "Reset", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetObjectInfo(
        &mut self,
        newObjectInfo: crate::HoudiniEngineUnity::HAPI_ObjectInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::HoudiniEngineUnity::HAPI_ObjectInfo),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetObjectInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "SetObjectInfo", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (newObjectInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SyncWithObjectInfo(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SyncWithObjectInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "SyncWithObjectInfo", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (session))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "ToString", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateObject(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        bForceUpdate: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_SessionBase,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("UpdateObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "UpdateObject", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (session, bForceUpdate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ObjectID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_ObjectID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "get_ObjectID", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ObjectName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_ObjectName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "get_ObjectName", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ParentAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_ObjectNode as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
                0usize,
            >("get_ParentAsset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_ObjectNode as
                    quest_hook::libil2cpp::Type > ::class(), "get_ParentAsset", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_HoudiniAsset,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ObjectNode")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_ObjectNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ObjectNode")]
impl AsRef<
    crate::HoudiniEngineUnity::IEquivable_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ObjectNode>,
    >,
> for crate::HoudiniEngineUnity::HEU_ObjectNode {
    fn as_ref(
        &self,
    ) -> &crate::HoudiniEngineUnity::IEquivable_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ObjectNode>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ObjectNode")]
impl AsMut<
    crate::HoudiniEngineUnity::IEquivable_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ObjectNode>,
    >,
> for crate::HoudiniEngineUnity::HEU_ObjectNode {
    fn as_mut(
        &mut self,
    ) -> &mut crate::HoudiniEngineUnity::IEquivable_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ObjectNode>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
