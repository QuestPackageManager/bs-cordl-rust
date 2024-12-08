#[cfg(feature = "Zenject+IAnimatorMoveHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct IAnimatorMoveHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+IAnimatorMoveHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::IAnimatorMoveHandler => "Zenject"
    ."IAnimatorMoveHandler"
);
#[cfg(feature = "Zenject+IAnimatorMoveHandler")]
impl std::ops::Deref for crate::Zenject::IAnimatorMoveHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+IAnimatorMoveHandler")]
impl std::ops::DerefMut for crate::Zenject::IAnimatorMoveHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+IAnimatorMoveHandler")]
impl crate::Zenject::IAnimatorMoveHandler {
    pub fn OnAnimatorMove(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnAnimatorMove", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Zenject+IAnimatorMoveHandler")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::IAnimatorMoveHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
