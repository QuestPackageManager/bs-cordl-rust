#[cfg(feature = "System+Linq+Expressions+SwitchCase")]
#[repr(C)]
#[derive(Debug)]
pub struct SwitchCase {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _TestValues_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        >,
    >,
    pub _Body_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Expression,
    >,
}
#[cfg(feature = "System+Linq+Expressions+SwitchCase")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::SwitchCase =>
    "System.Linq.Expressions"."SwitchCase"
);
#[cfg(feature = "System+Linq+Expressions+SwitchCase")]
impl std::ops::Deref for crate::System::Linq::Expressions::SwitchCase {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+SwitchCase")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::SwitchCase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+SwitchCase")]
impl crate::System::Linq::Expressions::SwitchCase {
    pub fn get_Body(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("get_Body", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TestValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
            >,
        > = __cordl_object.invoke("get_TestValues", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+SwitchCase")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Linq::Expressions::SwitchCase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
