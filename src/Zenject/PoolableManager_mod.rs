#[cfg(feature = "Zenject+PoolableManager")]
#[repr(C)]
#[derive(Debug)]
pub struct PoolableManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _poolables: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::Zenject::IPoolable>,
        >,
    >,
    pub _isSpawned: bool,
}
#[cfg(feature = "Zenject+PoolableManager")]
unsafe impl quest_hook::libil2cpp::Type for crate::Zenject::PoolableManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "PoolableManager";
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
#[cfg(feature = "Zenject+PoolableManager")]
impl std::ops::Deref for crate::Zenject::PoolableManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PoolableManager")]
impl std::ops::DerefMut for crate::Zenject::PoolableManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PoolableManager")]
impl crate::Zenject::PoolableManager {
    #[cfg(feature = "Zenject+PoolableManager+PoolableInfo")]
    pub type PoolableInfo = crate::Zenject::PoolableManager_PoolableInfo;
    pub fn CreatePoolableInfo(
        &mut self,
        poolable: quest_hook::libil2cpp::Gc<crate::Zenject::IPoolable>,
        priorities: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::ModestTree::Util::ValuePair_2<
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        i32,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::Zenject::PoolableManager_PoolableInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Zenject::PoolableManager_PoolableInfo = __cordl_object
            .invoke("CreatePoolableInfo", (poolable, priorities))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        poolables: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::IPoolable>,
            >,
        >,
        priorities: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::ModestTree::Util::ValuePair_2<
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        i32,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (poolables, priorities))?;
        Ok(__cordl_object.into())
    }
    pub fn TriggerOnDespawned(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TriggerOnDespawned", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TriggerOnSpawned(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TriggerOnSpawned", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn __zenCreate(
        P_0: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("__zenCreate", (P_0))?;
        Ok(__cordl_ret.into())
    }
    pub fn __zenCreateInjectTypeInfo() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("__zenCreateInjectTypeInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        poolables: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::IPoolable>,
            >,
        >,
        priorities: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::ModestTree::Util::ValuePair_2<
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        i32,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (poolables, priorities))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+PoolableManager")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::PoolableManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+PoolableManager+PoolableInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PoolableManager_PoolableInfo {
    pub Poolable: quest_hook::libil2cpp::Gc<crate::Zenject::IPoolable>,
    pub Priority: i32,
}
#[cfg(feature = "Zenject+PoolableManager+PoolableInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Zenject::PoolableManager_PoolableInfo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "PoolableManager/PoolableInfo";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "Zenject+PoolableManager+PoolableInfo")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Zenject::PoolableManager_PoolableInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Zenject+PoolableManager+PoolableInfo")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Zenject::PoolableManager_PoolableInfo {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "Zenject+PoolableManager+PoolableInfo")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Zenject::PoolableManager_PoolableInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "Zenject+PoolableManager+PoolableInfo")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Zenject::PoolableManager_PoolableInfo {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "Zenject+PoolableManager+PoolableInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Zenject::PoolableManager_PoolableInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Zenject+PoolableManager+PoolableInfo")]
impl crate::Zenject::PoolableManager_PoolableInfo {
    pub fn _ctor(
        &mut self,
        poolable: quest_hook::libil2cpp::Gc<crate::Zenject::IPoolable>,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (poolable, priority),
        )?;
        Ok(__cordl_ret.into())
    }
}
