#[cfg(feature = "System+Collections+Generic+BitHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct BitHelper {
    __cordl_parent: crate::System::Object,
    pub _length: i32,
    pub _arrayPtr: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _array: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _useStackAlloc: bool,
}
#[cfg(feature = "System+Collections+Generic+BitHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Collections::Generic::BitHelper =>
    "System.Collections.Generic"."BitHelper"
);
#[cfg(feature = "System+Collections+Generic+BitHelper")]
impl std::ops::Deref for crate::System::Collections::Generic::BitHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+BitHelper")]
impl std::ops::DerefMut for crate::System::Collections::Generic::BitHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+BitHelper")]
impl crate::System::Collections::Generic::BitHelper {
    pub fn IsMarked(&mut self, bitPosition: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsMarked", (bitPosition))?;
        Ok(__cordl_ret)
    }
    pub fn MarkBit(
        &mut self,
        bitPosition: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkBit", (bitPosition))?;
        Ok(__cordl_ret)
    }
    pub fn New_Il2CppArray1(
        bitArray: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bitArray, length))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppObject0(
        bitArrayPtr: *mut quest_hook::libil2cpp::Il2CppObject,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bitArrayPtr, length))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_Il2CppArray1(
        &mut self,
        bitArray: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bitArray, length))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppObject0(
        &mut self,
        bitArrayPtr: *mut quest_hook::libil2cpp::Il2CppObject,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bitArrayPtr, length))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Collections+Generic+BitHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Collections::Generic::BitHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}