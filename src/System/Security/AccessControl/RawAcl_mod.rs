#[cfg(feature = "System+Security+AccessControl+RawAcl")]
#[repr(C)]
#[derive(Debug)]
pub struct RawAcl {
    __cordl_parent: crate::System::Security::AccessControl::GenericAcl,
    pub revision: u8,
    pub list: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::System::Security::AccessControl::GenericAce>,
        >,
    >,
}
#[cfg(feature = "System+Security+AccessControl+RawAcl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::AccessControl::RawAcl =>
    "System.Security.AccessControl"."RawAcl"
);
#[cfg(feature = "System+Security+AccessControl+RawAcl")]
impl std::ops::Deref for crate::System::Security::AccessControl::RawAcl {
    type Target = crate::System::Security::AccessControl::GenericAcl;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+RawAcl")]
impl std::ops::DerefMut for crate::System::Security::AccessControl::RawAcl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+RawAcl")]
impl crate::System::Security::AccessControl::RawAcl {
    pub fn InsertAce(
        &mut self,
        index: i32,
        ace: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::GenericAce,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InsertAce", (index, ace))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        revision: u8,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (revision, capacity))?;
        Ok(__cordl_object.into())
    }
    pub fn RemoveAce(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveAce", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        revision: u8,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (revision, capacity))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::AccessControl::GenericAce>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::GenericAce,
        > = __cordl_object.invoke("get_Item", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Item(
        &mut self,
        index: i32,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::GenericAce,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Item", (index, value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+AccessControl+RawAcl")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::AccessControl::RawAcl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
