#[cfg(feature = "JumpMarker")]
#[repr(C)]
#[derive(Debug)]
pub struct JumpMarker {
    __cordl_parent: crate::UnityEngine::Timeline::Marker,
    pub _destination: *mut JumpDestinationMarker,
    pub _id_k__BackingField: crate::UnityEngine::PropertyName,
}
#[cfg(feature = "JumpMarker")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for JumpMarker => ""."JumpMarker"
);
#[cfg(feature = "JumpMarker")]
impl std::ops::Deref for JumpMarker {
    type Target = crate::UnityEngine::Timeline::Marker;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "JumpMarker")]
impl std::ops::DerefMut for JumpMarker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "JumpMarker")]
impl JumpMarker {
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
    pub fn get_jumpDestination(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut JumpDestinationMarker> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut JumpDestinationMarker = __cordl_object
            .invoke("get_jumpDestination", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_id(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::PropertyName> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::PropertyName = __cordl_object
            .invoke("get_id", ())?;
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
#[cfg(feature = "JumpMarker")]
impl quest_hook::libil2cpp::ObjectType for JumpMarker {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
