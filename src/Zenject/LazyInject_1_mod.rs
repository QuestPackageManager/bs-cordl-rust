#[cfg(feature = "Zenject+LazyInject_1")]
#[repr(C)]
#[derive(Debug)]
pub struct LazyInject_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    pub _context: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
    pub _hasValue: bool,
    pub _value: T,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Zenject+LazyInject_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::LazyInject_1 < T > => "Zenject"
    ."LazyInject`1" < T >
);
#[cfg(feature = "Zenject+LazyInject_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Zenject::LazyInject_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+LazyInject_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Zenject::LazyInject_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+LazyInject_1")]
impl<T: quest_hook::libil2cpp::Type> crate::Zenject::LazyInject_1<T> {
    pub fn New(
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        context: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (container, context))?;
        Ok(__cordl_object.into())
    }
    pub fn Zenject_IValidatable_Validate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Zenject.IValidatable.Validate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        context: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (container, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("get_Value", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+LazyInject_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Zenject::LazyInject_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+LazyInject_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::Zenject::IValidatable>
for crate::Zenject::LazyInject_1<T> {
    fn as_ref(&self) -> &crate::Zenject::IValidatable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+LazyInject_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::Zenject::IValidatable>
for crate::Zenject::LazyInject_1<T> {
    fn as_mut(&mut self) -> &mut crate::Zenject::IValidatable {
        unsafe { std::mem::transmute(self) }
    }
}
