#[cfg(feature = "cordl_class_LiteNetLib+Utils+INetImmutableSerializable_1")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[repr(C)]
pub struct INetImmutableSerializable_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "cordl_class_LiteNetLib+Utils+INetImmutableSerializable_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
    for crate::LiteNetLib::Utils::INetImmutableSerializable_1<T>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "INetImmutableSerializable`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find(
                "LiteNetLib.Utils",
                "INetImmutableSerializable`1",
            )
            .unwrap()
            .make_generic::<(T)>()
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
#[cfg(feature = "LiteNetLib+Utils+INetImmutableSerializable_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
    for crate::LiteNetLib::Utils::INetImmutableSerializable_1<T>
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+INetImmutableSerializable_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
    for crate::LiteNetLib::Utils::INetImmutableSerializable_1<T>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+INetImmutableSerializable_1")]
impl<T: quest_hook::libil2cpp::Type> crate::LiteNetLib::Utils::INetImmutableSerializable_1<T> {
    pub fn CreateFromSerializedData(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::LiteNetLib::Utils::NetDataReader,
                        >),
                        T,
                        1usize,
                    >("CreateFromSerializedData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateFromSerializedData", 1usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe { cordl_method_info.invoke_unchecked(self, (reader))? };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(object_param: *mut quest_hook::libil2cpp::Il2CppObject) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_LiteNetLib+Utils+INetImmutableSerializable_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
    for crate::LiteNetLib::Utils::INetImmutableSerializable_1<T>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+INetImmutableSerializable_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::LiteNetLib::Utils::INetImmutableSerializable>
    for crate::LiteNetLib::Utils::INetImmutableSerializable_1<T>
{
    fn as_ref(&self) -> &crate::LiteNetLib::Utils::INetImmutableSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LiteNetLib+Utils+INetImmutableSerializable_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::LiteNetLib::Utils::INetImmutableSerializable>
    for crate::LiteNetLib::Utils::INetImmutableSerializable_1<T>
{
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::Utils::INetImmutableSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
