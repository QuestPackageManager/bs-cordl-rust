#[cfg(feature = "HoudiniEngineUnity+HEU_GeneratedOutput")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_GeneratedOutput {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _outputData: *mut crate::HoudiniEngineUnity::HEU_GeneratedOutputData,
    pub _childOutputs: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_GeneratedOutputData,
    >,
    pub isInstancer: bool,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_GeneratedOutput")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_GeneratedOutput =>
    "HoudiniEngineUnity"."HEU_GeneratedOutput"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_GeneratedOutput")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_GeneratedOutput {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_GeneratedOutput")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_GeneratedOutput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_GeneratedOutput")]
impl crate::HoudiniEngineUnity::HEU_GeneratedOutput {
    pub fn BakeGameObjectComponents(
        sourceGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        targetGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        assetName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        outputPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bIsInstancer: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "BakeGameObjectComponents",
                (sourceGO, targetGO, assetName, outputPath, bIsInstancer),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearGeneratedMaterialReferences(
        generatedOutputData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GeneratedOutputData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearGeneratedMaterialReferences", (generatedOutputData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearMaterialsNoLongerUsed(
        materialsToCheck: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Material>,
        >,
        materialsInUse: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Material>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearMaterialsNoLongerUsed", (materialsToCheck, materialsInUse))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyMaterialOverrides(
        sourceOutputData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GeneratedOutputData,
        >,
        destOutputData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GeneratedOutputData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CopyMaterialOverrides", (sourceOutputData, destOutputData))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyAllGeneratedColliders(
        outputData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GeneratedOutputData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DestroyAllGeneratedColliders", (outputData))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyGeneratedOutput(
        generatedOutput: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GeneratedOutput,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DestroyGeneratedOutput", (generatedOutput))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyGeneratedOutputChildren(
        generatedOutput: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GeneratedOutput,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DestroyGeneratedOutputChildren", (generatedOutput))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyGeneratedOutputData(
        generatedOutputData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GeneratedOutputData,
        >,
        bDontDeletePersistantResources: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DestroyGeneratedOutputData",
                (generatedOutputData, bDontDeletePersistantResources),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGeneratedMaterialsForGameObject(
        output: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GeneratedOutput,
        >,
        inGameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Material>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Material>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGeneratedMaterialsForGameObject", (output, inGameObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasLODGroup(
        output: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_GeneratedOutput>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasLODGroup", (output))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEquivalentTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_GeneratedOutput>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEquivalentTo", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsOutputDataUsingMaterial(
        checkMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        outputData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GeneratedOutputData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsOutputDataUsingMaterial", (checkMaterial, outputData))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsOutputUsingMaterial(
        checkMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        output: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_GeneratedOutput>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsOutputUsingMaterial", (checkMaterial, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ResetMaterialOverrides_HEU_GeneratedOutput0(
        output: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_GeneratedOutput>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResetMaterialOverrides", (output))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetMaterialOverrides_HEU_GeneratedOutputData1(
        outputData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GeneratedOutputData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResetMaterialOverrides", (outputData))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteOutputToAssetCache(
        &mut self,
        parentObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        outputPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bIsInstancer: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WriteOutputToAssetCache",
                (parentObject, outputPath, bIsInstancer),
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
    pub fn get_IsInstancer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsInstancer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IsInstancer(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsInstancer", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_GeneratedOutput")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_GeneratedOutput {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_GeneratedOutput")]
impl AsRef<
    crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::HEU_GeneratedOutput,
    >,
> for crate::HoudiniEngineUnity::HEU_GeneratedOutput {
    fn as_ref(
        &self,
    ) -> &crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::HEU_GeneratedOutput,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_GeneratedOutput")]
impl AsMut<
    crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::HEU_GeneratedOutput,
    >,
> for crate::HoudiniEngineUnity::HEU_GeneratedOutput {
    fn as_mut(
        &mut self,
    ) -> &mut crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::HEU_GeneratedOutput,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
