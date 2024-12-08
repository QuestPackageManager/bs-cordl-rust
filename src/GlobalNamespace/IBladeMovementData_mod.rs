#[cfg(feature = "IBladeMovementData")]
#[repr(C)]
#[derive(Debug)]
pub struct IBladeMovementData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IBladeMovementData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IBladeMovementData => ""
    ."IBladeMovementData"
);
#[cfg(feature = "IBladeMovementData")]
impl std::ops::Deref for crate::GlobalNamespace::IBladeMovementData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IBladeMovementData")]
impl std::ops::DerefMut for crate::GlobalNamespace::IBladeMovementData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IBladeMovementData")]
impl crate::GlobalNamespace::IBladeMovementData {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_bladeSpeed(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_bladeSpeed", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lastAddedData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::BladeMovementDataElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BladeMovementDataElement = __cordl_object
            .invoke("get_lastAddedData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_prevAddedData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::BladeMovementDataElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BladeMovementDataElement = __cordl_object
            .invoke("get_prevAddedData", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "IBladeMovementData")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::IBladeMovementData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
