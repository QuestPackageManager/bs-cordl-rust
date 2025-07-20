#[cfg(feature = "System+ComponentModel+IRevertibleChangeTracking")]
#[repr(C)]
#[derive(Debug)]
pub struct IRevertibleChangeTracking {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+ComponentModel+IRevertibleChangeTracking")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::ComponentModel::IRevertibleChangeTracking {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.ComponentModel";
    const CLASS_NAME: &'static str = "IRevertibleChangeTracking";
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
#[cfg(feature = "System+ComponentModel+IRevertibleChangeTracking")]
impl std::ops::Deref for crate::System::ComponentModel::IRevertibleChangeTracking {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+IRevertibleChangeTracking")]
impl std::ops::DerefMut for crate::System::ComponentModel::IRevertibleChangeTracking {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+IRevertibleChangeTracking")]
impl crate::System::ComponentModel::IRevertibleChangeTracking {
    pub fn RejectChanges(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("RejectChanges")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "RejectChanges", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+ComponentModel+IRevertibleChangeTracking")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::IRevertibleChangeTracking {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+ComponentModel+IRevertibleChangeTracking")]
impl AsRef<crate::System::ComponentModel::IChangeTracking>
for crate::System::ComponentModel::IRevertibleChangeTracking {
    fn as_ref(&self) -> &crate::System::ComponentModel::IChangeTracking {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+ComponentModel+IRevertibleChangeTracking")]
impl AsMut<crate::System::ComponentModel::IChangeTracking>
for crate::System::ComponentModel::IRevertibleChangeTracking {
    fn as_mut(&mut self) -> &mut crate::System::ComponentModel::IChangeTracking {
        unsafe { std::mem::transmute(self) }
    }
}
