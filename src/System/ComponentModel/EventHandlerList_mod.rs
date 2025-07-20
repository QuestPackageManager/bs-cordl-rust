#[cfg(feature = "System+ComponentModel+EventHandlerList")]
#[repr(C)]
#[derive(Debug)]
pub struct EventHandlerList {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _head: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::EventHandlerList_ListEntry,
    >,
    pub _parent: quest_hook::libil2cpp::Gc<crate::System::ComponentModel::Component>,
}
#[cfg(feature = "System+ComponentModel+EventHandlerList")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::ComponentModel::EventHandlerList {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.ComponentModel";
    const CLASS_NAME: &'static str = "EventHandlerList";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+ComponentModel+EventHandlerList")]
impl std::ops::Deref for crate::System::ComponentModel::EventHandlerList {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+EventHandlerList")]
impl std::ops::DerefMut for crate::System::ComponentModel::EventHandlerList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+EventHandlerList")]
impl crate::System::ComponentModel::EventHandlerList {
    #[cfg(feature = "System+ComponentModel+EventHandlerList+ListEntry")]
    pub type ListEntry = crate::System::ComponentModel::EventHandlerList_ListEntry;
    pub fn Find(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventHandlerList_ListEntry,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::EventHandlerList as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::EventHandlerList_ListEntry,
                >,
                1usize,
            >("Find")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::EventHandlerList as
                    quest_hook::libil2cpp::Type > ::class(), "Find", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventHandlerList_ListEntry,
        > = unsafe { method.invoke_unchecked(self, (key))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::EventHandlerList as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<crate::System::Delegate>,
                1usize,
            >("get_Item")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::EventHandlerList as
                    quest_hook::libil2cpp::Type > ::class(), "get_Item", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Delegate> = unsafe {
            method.invoke_unchecked(self, (key))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ComponentModel+EventHandlerList")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::EventHandlerList {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+ComponentModel+EventHandlerList+ListEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct EventHandlerList_ListEntry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _next: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::EventHandlerList_ListEntry,
    >,
    pub _key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _handler: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
}
#[cfg(feature = "System+ComponentModel+EventHandlerList+ListEntry")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::ComponentModel::EventHandlerList_ListEntry {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.ComponentModel";
    const CLASS_NAME: &'static str = "EventHandlerList/ListEntry";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+ComponentModel+EventHandlerList+ListEntry")]
impl std::ops::Deref for crate::System::ComponentModel::EventHandlerList_ListEntry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+EventHandlerList+ListEntry")]
impl std::ops::DerefMut for crate::System::ComponentModel::EventHandlerList_ListEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+EventHandlerList+ListEntry")]
impl crate::System::ComponentModel::EventHandlerList_ListEntry {}
#[cfg(feature = "System+ComponentModel+EventHandlerList+ListEntry")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::EventHandlerList_ListEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
