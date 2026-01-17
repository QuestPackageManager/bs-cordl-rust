#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+FastAction_3")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct FastAction_3<
    A: quest_hook::libil2cpp::Type,
    B: quest_hook::libil2cpp::Type,
    C: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub delegates: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::LinkedList_1<
            quest_hook::libil2cpp::Gc<crate::System::Action_3<A, B, C>>,
        >,
    >,
    pub lookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::System::Action_3<A, B, C>>,
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::LinkedListNode_1<
                    quest_hook::libil2cpp::Gc<crate::System::Action_3<A, B, C>>,
                >,
            >,
        >,
    >,
    __cordl_phantom_A: std::marker::PhantomData<A>,
    __cordl_phantom_B: std::marker::PhantomData<B>,
    __cordl_phantom_C: std::marker::PhantomData<C>,
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+FastAction_3")]
unsafe impl<
        A: quest_hook::libil2cpp::Type,
        B: quest_hook::libil2cpp::Type,
        C: quest_hook::libil2cpp::Type,
    > quest_hook::libil2cpp::Type for crate::UnityEngine::TextCore::Text::FastAction_3<A, B, C>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.Text";
    const CLASS_NAME: &'static str = "FastAction`3";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find("UnityEngine.TextCore.Text", "FastAction`3")
                .unwrap()
                .make_generic::<(A, B, C)>()
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
#[cfg(feature = "UnityEngine+TextCore+Text+FastAction_3")]
impl<
        A: quest_hook::libil2cpp::Type,
        B: quest_hook::libil2cpp::Type,
        C: quest_hook::libil2cpp::Type,
    > std::ops::Deref for crate::UnityEngine::TextCore::Text::FastAction_3<A, B, C>
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+FastAction_3")]
impl<
        A: quest_hook::libil2cpp::Type,
        B: quest_hook::libil2cpp::Type,
        C: quest_hook::libil2cpp::Type,
    > std::ops::DerefMut for crate::UnityEngine::TextCore::Text::FastAction_3<A, B, C>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+FastAction_3")]
impl<
        A: quest_hook::libil2cpp::Type,
        B: quest_hook::libil2cpp::Type,
        C: quest_hook::libil2cpp::Type,
    > crate::UnityEngine::TextCore::Text::FastAction_3<A, B, C>
{
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        A: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        B: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        C: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        A: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        B: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        C: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+FastAction_3")]
impl<
        A: quest_hook::libil2cpp::Type,
        B: quest_hook::libil2cpp::Type,
        C: quest_hook::libil2cpp::Type,
    > quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::TextCore::Text::FastAction_3<A, B, C>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
