#[cfg(feature = "Zenject+TickableManager")]
#[repr(C)]
#[derive(Debug)]
pub struct TickableManager {
    __cordl_parent: crate::System::Object,
    pub _tickables: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Zenject::ITickable,
    >,
    pub _fixedTickables: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Zenject::IFixedTickable,
    >,
    pub _lateTickables: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Zenject::ILateTickable,
    >,
    pub _priorities: *mut crate::System::Collections::Generic::List_1<
        *mut crate::ModestTree::Util::ValuePair_2<*mut crate::System::Type, i32>,
    >,
    pub _fixedPriorities: *mut crate::System::Collections::Generic::List_1<
        *mut crate::ModestTree::Util::ValuePair_2<*mut crate::System::Type, i32>,
    >,
    pub _latePriorities: *mut crate::System::Collections::Generic::List_1<
        *mut crate::ModestTree::Util::ValuePair_2<*mut crate::System::Type, i32>,
    >,
    pub _updater: *mut crate::Zenject::TickablesTaskUpdater,
    pub _fixedUpdater: *mut crate::Zenject::FixedTickablesTaskUpdater,
    pub _lateUpdater: *mut crate::Zenject::LateTickablesTaskUpdater,
    pub _isPaused: bool,
}
#[cfg(feature = "Zenject+TickableManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::TickableManager => "Zenject"
    ."TickableManager"
);
#[cfg(feature = "Zenject+TickableManager")]
impl std::ops::Deref for crate::Zenject::TickableManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+TickableManager")]
impl std::ops::DerefMut for crate::Zenject::TickableManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+TickableManager")]
impl crate::Zenject::TickableManager {
    #[cfg(feature = "Zenject+TickableManager+__c")]
    pub type __c = crate::Zenject::TickableManager___c;
    #[cfg(feature = "Zenject+TickableManager+__c__DisplayClass17_0")]
    pub type __c__DisplayClass17_0 = crate::Zenject::TickableManager___c__DisplayClass17_0;
    #[cfg(feature = "Zenject+TickableManager+__c__DisplayClass18_0")]
    pub type __c__DisplayClass18_0 = crate::Zenject::TickableManager___c__DisplayClass18_0;
    #[cfg(feature = "Zenject+TickableManager+__c__DisplayClass19_0")]
    pub type __c__DisplayClass19_0 = crate::Zenject::TickableManager___c__DisplayClass19_0;
    pub fn AddFixed_IFixedTickable1(
        &mut self,
        tickable: *mut crate::Zenject::IFixedTickable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddFixed", (tickable))?;
        Ok(__cordl_ret)
    }
    pub fn AddFixed_i32_0(
        &mut self,
        tickable: *mut crate::Zenject::IFixedTickable,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddFixed", (tickable, priority))?;
        Ok(__cordl_ret)
    }
    pub fn AddLate_ILateTickable1(
        &mut self,
        tickable: *mut crate::Zenject::ILateTickable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddLate", (tickable))?;
        Ok(__cordl_ret)
    }
    pub fn AddLate_i32_0(
        &mut self,
        tickable: *mut crate::Zenject::ILateTickable,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddLate", (tickable, priority))?;
        Ok(__cordl_ret)
    }
    pub fn Add_ITickable1(
        &mut self,
        tickable: *mut crate::Zenject::ITickable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (tickable))?;
        Ok(__cordl_ret)
    }
    pub fn Add_i32_0(
        &mut self,
        tickable: *mut crate::Zenject::ITickable,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (tickable, priority))?;
        Ok(__cordl_ret)
    }
    pub fn FixedUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FixedUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn InitFixedTickables(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitFixedTickables", ())?;
        Ok(__cordl_ret)
    }
    pub fn InitLateTickables(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitLateTickables", ())?;
        Ok(__cordl_ret)
    }
    pub fn InitTickables(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitTickables", ())?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Remove(
        &mut self,
        tickable: *mut crate::Zenject::ITickable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (tickable))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveFixed(
        &mut self,
        tickable: *mut crate::Zenject::IFixedTickable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveFixed", (tickable))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveLate(
        &mut self,
        tickable: *mut crate::Zenject::ILateTickable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveLate", (tickable))?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
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
    pub fn get_IsPaused(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsPaused", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Tickables(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::ITickable,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::ITickable,
        > = __cordl_object.invoke("get_Tickables", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_IsPaused(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsPaused", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+TickableManager")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::TickableManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
