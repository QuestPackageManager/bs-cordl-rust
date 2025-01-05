#[cfg(feature = "Org+BouncyCastle+Cms+RecipientInformationStore")]
#[repr(C)]
#[derive(Debug)]
pub struct RecipientInformationStore {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub all: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    pub table: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
}
#[cfg(feature = "Org+BouncyCastle+Cms+RecipientInformationStore")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::RecipientInformationStore => "Org.BouncyCastle.Cms"
    ."RecipientInformationStore"
);
#[cfg(feature = "Org+BouncyCastle+Cms+RecipientInformationStore")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::RecipientInformationStore {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+RecipientInformationStore")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::RecipientInformationStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+RecipientInformationStore")]
impl crate::Org::BouncyCastle::Cms::RecipientInformationStore {
    pub fn GetFirstRecipient(
        &mut self,
        selector: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::RecipientID>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::RecipientInformation>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::RecipientInformation,
        > = __cordl_object.invoke("GetFirstRecipient", (selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRecipients_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ICollection,
        > = __cordl_object.invoke("GetRecipients", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRecipients_Gc1(
        &mut self,
        selector: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::RecipientID>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ICollection,
        > = __cordl_object.invoke("GetRecipients", (selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        recipientInfos: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ICollection,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (recipientInfos))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        recipientInfos: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ICollection,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (recipientInfos))?;
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
        selector: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::RecipientID>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::RecipientInformation>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::RecipientInformation,
        > = __cordl_object.invoke("get_Item", (selector))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+RecipientInformationStore")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::RecipientInformationStore {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
