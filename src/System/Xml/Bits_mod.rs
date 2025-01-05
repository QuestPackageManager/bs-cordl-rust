#[cfg(feature = "System+Xml+Bits")]
#[repr(C)]
#[derive(Debug)]
pub struct Bits {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Xml+Bits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Bits => "System.Xml"."Bits"
);
#[cfg(feature = "System+Xml+Bits")]
impl std::ops::Deref for crate::System::Xml::Bits {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Bits")]
impl std::ops::DerefMut for crate::System::Xml::Bits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Bits")]
impl crate::System::Xml::Bits {
    pub fn Count(num: u32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Count", (num))?;
        Ok(__cordl_ret.into())
    }
    pub fn LeastPosition(num: u32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LeastPosition", (num))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Bits")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Bits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
