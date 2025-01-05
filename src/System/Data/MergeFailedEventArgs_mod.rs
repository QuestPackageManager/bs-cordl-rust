#[cfg(feature = "System+Data+MergeFailedEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct MergeFailedEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub _Table_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Data::DataTable,
    >,
    pub _Conflict_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "System+Data+MergeFailedEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::MergeFailedEventArgs =>
    "System.Data"."MergeFailedEventArgs"
);
#[cfg(feature = "System+Data+MergeFailedEventArgs")]
impl std::ops::Deref for crate::System::Data::MergeFailedEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+MergeFailedEventArgs")]
impl std::ops::DerefMut for crate::System::Data::MergeFailedEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+MergeFailedEventArgs")]
impl crate::System::Data::MergeFailedEventArgs {
    pub fn New(
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        conflict: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (table, conflict))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        conflict: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (table, conflict))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Conflict(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Conflict", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+MergeFailedEventArgs")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::MergeFailedEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
