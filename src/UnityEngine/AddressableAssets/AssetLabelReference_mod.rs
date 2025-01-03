#[cfg(feature = "UnityEngine+AddressableAssets+AssetLabelReference")]
#[repr(C)]
#[derive(Debug)]
pub struct AssetLabelReference {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_LabelString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "UnityEngine+AddressableAssets+AssetLabelReference")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::AssetLabelReference =>
    "UnityEngine.AddressableAssets"."AssetLabelReference"
);
#[cfg(feature = "UnityEngine+AddressableAssets+AssetLabelReference")]
impl std::ops::Deref for crate::UnityEngine::AddressableAssets::AssetLabelReference {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+AssetLabelReference")]
impl std::ops::DerefMut for crate::UnityEngine::AddressableAssets::AssetLabelReference {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+AssetLabelReference")]
impl crate::UnityEngine::AddressableAssets::AssetLabelReference {
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RuntimeKeyIsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("RuntimeKeyIsValid", ())?;
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
    pub fn get_RuntimeKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_RuntimeKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_labelString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_labelString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_labelString(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_labelString", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+AssetLabelReference")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::AssetLabelReference {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+AssetLabelReference")]
impl AsRef<crate::UnityEngine::AddressableAssets::IKeyEvaluator>
for crate::UnityEngine::AddressableAssets::AssetLabelReference {
    fn as_ref(&self) -> &crate::UnityEngine::AddressableAssets::IKeyEvaluator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+AssetLabelReference")]
impl AsMut<crate::UnityEngine::AddressableAssets::IKeyEvaluator>
for crate::UnityEngine::AddressableAssets::AssetLabelReference {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::AddressableAssets::IKeyEvaluator {
        unsafe { std::mem::transmute(self) }
    }
}
