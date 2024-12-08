#[cfg(feature = "System+Xml+ByteStack")]
#[repr(C)]
#[derive(Debug)]
pub struct ByteStack {
    __cordl_parent: crate::System::Object,
    pub stack: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub growthRate: i32,
    pub top: i32,
    pub _cordl_size: i32,
}
#[cfg(feature = "System+Xml+ByteStack")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::ByteStack => "System.Xml"
    ."ByteStack"
);
#[cfg(feature = "System+Xml+ByteStack")]
impl std::ops::Deref for crate::System::Xml::ByteStack {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+ByteStack")]
impl std::ops::DerefMut for crate::System::Xml::ByteStack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+ByteStack")]
impl crate::System::Xml::ByteStack {
    pub fn New(growthRate: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (growthRate))?;
        Ok(__cordl_object)
    }
    pub fn Pop(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("Pop", ())?;
        Ok(__cordl_ret)
    }
    pub fn Push(
        &mut self,
        data: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Push", (data))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        growthRate: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (growthRate))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+ByteStack")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::ByteStack {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
