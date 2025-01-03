#[cfg(feature = "System+Data+Operators")]
#[repr(C)]
#[derive(Debug)]
pub struct Operators {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Data+Operators")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::Operators => "System.Data"
    ."Operators"
);
#[cfg(feature = "System+Data+Operators")]
impl std::ops::Deref for crate::System::Data::Operators {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+Operators")]
impl std::ops::DerefMut for crate::System::Data::Operators {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+Operators")]
impl crate::System::Data::Operators {
    pub fn IsArithmetical(op: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsArithmetical", (op))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsLogical(op: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsLogical", (op))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsRelational(op: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsRelational", (op))?;
        Ok(__cordl_ret.into())
    }
    pub fn Priority(op: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Priority", (op))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        op: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (op))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+Operators")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::Operators {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
