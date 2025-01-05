#[cfg(feature = "Org+BouncyCastle+Utilities+Date+DateTimeObject")]
#[repr(C)]
#[derive(Debug)]
pub struct DateTimeObject {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub dt: crate::System::DateTime,
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Date+DateTimeObject")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Utilities::Date::DateTimeObject =>
    "Org.BouncyCastle.Utilities.Date"."DateTimeObject"
);
#[cfg(feature = "Org+BouncyCastle+Utilities+Date+DateTimeObject")]
impl std::ops::Deref for crate::Org::BouncyCastle::Utilities::Date::DateTimeObject {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Date+DateTimeObject")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Utilities::Date::DateTimeObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Date+DateTimeObject")]
impl crate::Org::BouncyCastle::Utilities::Date::DateTimeObject {
    pub fn New(
        dt: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dt))?;
        Ok(__cordl_object.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        dt: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dt))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("get_Value", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Date+DateTimeObject")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Utilities::Date::DateTimeObject {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
