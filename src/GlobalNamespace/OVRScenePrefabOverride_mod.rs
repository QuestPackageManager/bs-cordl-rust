#[cfg(feature = "OVRScenePrefabOverride")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRScenePrefabOverride {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Prefab: *mut crate::GlobalNamespace::OVRSceneAnchor,
    pub ClassificationLabel: *mut quest_hook::libil2cpp::Il2CppString,
    pub _editorClassificationIndex: i32,
}
#[cfg(feature = "OVRScenePrefabOverride")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRScenePrefabOverride => ""
    ."OVRScenePrefabOverride"
);
#[cfg(feature = "OVRScenePrefabOverride")]
impl std::ops::Deref for crate::GlobalNamespace::OVRScenePrefabOverride {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRScenePrefabOverride")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRScenePrefabOverride {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRScenePrefabOverride")]
impl crate::GlobalNamespace::OVRScenePrefabOverride {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UnityEngine_ISerializationCallbackReceiver_OnAfterDeserialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.ISerializationCallbackReceiver.OnAfterDeserialize",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ISerializationCallbackReceiver_OnBeforeSerialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.ISerializationCallbackReceiver.OnBeforeSerialize", ())?;
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
#[cfg(feature = "OVRScenePrefabOverride")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRScenePrefabOverride {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRScenePrefabOverride")]
impl AsRef<crate::UnityEngine::ISerializationCallbackReceiver>
for crate::GlobalNamespace::OVRScenePrefabOverride {
    fn as_ref(&self) -> &crate::UnityEngine::ISerializationCallbackReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRScenePrefabOverride")]
impl AsMut<crate::UnityEngine::ISerializationCallbackReceiver>
for crate::GlobalNamespace::OVRScenePrefabOverride {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::ISerializationCallbackReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
