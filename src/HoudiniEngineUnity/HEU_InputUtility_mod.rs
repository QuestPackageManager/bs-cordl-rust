#[cfg(feature = "HoudiniEngineUnity+HEU_InputUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_InputUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputUtility")]
unsafe impl quest_hook::libil2cpp::Type for crate::HoudiniEngineUnity::HEU_InputUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_InputUtility";
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
#[cfg(feature = "HoudiniEngineUnity+HEU_InputUtility")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_InputUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputUtility")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_InputUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputUtility")]
impl crate::HoudiniEngineUnity::HEU_InputUtility {
    pub fn CreateInputNodeWithMultiAssets(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        parentAsset: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_HoudiniAsset,
        >,
        connectMergeID: quest_hook::libil2cpp::ByRefMut<i32>,
        inputAssetInfos: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_InputHDAInfo,
                    >,
                >,
            >,
        >,
        bKeepWorldTransform: bool,
        mergeParentID: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_SessionBase,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_HoudiniAsset,
                    >,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::HoudiniEngineUnity::HEU_InputHDAInfo,
                                >,
                            >,
                        >,
                    >,
                    bool,
                    i32,
                ),
                bool,
                6usize,
            >("CreateInputNodeWithMultiAssets")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateInputNodeWithMultiAssets", 6usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        session,
                        parentAsset,
                        connectMergeID,
                        inputAssetInfos,
                        bKeepWorldTransform,
                        mergeParentID,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateInputNodeWithMultiObjects(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        assetID: i32,
        connectMergeID: quest_hook::libil2cpp::ByRefMut<i32>,
        inputObjects: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_InputObjectInfo,
                    >,
                >,
            >,
        >,
        inputObjectsConnectedAssetIDs: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
        >,
        inputNode: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_InputNode>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_SessionBase,
                    >,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::HoudiniEngineUnity::HEU_InputObjectInfo,
                                >,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<i32>,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_InputNode>,
                ),
                bool,
                6usize,
            >("CreateInputNodeWithMultiObjects")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateInputNodeWithMultiObjects", 6usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        session,
                        assetID,
                        connectMergeID,
                        inputObjects,
                        inputObjectsConnectedAssetIDs,
                        inputNode,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetHighestPriority() -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), i32, 0usize>("GetHighestPriority")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetHighestPriority", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetInputInterfaceByType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_InputInterface>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_InputInterface>,
                1usize,
            >("GetInputInterfaceByType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetInputInterfaceByType", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_InputInterface,
        > = unsafe { method.invoke_unchecked((), (_cordl_type)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetInputInterface_GameObject0(
        inputObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_InputInterface>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>),
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_InputInterface>,
                1usize,
            >("GetInputInterface")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetInputInterface", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_InputInterface,
        > = unsafe { method.invoke_unchecked((), (inputObject)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetInputInterface_HEU_InputObjectInfo1(
        inputObjectInfo: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_InputObjectInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_InputInterface>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::HoudiniEngineUnity::HEU_InputObjectInfo,
                >),
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_InputInterface>,
                1usize,
            >("GetInputInterface")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetInputInterface", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_InputInterface,
        > = unsafe { method.invoke_unchecked((), (inputObjectInfo)) };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterInputInterface(
        inputInterface: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_InputInterface,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::HoudiniEngineUnity::HEU_InputInterface,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RegisterInputInterface")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterInputInterface", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (inputInterface))
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterInputInterface(
        inputInterface: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_InputInterface,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::HoudiniEngineUnity::HEU_InputInterface,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("UnregisterInputInterface")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UnregisterInputInterface", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (inputInterface))
        };
        Ok(__cordl_ret.into())
    }
    pub fn UploadInputObjectTransform(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        inputObject: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_InputObjectInfo,
        >,
        inputNodeID: i32,
        bKeepWorldTransform: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_SessionBase,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_InputObjectInfo,
                    >,
                    i32,
                    bool,
                ),
                bool,
                4usize,
            >("UploadInputObjectTransform")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UploadInputObjectTransform", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (session, inputObject, inputNodeID, bKeepWorldTransform),
                )
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_InputUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
