#[cfg(feature = "System+Runtime+Serialization+ObjectHolderListEnumerator")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectHolderListEnumerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_isFixupEnumerator: bool,
    pub m_list: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::ObjectHolderList,
    >,
    pub m_startingVersion: i32,
    pub m_currPos: i32,
}
#[cfg(feature = "System+Runtime+Serialization+ObjectHolderListEnumerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::ObjectHolderListEnumerator =>
    "System.Runtime.Serialization"."ObjectHolderListEnumerator"
);
#[cfg(feature = "System+Runtime+Serialization+ObjectHolderListEnumerator")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::ObjectHolderListEnumerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+ObjectHolderListEnumerator")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::ObjectHolderListEnumerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+ObjectHolderListEnumerator")]
impl crate::System::Runtime::Serialization::ObjectHolderListEnumerator {
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveNext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        list: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ObjectHolderList,
        >,
        isFixupEnumerator: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (list, isFixupEnumerator))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        list: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ObjectHolderList,
        >,
        isFixupEnumerator: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (list, isFixupEnumerator))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::ObjectHolder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ObjectHolder,
        > = __cordl_object.invoke("get_Current", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Serialization+ObjectHolderListEnumerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::ObjectHolderListEnumerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
