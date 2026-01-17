#[cfg(feature = "cordl_class_Unity+Properties+IListPropertyAccept_1")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[repr(C)]
pub struct IListPropertyAccept_1<TList: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TList: std::marker::PhantomData<TList>,
}
#[cfg(feature = "cordl_class_Unity+Properties+IListPropertyAccept_1")]
unsafe impl<TList: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
    for crate::Unity::Properties::IListPropertyAccept_1<TList>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Properties";
    const CLASS_NAME: &'static str = "IListPropertyAccept`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find("Unity.Properties", "IListPropertyAccept`1")
                .unwrap()
                .make_generic::<(TList)>()
                .unwrap()
                .unwrap()
        })
    }
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
#[cfg(feature = "Unity+Properties+IListPropertyAccept_1")]
impl<TList: quest_hook::libil2cpp::Type> std::ops::Deref
    for crate::Unity::Properties::IListPropertyAccept_1<TList>
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+IListPropertyAccept_1")]
impl<TList: quest_hook::libil2cpp::Type> std::ops::DerefMut
    for crate::Unity::Properties::IListPropertyAccept_1<TList>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+IListPropertyAccept_1")]
impl<TList: quest_hook::libil2cpp::Type> crate::Unity::Properties::IListPropertyAccept_1<TList> {
    pub fn Accept<TContainer>(
        &mut self,
        visitor: quest_hook::libil2cpp::Gc<crate::Unity::Properties::IListPropertyVisitor>,
        property: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::Property_2<TContainer, TList>,
        >,
        container: quest_hook::libil2cpp::ByRefMut<TContainer>,
        list: quest_hook::libil2cpp::ByRefMut<TList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TList: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContainer: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::Unity::Properties::IListPropertyVisitor>,
                        quest_hook::libil2cpp::Gc<
                            crate::Unity::Properties::Property_2<TContainer, TList>,
                        >,
                        quest_hook::libil2cpp::ByRefMut<TContainer>,
                        quest_hook::libil2cpp::ByRefMut<TList>,
                    ), quest_hook::libil2cpp::Void, 4usize>("Accept")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Accept",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (visitor, property, container, list))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(object_param: *mut quest_hook::libil2cpp::Il2CppObject) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_Unity+Properties+IListPropertyAccept_1")]
impl<TList: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
    for crate::Unity::Properties::IListPropertyAccept_1<TList>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
