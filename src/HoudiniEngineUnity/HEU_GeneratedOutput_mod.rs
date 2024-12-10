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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
