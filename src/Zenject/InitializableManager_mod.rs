#[cfg(feature = "Zenject+InitializableManager+InitializableInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct InitializableManager_InitializableInfo {
    __cordl_parent: crate::System::Object,
    pub Initializable: *mut crate::Zenject::IInitializable,
    pub Priority: i32,
}
#[cfg(feature = "Zenject+InitializableManager+InitializableInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::InitializableManager_InitializableInfo
    => "Zenject"."InitializableManager/InitializableInfo"
);
#[cfg(feature = "Zenject+InitializableManager+InitializableInfo")]
impl std::ops::Deref for crate::Zenject::InitializableManager_InitializableInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+InitializableManager+InitializableInfo")]
impl std::ops::DerefMut for crate::Zenject::InitializableManager_InitializableInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+InitializableManager+InitializableInfo")]
impl crate::Zenject::InitializableManager_InitializableInfo {
    pub fn _ctor(
        &mut self,
        initializable: *mut crate::Zenject::IInitializable,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initializable, priority))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        initializable: *mut crate::Zenject::IInitializable,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initializable, priority))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Zenject+InitializableManager+InitializableInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::InitializableManager_InitializableInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+InitializableManager")]
#[repr(C)]
#[derive(Debug)]
pub struct InitializableManager {
    __cordl_parent: crate::System::Object,
    pub _initializables: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Zenject::InitializableManager_InitializableInfo,
    >,
    pub _hasInitialized: bool,
}
#[cfg(feature = "Zenject+InitializableManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::InitializableManager => "Zenject"
    ."InitializableManager"
);
#[cfg(feature = "Zenject+InitializableManager")]
impl std::ops::Deref for crate::Zenject::InitializableManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+InitializableManager")]
impl std::ops::DerefMut for crate::Zenject::InitializableManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+InitializableManager")]
impl crate::Zenject::InitializableManager {
    #[cfg(feature = "Zenject+InitializableManager+__c")]
    pub type __c = crate::Zenject::InitializableManager___c;
    #[cfg(feature = "Zenject+InitializableManager+__c__DisplayClass2_0")]
    pub type __c__DisplayClass2_0 = crate::Zenject::InitializableManager___c__DisplayClass2_0;
    #[cfg(feature = "Zenject+InitializableManager+InitializableInfo")]
    pub type InitializableInfo = crate::Zenject::InitializableManager_InitializableInfo;
    pub fn Add_IInitializable0(
        &mut self,
        initializable: *mut crate::Zenject::IInitializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (initializable))?;
        Ok(__cordl_ret)
    }
    pub fn Add_i32_1(
        &mut self,
        initializable: *mut crate::Zenject::IInitializable,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (initializable, priority))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        initializables: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::IInitializable,
        >,
        priorities: *mut crate::System::Collections::Generic::List_1<
            *mut crate::ModestTree::Util::ValuePair_2<*mut crate::System::Type, i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initializables, priorities))?;
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
    pub fn New(
        initializables: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::IInitializable,
        >,
        priorities: *mut crate::System::Collections::Generic::List_1<
            *mut crate::ModestTree::Util::ValuePair_2<*mut crate::System::Type, i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initializables, priorities))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Zenject+InitializableManager")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::InitializableManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
