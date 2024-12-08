#[cfg(feature = "ObstacleControllerBase")]
#[repr(C)]
#[derive(Debug)]
pub struct ObstacleControllerBase {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub didInitEvent: *mut crate::System::Action_1<*mut ObstacleControllerBase>,
    pub didStartDissolvingEvent: *mut crate::System::Action_2<
        *mut ObstacleControllerBase,
        f32,
    >,
}
#[cfg(feature = "ObstacleControllerBase")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ObstacleControllerBase => ""."ObstacleControllerBase"
);
#[cfg(feature = "ObstacleControllerBase")]
impl std::ops::Deref for ObstacleControllerBase {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ObstacleControllerBase")]
impl std::ops::DerefMut for ObstacleControllerBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ObstacleControllerBase")]
impl ObstacleControllerBase {
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
    pub fn InvokeDidStartDissolvingEvent(
        &mut self,
        obstacleController: *mut ObstacleControllerBase,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeDidStartDissolvingEvent", (obstacleController, duration))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeDidInitEvent(
        &mut self,
        obstacleController: *mut ObstacleControllerBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeDidInitEvent", (obstacleController))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didInitEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ObstacleControllerBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didInitEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didInitEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ObstacleControllerBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didInitEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didStartDissolvingEvent(
        &mut self,
        value: *mut crate::System::Action_2<*mut ObstacleControllerBase, f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didStartDissolvingEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didStartDissolvingEvent(
        &mut self,
        value: *mut crate::System::Action_2<*mut ObstacleControllerBase, f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didStartDissolvingEvent", (value))?;
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
#[cfg(feature = "ObstacleControllerBase")]
impl quest_hook::libil2cpp::ObjectType for ObstacleControllerBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
