#[cfg(feature = "IMultiplayerSpectatingSpot")]
#[repr(C)]
#[derive(Debug)]
pub struct IMultiplayerSpectatingSpot {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IMultiplayerSpectatingSpot")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for IMultiplayerSpectatingSpot => ""
    ."IMultiplayerSpectatingSpot"
);
#[cfg(feature = "IMultiplayerSpectatingSpot")]
impl std::ops::Deref for IMultiplayerSpectatingSpot {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IMultiplayerSpectatingSpot")]
impl std::ops::DerefMut for IMultiplayerSpectatingSpot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IMultiplayerSpectatingSpot")]
impl IMultiplayerSpectatingSpot {
    pub fn get_observable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut IMultiplayerObservable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut IMultiplayerObservable = __cordl_object
            .invoke("get_observable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_transform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_transform", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_hasBeenRemovedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut IMultiplayerSpectatingSpot>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_hasBeenRemovedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_spotName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_spotName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isMain(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isMain", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetIsObserved(
        &mut self,
        isObserved: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIsObserved", (isObserved))?;
        Ok(__cordl_ret)
    }
    pub fn add_hasBeenRemovedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut IMultiplayerSpectatingSpot>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_hasBeenRemovedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IMultiplayerSpectatingSpot")]
impl quest_hook::libil2cpp::ObjectType for IMultiplayerSpectatingSpot {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
