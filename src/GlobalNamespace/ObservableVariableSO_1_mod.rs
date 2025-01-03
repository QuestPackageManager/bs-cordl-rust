#[cfg(feature = "ObservableVariableSO_1")]
#[repr(C)]
#[derive(Debug)]
pub struct ObservableVariableSO_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub didChangeEvent: *mut crate::System::Action,
    pub _value: T,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "ObservableVariableSO_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ObservableVariableSO_1 < T > =>
    ""."ObservableVariableSO`1" < T >
);
#[cfg(feature = "ObservableVariableSO_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::GlobalNamespace::ObservableVariableSO_1<T> {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ObservableVariableSO_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::GlobalNamespace::ObservableVariableSO_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ObservableVariableSO_1")]
impl<T: quest_hook::libil2cpp::Type> crate::GlobalNamespace::ObservableVariableSO_1<T> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_value(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("get_value", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        obj: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ObservableVariableSO_1<T>>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_value(
        &mut self,
        value: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_value", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ObservableVariableSO_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ObservableVariableSO_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ObservableVariableSO_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::GlobalNamespace::IObservableChange>
for crate::GlobalNamespace::ObservableVariableSO_1<T> {
    fn as_ref(&self) -> &crate::GlobalNamespace::IObservableChange {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ObservableVariableSO_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::GlobalNamespace::IObservableChange>
for crate::GlobalNamespace::ObservableVariableSO_1<T> {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IObservableChange {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ObservableVariableSO_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::GlobalNamespace::IValue_1<T>>
for crate::GlobalNamespace::ObservableVariableSO_1<T> {
    fn as_ref(&self) -> &crate::GlobalNamespace::IValue_1<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ObservableVariableSO_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::GlobalNamespace::IValue_1<T>>
for crate::GlobalNamespace::ObservableVariableSO_1<T> {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IValue_1<T> {
        unsafe { std::mem::transmute(self) }
    }
}
