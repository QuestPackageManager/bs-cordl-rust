#[cfg(feature = "Zenject+DisposableManager")]
#[repr(C)]
#[derive(Debug)]
pub struct DisposableManager {
    __cordl_parent: crate::System::Object,
    pub _disposables: *mut crate::System::Collections::Generic::List_1<
        crate::Zenject::DisposableManager_DisposableInfo,
    >,
    pub _lateDisposables: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Zenject::DisposableManager_LateDisposableInfo,
    >,
    pub _disposed: bool,
    pub _lateDisposed: bool,
}
#[cfg(feature = "Zenject+DisposableManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::DisposableManager => "Zenject"
    ."DisposableManager"
);
#[cfg(feature = "Zenject+DisposableManager")]
impl std::ops::Deref for crate::Zenject::DisposableManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+DisposableManager")]
impl std::ops::DerefMut for crate::Zenject::DisposableManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+DisposableManager")]
impl crate::Zenject::DisposableManager {
    #[cfg(feature = "Zenject+DisposableManager+DisposableInfo")]
    pub type DisposableInfo = crate::Zenject::DisposableManager_DisposableInfo;
    #[cfg(feature = "Zenject+DisposableManager+LateDisposableInfo")]
    pub type LateDisposableInfo = crate::Zenject::DisposableManager_LateDisposableInfo;
    #[cfg(feature = "Zenject+DisposableManager+__c")]
    pub type __c = crate::Zenject::DisposableManager___c;
    #[cfg(feature = "Zenject+DisposableManager+__c__DisplayClass4_0")]
    pub type __c__DisplayClass4_0 = crate::Zenject::DisposableManager___c__DisplayClass4_0;
    #[cfg(feature = "Zenject+DisposableManager+__c__DisplayClass4_1")]
    pub type __c__DisplayClass4_1 = crate::Zenject::DisposableManager___c__DisplayClass4_1;
    #[cfg(feature = "Zenject+DisposableManager+__c__DisplayClass9_0")]
    pub type __c__DisplayClass9_0 = crate::Zenject::DisposableManager___c__DisplayClass9_0;
    pub fn AddLate_ILateDisposable0(
        &mut self,
        disposable: *mut crate::Zenject::ILateDisposable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddLate", (disposable))?;
        Ok(__cordl_ret)
    }
    pub fn AddLate_i32_1(
        &mut self,
        disposable: *mut crate::Zenject::ILateDisposable,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddLate", (disposable, priority))?;
        Ok(__cordl_ret)
    }
    pub fn Add_IDisposable0(
        &mut self,
        disposable: *mut crate::System::IDisposable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (disposable))?;
        Ok(__cordl_ret)
    }
    pub fn Add_i32_1(
        &mut self,
        disposable: *mut crate::System::IDisposable,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (disposable, priority))?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn LateDispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateDispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        disposables: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::IDisposable,
        >,
        priorities: *mut crate::System::Collections::Generic::List_1<
            *mut crate::ModestTree::Util::ValuePair_2<*mut crate::System::Type, i32>,
        >,
        lateDisposables: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::ILateDisposable,
        >,
        latePriorities: *mut crate::System::Collections::Generic::List_1<
            *mut crate::ModestTree::Util::ValuePair_2<*mut crate::System::Type, i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (disposables, priorities, lateDisposables, latePriorities),
            )?;
        Ok(__cordl_object)
    }
    pub fn Remove(
        &mut self,
        disposable: *mut crate::System::IDisposable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (disposable))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        disposables: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::IDisposable,
        >,
        priorities: *mut crate::System::Collections::Generic::List_1<
            *mut crate::ModestTree::Util::ValuePair_2<*mut crate::System::Type, i32>,
        >,
        lateDisposables: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::ILateDisposable,
        >,
        latePriorities: *mut crate::System::Collections::Generic::List_1<
            *mut crate::ModestTree::Util::ValuePair_2<*mut crate::System::Type, i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (disposables, priorities, lateDisposables, latePriorities),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+DisposableManager")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::DisposableManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+DisposableManager+DisposableInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DisposableManager_DisposableInfo {
    pub Disposable: *mut crate::System::IDisposable,
    pub Priority: i32,
}
#[cfg(feature = "Zenject+DisposableManager+DisposableInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Zenject::DisposableManager_DisposableInfo =>
    "Zenject"."DisposableManager/DisposableInfo"
);
#[cfg(feature = "Zenject+DisposableManager+DisposableInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Zenject::DisposableManager_DisposableInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Zenject+DisposableManager+DisposableInfo")]
impl crate::Zenject::DisposableManager_DisposableInfo {
    pub fn _ctor(
        &mut self,
        disposable: *mut crate::System::IDisposable,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (disposable, priority),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+DisposableManager+LateDisposableInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct DisposableManager_LateDisposableInfo {
    __cordl_parent: crate::System::Object,
    pub LateDisposable: *mut crate::Zenject::ILateDisposable,
    pub Priority: i32,
}
#[cfg(feature = "Zenject+DisposableManager+LateDisposableInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::DisposableManager_LateDisposableInfo =>
    "Zenject"."DisposableManager/LateDisposableInfo"
);
#[cfg(feature = "Zenject+DisposableManager+LateDisposableInfo")]
impl std::ops::Deref for crate::Zenject::DisposableManager_LateDisposableInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+DisposableManager+LateDisposableInfo")]
impl std::ops::DerefMut for crate::Zenject::DisposableManager_LateDisposableInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+DisposableManager+LateDisposableInfo")]
impl crate::Zenject::DisposableManager_LateDisposableInfo {
    pub fn New(
        lateDisposable: *mut crate::Zenject::ILateDisposable,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lateDisposable, priority))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        lateDisposable: *mut crate::Zenject::ILateDisposable,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lateDisposable, priority))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+DisposableManager+LateDisposableInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::DisposableManager_LateDisposableInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
