#[cfg(feature = "Org+BouncyCastle+Cms+RecipientInformationStore")]
#[repr(C)]
#[derive(Debug)]
pub struct RecipientInformationStore {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub all: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    pub table: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
}
#[cfg(feature = "Org+BouncyCastle+Cms+RecipientInformationStore")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Cms::RecipientInformationStore {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Cms";
    const CLASS_NAME: &'static str = "RecipientInformationStore";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+RecipientInformationStore")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::RecipientInformationStore {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn GetRecipients_RecipientID1(
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
