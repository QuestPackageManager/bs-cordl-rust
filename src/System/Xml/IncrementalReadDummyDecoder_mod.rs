#[cfg(feature = "System+Xml+IncrementalReadDummyDecoder")]
#[repr(C)]
#[derive(Debug)]
pub struct IncrementalReadDummyDecoder {
    __cordl_parent: crate::System::Xml::IncrementalReadDecoder,
}
#[cfg(feature = "System+Xml+IncrementalReadDummyDecoder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::IncrementalReadDummyDecoder =>
    "System.Xml"."IncrementalReadDummyDecoder"
);
#[cfg(feature = "System+Xml+IncrementalReadDummyDecoder")]
impl std::ops::Deref for crate::System::Xml::IncrementalReadDummyDecoder {
    type Target = crate::System::Xml::IncrementalReadDecoder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+IncrementalReadDummyDecoder")]
impl std::ops::DerefMut for crate::System::Xml::IncrementalReadDummyDecoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+IncrementalReadDummyDecoder")]
impl crate::System::Xml::IncrementalReadDummyDecoder {
    pub fn get_IsFull(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsFull", ())?;
        Ok(__cordl_ret)
    }
    pub fn Decode(
        &mut self,
        chars: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        startPos: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Decode", (chars, startPos, len))?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+IncrementalReadDummyDecoder")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::IncrementalReadDummyDecoder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
