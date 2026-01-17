#[cfg(feature = "cordl_class_System+Data+DataViewManagerListItemTypeDescriptor")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct DataViewManagerListItemTypeDescriptor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _dataViewManager: quest_hook::libil2cpp::Gc<crate::System::Data::DataViewManager>,
}
#[cfg(feature = "cordl_class_System+Data+DataViewManagerListItemTypeDescriptor")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::System::Data::DataViewManagerListItemTypeDescriptor
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Data";
    const CLASS_NAME: &'static str = "DataViewManagerListItemTypeDescriptor";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "System+Data+DataViewManagerListItemTypeDescriptor")]
impl std::ops::Deref for crate::System::Data::DataViewManagerListItemTypeDescriptor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataViewManagerListItemTypeDescriptor")]
impl std::ops::DerefMut for crate::System::Data::DataViewManagerListItemTypeDescriptor {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataViewManagerListItemTypeDescriptor")]
impl crate::System::Data::DataViewManagerListItemTypeDescriptor {
    pub fn GetDataView(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Data::DataView>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>),
                        quest_hook::libil2cpp::Gc<crate::System::Data::DataView>,
                        1usize,
                    >("GetDataView")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDataView", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataView> =
            unsafe { cordl_method_info.invoke_unchecked(self, (table))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Data+DataViewManagerListItemTypeDescriptor")]
impl quest_hook::libil2cpp::ObjectType
    for crate::System::Data::DataViewManagerListItemTypeDescriptor
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
