#[cfg(feature = "System+Security+Policy+Evidence")]
#[repr(C)]
#[derive(Debug)]
pub struct Evidence {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _locked: bool,
    pub hostEvidenceList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::ArrayList,
    >,
    pub assemblyEvidenceList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::ArrayList,
    >,
}
#[cfg(feature = "System+Security+Policy+Evidence")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Policy::Evidence =>
    "System.Security.Policy"."Evidence"
);
#[cfg(feature = "System+Security+Policy+Evidence")]
impl std::ops::Deref for crate::System::Security::Policy::Evidence {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Policy+Evidence")]
impl std::ops::DerefMut for crate::System::Security::Policy::Evidence {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Policy+Evidence")]
impl crate::System::Security::Policy::Evidence {
    #[cfg(feature = "System+Security+Policy+Evidence+EvidenceEnumerator")]
    pub type EvidenceEnumerator = crate::System::Security::Policy::Evidence_EvidenceEnumerator;
    pub fn CopyTo(
        &mut self,
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyTo", (array, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSynchronized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsSynchronized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SyncRoot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_SyncRoot", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+Policy+Evidence")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Security::Policy::Evidence {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Security+Policy+Evidence")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>>
for crate::System::Security::Policy::Evidence {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Security+Policy+Evidence")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>>
for crate::System::Security::Policy::Evidence {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Security+Policy+Evidence")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>>
for crate::System::Security::Policy::Evidence {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Security+Policy+Evidence")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>>
for crate::System::Security::Policy::Evidence {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Security+Policy+Evidence+EvidenceEnumerator")]
#[repr(C)]
#[derive(Debug)]
pub struct Evidence_EvidenceEnumerator {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub currentEnum: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    pub hostEnum: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    pub assemblyEnum: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
}
#[cfg(feature = "System+Security+Policy+Evidence+EvidenceEnumerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Policy::Evidence_EvidenceEnumerator => "System.Security.Policy"
    ."Evidence/EvidenceEnumerator"
);
#[cfg(feature = "System+Security+Policy+Evidence+EvidenceEnumerator")]
impl std::ops::Deref for crate::System::Security::Policy::Evidence_EvidenceEnumerator {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Policy+Evidence+EvidenceEnumerator")]
impl std::ops::DerefMut
for crate::System::Security::Policy::Evidence_EvidenceEnumerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Policy+Evidence+EvidenceEnumerator")]
impl crate::System::Security::Policy::Evidence_EvidenceEnumerator {
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveNext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        hostenum: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
        assemblyenum: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (hostenum, assemblyenum))?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        hostenum: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
        assemblyenum: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (hostenum, assemblyenum))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_Current", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+Policy+Evidence+EvidenceEnumerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Policy::Evidence_EvidenceEnumerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Security+Policy+Evidence+EvidenceEnumerator")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>>
for crate::System::Security::Policy::Evidence_EvidenceEnumerator {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Security+Policy+Evidence+EvidenceEnumerator")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>>
for crate::System::Security::Policy::Evidence_EvidenceEnumerator {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator> {
        unsafe { std::mem::transmute(self) }
    }
}
