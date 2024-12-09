#[cfg(feature = "System+ComponentModel+CollectionChangeEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct CollectionChangeEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub _Action_k__BackingField: crate::System::ComponentModel::CollectionChangeAction,
    pub _Element_k__BackingField: *mut quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+ComponentModel+CollectionChangeEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::CollectionChangeEventArgs => "System.ComponentModel"
    ."CollectionChangeEventArgs"
);
#[cfg(feature = "System+ComponentModel+CollectionChangeEventArgs")]
impl std::ops::Deref for crate::System::ComponentModel::CollectionChangeEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+CollectionChangeEventArgs")]
impl std::ops::DerefMut for crate::System::ComponentModel::CollectionChangeEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+CollectionChangeEventArgs")]
impl crate::System::ComponentModel::CollectionChangeEventArgs {
    pub fn New(
        action: crate::System::ComponentModel::CollectionChangeAction,
        element: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (action, element))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        action: crate::System::ComponentModel::CollectionChangeAction,
        element: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (action, element))?;
        Ok(__cordl_ret)
    }
    pub fn get_Action(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ComponentModel::CollectionChangeAction,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ComponentModel::CollectionChangeAction = __cordl_object
            .invoke("get_Action", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Element(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("get_Element", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+ComponentModel+CollectionChangeEventArgs")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::CollectionChangeEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
