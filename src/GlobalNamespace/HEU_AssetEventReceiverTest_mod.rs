#[cfg(feature = "HEU_AssetEventReceiverTest")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_AssetEventReceiverTest {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
}
#[cfg(feature = "HEU_AssetEventReceiverTest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for HEU_AssetEventReceiverTest => ""
    ."HEU_AssetEventReceiverTest"
);
#[cfg(feature = "HEU_AssetEventReceiverTest")]
impl std::ops::Deref for HEU_AssetEventReceiverTest {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HEU_AssetEventReceiverTest")]
impl std::ops::DerefMut for HEU_AssetEventReceiverTest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HEU_AssetEventReceiverTest")]
impl HEU_AssetEventReceiverTest {
    pub fn BakedCallback(
        &mut self,
        asset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
        success: bool,
        outputList: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::GameObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BakedCallback", (asset, success, outputList))?;
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
    pub fn ReloadCallback(
        &mut self,
        asset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
        success: bool,
        outputList: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::GameObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReloadCallback", (asset, success, outputList))?;
        Ok(__cordl_ret)
    }
    pub fn CookedCallback(
        &mut self,
        asset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
        success: bool,
        outputList: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::GameObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CookedCallback", (asset, success, outputList))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "HEU_AssetEventReceiverTest")]
impl quest_hook::libil2cpp::ObjectType for HEU_AssetEventReceiverTest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
