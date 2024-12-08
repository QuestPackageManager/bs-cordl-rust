#[cfg(feature = "System+Collections+Specialized+NotifyCollectionChangedEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct NotifyCollectionChangedEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub _action: crate::System::Collections::Specialized::NotifyCollectionChangedAction,
    pub _newItems: *mut crate::System::Collections::IList,
    pub _oldItems: *mut crate::System::Collections::IList,
    pub _newStartingIndex: i32,
    pub _oldStartingIndex: i32,
}
#[cfg(feature = "System+Collections+Specialized+NotifyCollectionChangedEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Collections::Specialized::NotifyCollectionChangedEventArgs =>
    "System.Collections.Specialized"."NotifyCollectionChangedEventArgs"
);
#[cfg(feature = "System+Collections+Specialized+NotifyCollectionChangedEventArgs")]
impl std::ops::Deref
for crate::System::Collections::Specialized::NotifyCollectionChangedEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Specialized+NotifyCollectionChangedEventArgs")]
impl std::ops::DerefMut
for crate::System::Collections::Specialized::NotifyCollectionChangedEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Specialized+NotifyCollectionChangedEventArgs")]
impl crate::System::Collections::Specialized::NotifyCollectionChangedEventArgs {
    pub fn InitializeAdd(
        &mut self,
        action: crate::System::Collections::Specialized::NotifyCollectionChangedAction,
        newItems: *mut crate::System::Collections::IList,
        newStartingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeAdd", (action, newItems, newStartingIndex))?;
        Ok(__cordl_ret)
    }
    pub fn InitializeAddOrRemove(
        &mut self,
        action: crate::System::Collections::Specialized::NotifyCollectionChangedAction,
        changedItems: *mut crate::System::Collections::IList,
        startingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeAddOrRemove", (action, changedItems, startingIndex))?;
        Ok(__cordl_ret)
    }
    pub fn InitializeMoveOrReplace(
        &mut self,
        action: crate::System::Collections::Specialized::NotifyCollectionChangedAction,
        newItems: *mut crate::System::Collections::IList,
        oldItems: *mut crate::System::Collections::IList,
        startingIndex: i32,
        oldStartingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InitializeMoveOrReplace",
                (action, newItems, oldItems, startingIndex, oldStartingIndex),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InitializeRemove(
        &mut self,
        action: crate::System::Collections::Specialized::NotifyCollectionChangedAction,
        oldItems: *mut crate::System::Collections::IList,
        oldStartingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeRemove", (action, oldItems, oldStartingIndex))?;
        Ok(__cordl_ret)
    }
    pub fn New_IList_IList_i32_3(
        action: crate::System::Collections::Specialized::NotifyCollectionChangedAction,
        newItems: *mut crate::System::Collections::IList,
        oldItems: *mut crate::System::Collections::IList,
        startingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (action, newItems, oldItems, startingIndex))?;
        Ok(__cordl_object)
    }
    pub fn New_NotifyCollectionChangedAction0(
        action: crate::System::Collections::Specialized::NotifyCollectionChangedAction,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (action))?;
        Ok(__cordl_object)
    }
    pub fn New_Object_Object_i32_2(
        action: crate::System::Collections::Specialized::NotifyCollectionChangedAction,
        newItem: *mut crate::System::Object,
        oldItem: *mut crate::System::Object,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (action, newItem, oldItem, index))?;
        Ok(__cordl_object)
    }
    pub fn New_Object_i32_1(
        action: crate::System::Collections::Specialized::NotifyCollectionChangedAction,
        changedItem: *mut crate::System::Object,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (action, changedItem, index))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_IList_IList_i32_3(
        &mut self,
        action: crate::System::Collections::Specialized::NotifyCollectionChangedAction,
        newItems: *mut crate::System::Collections::IList,
        oldItems: *mut crate::System::Collections::IList,
        startingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (action, newItems, oldItems, startingIndex))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_NotifyCollectionChangedAction0(
        &mut self,
        action: crate::System::Collections::Specialized::NotifyCollectionChangedAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (action))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Object_Object_i32_2(
        &mut self,
        action: crate::System::Collections::Specialized::NotifyCollectionChangedAction,
        newItem: *mut crate::System::Object,
        oldItem: *mut crate::System::Object,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (action, newItem, oldItem, index))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Object_i32_1(
        &mut self,
        action: crate::System::Collections::Specialized::NotifyCollectionChangedAction,
        changedItem: *mut crate::System::Object,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (action, changedItem, index))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Collections+Specialized+NotifyCollectionChangedEventArgs")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Collections::Specialized::NotifyCollectionChangedEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
