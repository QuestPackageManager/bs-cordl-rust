#[cfg(feature = "System+Collections+Generic+InternalStringComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct InternalStringComparer {
    __cordl_parent: crate::System::Collections::Generic::EqualityComparer_1<
        *mut crate::System::String,
    >,
}
#[cfg(feature = "System+Collections+Generic+InternalStringComparer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Collections::Generic::InternalStringComparer =>
    "System.Collections.Generic"."InternalStringComparer"
);
#[cfg(feature = "System+Collections+Generic+InternalStringComparer")]
impl std::ops::Deref for crate::System::Collections::Generic::InternalStringComparer {
    type Target = crate::System::Collections::Generic::EqualityComparer_1<
        *mut crate::System::String,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+InternalStringComparer")]
impl std::ops::DerefMut for crate::System::Collections::Generic::InternalStringComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+InternalStringComparer")]
impl crate::System::Collections::Generic::InternalStringComparer {
    pub fn IndexOf(
        &mut self,
        array: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        value: *mut crate::System::String,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("IndexOf", (array, value, startIndex, count))?;
        Ok(__cordl_ret)
    }
    pub fn Equals(
        &mut self,
        x: *mut crate::System::String,
        y: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (x, y))?;
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
    pub fn GetHashCode(
        &mut self,
        obj: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Collections+Generic+InternalStringComparer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Collections::Generic::InternalStringComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
