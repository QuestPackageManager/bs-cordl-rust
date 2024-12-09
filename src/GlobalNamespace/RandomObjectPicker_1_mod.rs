#[cfg(feature = "RandomObjectPicker_1")]
#[repr(C)]
#[derive(Debug)]
pub struct RandomObjectPicker_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Object,
    pub _objects: *mut quest_hook::libil2cpp::Il2CppArray<T>,
    pub _lastPickTime: f32,
    pub _minimumPickInterval: f32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "RandomObjectPicker_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::RandomObjectPicker_1 < T > =>
    ""."RandomObjectPicker`1" < T >
);
#[cfg(feature = "RandomObjectPicker_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::GlobalNamespace::RandomObjectPicker_1<T> {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RandomObjectPicker_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::GlobalNamespace::RandomObjectPicker_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RandomObjectPicker_1")]
impl<T: quest_hook::libil2cpp::Type> crate::GlobalNamespace::RandomObjectPicker_1<T> {
    pub fn New_Il2CppArray1(
        objects: *mut quest_hook::libil2cpp::Il2CppArray<T>,
        minimumPickInterval: f32,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (objects, minimumPickInterval))?;
        Ok(__cordl_object)
    }
    pub fn New_T0(
        obj: T,
        minimumPickInterval: f32,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (obj, minimumPickInterval))?;
        Ok(__cordl_object)
    }
    pub fn PickRandomObject(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("PickRandomObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray1(
        &mut self,
        objects: *mut quest_hook::libil2cpp::Il2CppArray<T>,
        minimumPickInterval: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (objects, minimumPickInterval))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_T0(
        &mut self,
        obj: T,
        minimumPickInterval: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (obj, minimumPickInterval))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "RandomObjectPicker_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::RandomObjectPicker_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
