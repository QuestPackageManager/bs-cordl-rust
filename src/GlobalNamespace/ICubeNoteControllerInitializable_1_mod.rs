#[cfg(feature = "ICubeNoteControllerInitializable_1")]
#[repr(C)]
#[derive(Debug)]
pub struct ICubeNoteControllerInitializable_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "ICubeNoteControllerInitializable_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ICubeNoteControllerInitializable_1 < T > => ""
    ."ICubeNoteControllerInitializable`1" < T >
);
#[cfg(feature = "ICubeNoteControllerInitializable_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for ICubeNoteControllerInitializable_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ICubeNoteControllerInitializable_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for ICubeNoteControllerInitializable_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ICubeNoteControllerInitializable_1")]
impl<T: quest_hook::libil2cpp::Type> ICubeNoteControllerInitializable_1<T> {
    pub fn add_cubeNoteControllerDidInitEvent(
        &mut self,
        value: *mut crate::System::Action_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_cubeNoteControllerDidInitEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn remove_cubeNoteControllerDidInitEvent(
        &mut self,
        value: *mut crate::System::Action_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_cubeNoteControllerDidInitEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "ICubeNoteControllerInitializable_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for ICubeNoteControllerInitializable_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
