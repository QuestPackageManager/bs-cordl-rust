#[cfg(feature = "HEU_ScriptMeshInputUVLayoutExample")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_ScriptMeshInputUVLayoutExample {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HEU_ScriptMeshInputUVLayoutExample")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::HEU_ScriptMeshInputUVLayoutExample => ""
    ."HEU_ScriptMeshInputUVLayoutExample"
);
#[cfg(feature = "HEU_ScriptMeshInputUVLayoutExample")]
impl std::ops::Deref for crate::GlobalNamespace::HEU_ScriptMeshInputUVLayoutExample {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HEU_ScriptMeshInputUVLayoutExample")]
impl std::ops::DerefMut for crate::GlobalNamespace::HEU_ScriptMeshInputUVLayoutExample {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HEU_ScriptMeshInputUVLayoutExample")]
impl crate::GlobalNamespace::HEU_ScriptMeshInputUVLayoutExample {
    #[cfg(feature = "HEU_ScriptMeshInputUVLayoutExample+OutputMode")]
    pub type OutputMode = crate::GlobalNamespace::HEU_ScriptMeshInputUVLayoutExample_OutputMode;
    pub fn ApplyUVLayoutTo(
        gameObjects: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::GameObject>,
        >,
        outputMode: crate::GlobalNamespace::HEU_ScriptMeshInputUVLayoutExample_OutputMode,
        output_name_suffix: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyUVLayoutTo", (gameObjects, outputMode, output_name_suffix))?;
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
}
#[cfg(feature = "HEU_ScriptMeshInputUVLayoutExample")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::HEU_ScriptMeshInputUVLayoutExample {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HEU_ScriptMeshInputUVLayoutExample+OutputMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HEU_ScriptMeshInputUVLayoutExample_OutputMode {
    COPY = 0i32,
    REPLACE = 1i32,
}
#[cfg(feature = "HEU_ScriptMeshInputUVLayoutExample+OutputMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::HEU_ScriptMeshInputUVLayoutExample_OutputMode => ""
    ."HEU_ScriptMeshInputUVLayoutExample/OutputMode"
);
