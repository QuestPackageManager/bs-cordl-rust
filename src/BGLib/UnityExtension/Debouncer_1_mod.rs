#[cfg(feature = "BGLib+UnityExtension+Debouncer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct Debouncer_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _callback: quest_hook::libil2cpp::Gc<crate::System::Action_1<T>>,
    pub _debounceDelaySeconds: f32,
    pub _debounceQueue: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Queue_1<T>,
    >,
    pub _nextCallbackTime: f32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "BGLib+UnityExtension+Debouncer_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::UnityExtension::Debouncer_1 < T > =>
    "BGLib.UnityExtension"."Debouncer`1" < T >
);
#[cfg(feature = "BGLib+UnityExtension+Debouncer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::BGLib::UnityExtension::Debouncer_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+Debouncer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::BGLib::UnityExtension::Debouncer_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+Debouncer_1")]
impl<T: quest_hook::libil2cpp::Type> crate::BGLib::UnityExtension::Debouncer_1<T> {
    pub fn Dispose(
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
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Enqueue(
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
            .invoke("Enqueue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentTime(&mut self) -> quest_hook::libil2cpp::Result<f32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetCurrentTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        callback: quest_hook::libil2cpp::Gc<crate::System::Action_1<T>>,
        debounceDelaySeconds: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (callback, debounceDelaySeconds))?;
        Ok(__cordl_object.into())
    }
    pub fn Tick(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Tick", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action_1<T>>,
        debounceDelaySeconds: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (callback, debounceDelaySeconds))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+UnityExtension+Debouncer_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::BGLib::UnityExtension::Debouncer_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BGLib+UnityExtension+Debouncer_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::System::IDisposable>
for crate::BGLib::UnityExtension::Debouncer_1<T> {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BGLib+UnityExtension+Debouncer_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::System::IDisposable>
for crate::BGLib::UnityExtension::Debouncer_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
