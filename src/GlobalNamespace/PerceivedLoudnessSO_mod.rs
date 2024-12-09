#[cfg(feature = "PerceivedLoudnessSO")]
#[repr(C)]
#[derive(Debug)]
pub struct PerceivedLoudnessSO {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _perceivedLoudnessPerLevel: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::PerceivedLoudnessSO_PerceivedLevelLoudnessPair,
    >,
}
#[cfg(feature = "PerceivedLoudnessSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PerceivedLoudnessSO => ""
    ."PerceivedLoudnessSO"
);
#[cfg(feature = "PerceivedLoudnessSO")]
impl std::ops::Deref for crate::GlobalNamespace::PerceivedLoudnessSO {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PerceivedLoudnessSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::PerceivedLoudnessSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PerceivedLoudnessSO")]
impl crate::GlobalNamespace::PerceivedLoudnessSO {
    #[cfg(feature = "PerceivedLoudnessSO+PerceivedLevelLoudnessPair")]
    pub type PerceivedLevelLoudnessPair = crate::GlobalNamespace::PerceivedLoudnessSO_PerceivedLevelLoudnessPair;
    #[cfg(feature = "PerceivedLoudnessSO+__c__DisplayClass6_0")]
    pub type __c__DisplayClass6_0 = crate::GlobalNamespace::PerceivedLoudnessSO___c__DisplayClass6_0;
    pub fn GetLoudnessData(
        &mut self,
        levelId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<f32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<f32> = __cordl_object
            .invoke("GetLoudnessData", (levelId))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetLoudnessData(
        &mut self,
        loudnessPairs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::PerceivedLoudnessSO_PerceivedLevelLoudnessPair,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLoudnessData", (loudnessPairs))?;
        Ok(__cordl_ret)
    }
    pub fn ToDictionary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::GlobalNamespace::PerceivedLoudnessSO_PerceivedLevelLoudnessPair,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::GlobalNamespace::PerceivedLoudnessSO_PerceivedLevelLoudnessPair,
        > = __cordl_object.invoke("ToDictionary", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_perceivedLoudnessPairs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::PerceivedLoudnessSO_PerceivedLevelLoudnessPair,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::PerceivedLoudnessSO_PerceivedLevelLoudnessPair,
        > = __cordl_object.invoke("get_perceivedLoudnessPairs", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PerceivedLoudnessSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PerceivedLoudnessSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PerceivedLoudnessSO+PerceivedLevelLoudnessPair")]
#[repr(C)]
#[derive(Debug)]
pub struct PerceivedLoudnessSO_PerceivedLevelLoudnessPair {
    __cordl_parent: crate::System::Object,
    pub _levelId: *mut crate::System::String,
    pub _perceivedLoudness: f32,
    pub _checkSum: *mut crate::System::String,
}
#[cfg(feature = "PerceivedLoudnessSO+PerceivedLevelLoudnessPair")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PerceivedLoudnessSO_PerceivedLevelLoudnessPair => ""
    ."PerceivedLoudnessSO/PerceivedLevelLoudnessPair"
);
#[cfg(feature = "PerceivedLoudnessSO+PerceivedLevelLoudnessPair")]
impl std::ops::Deref
for crate::GlobalNamespace::PerceivedLoudnessSO_PerceivedLevelLoudnessPair {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PerceivedLoudnessSO+PerceivedLevelLoudnessPair")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PerceivedLoudnessSO_PerceivedLevelLoudnessPair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PerceivedLoudnessSO+PerceivedLevelLoudnessPair")]
impl crate::GlobalNamespace::PerceivedLoudnessSO_PerceivedLevelLoudnessPair {
    pub fn New(
        levelId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (levelId))?;
        Ok(__cordl_object)
    }
    pub fn SetLoudness(
        &mut self,
        newPerceivedLoudness: f32,
        newChecksum: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLoudness", (newPerceivedLoudness, newChecksum))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        levelId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (levelId))?;
        Ok(__cordl_ret)
    }
    pub fn get_checksum(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_checksum", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_levelId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_levelId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_perceivedLoudness(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_perceivedLoudness", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PerceivedLoudnessSO+PerceivedLevelLoudnessPair")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PerceivedLoudnessSO_PerceivedLevelLoudnessPair {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
