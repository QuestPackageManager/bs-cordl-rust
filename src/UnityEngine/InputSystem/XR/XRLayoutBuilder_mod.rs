#[cfg(feature = "UnityEngine+InputSystem+XR+XRLayoutBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct XRLayoutBuilder {
    __cordl_parent: crate::System::Object,
    pub parentLayout: *mut crate::System::String,
    pub interfaceName: *mut crate::System::String,
    pub descriptor: *mut crate::UnityEngine::InputSystem::XR::XRDeviceDescriptor,
}
#[cfg(feature = "UnityEngine+InputSystem+XR+XRLayoutBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::XR::XRLayoutBuilder =>
    "UnityEngine.InputSystem.XR"."XRLayoutBuilder"
);
#[cfg(feature = "UnityEngine+InputSystem+XR+XRLayoutBuilder")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::XR::XRLayoutBuilder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+XRLayoutBuilder")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::XR::XRLayoutBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+XRLayoutBuilder")]
impl crate::UnityEngine::InputSystem::XR::XRLayoutBuilder {
    #[cfg(feature = "UnityEngine+InputSystem+XR+XRLayoutBuilder+__c__DisplayClass5_0")]
    pub type __c__DisplayClass5_0 = crate::UnityEngine::InputSystem::XR::XRLayoutBuilder___c__DisplayClass5_0;
    pub fn Build(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Layouts::InputControlLayout = __cordl_object
            .invoke("Build", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetParentControlName(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetParentControlName", (name))?;
        Ok(__cordl_ret)
    }
    pub fn IsPoseControl(
        &mut self,
        features: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::InputSystem::XR::XRFeatureDescriptor,
        >,
        startIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsPoseControl", (features, startIndex))?;
        Ok(__cordl_ret)
    }
    pub fn IsSubControl(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSubControl", (name))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "UnityEngine+InputSystem+XR+XRLayoutBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::XR::XRLayoutBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
