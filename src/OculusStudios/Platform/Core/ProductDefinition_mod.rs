#[cfg(feature = "cordl_class_OculusStudios+Platform+Core+ProductDefinition")]
#[repr(C)]
#[derive(Debug)]
pub struct ProductDefinition {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _idForVendor: quest_hook::libil2cpp::Gc<
        crate::AYellowpaper::SerializedCollections::SerializedDictionary_2<
            crate::OculusStudios::Platform::Core::Vendor,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub _children: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::OculusStudios::Platform::Core::ProductDefinition,
            >,
        >,
    >,
    pub _ownershipRequirement: crate::OculusStudios::Platform::Core::OwnershipRequirement,
}
#[cfg(feature = "cordl_class_OculusStudios+Platform+Core+ProductDefinition")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OculusStudios::Platform::Core::ProductDefinition {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OculusStudios.Platform.Core";
    const CLASS_NAME: &'static str = "ProductDefinition";
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
#[cfg(feature = "OculusStudios+Platform+Core+ProductDefinition")]
impl std::ops::Deref for crate::OculusStudios::Platform::Core::ProductDefinition {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusStudios+Platform+Core+ProductDefinition")]
impl std::ops::DerefMut for crate::OculusStudios::Platform::Core::ProductDefinition {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusStudios+Platform+Core+ProductDefinition")]
impl crate::OculusStudios::Platform::Core::ProductDefinition {
    pub fn New(
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        idForVendor: quest_hook::libil2cpp::Gc<
            crate::AYellowpaper::SerializedCollections::SerializedDictionary_2<
                crate::OculusStudios::Platform::Core::Vendor,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        children: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::OculusStudios::Platform::Core::ProductDefinition,
                >,
            >,
        >,
        ownershipRequirement: crate::OculusStudios::Platform::Core::OwnershipRequirement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (id, idForVendor, children, ownershipRequirement))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        idForVendor: quest_hook::libil2cpp::Gc<
            crate::AYellowpaper::SerializedCollections::SerializedDictionary_2<
                crate::OculusStudios::Platform::Core::Vendor,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        children: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::OculusStudios::Platform::Core::ProductDefinition,
                >,
            >,
        >,
        ownershipRequirement: crate::OculusStudios::Platform::Core::OwnershipRequirement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::AYellowpaper::SerializedCollections::SerializedDictionary_2<
                                    crate::OculusStudios::Platform::Core::Vendor,
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppString,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::OculusStudios::Platform::Core::ProductDefinition,
                                    >,
                                >,
                            >,
                            crate::OculusStudios::Platform::Core::OwnershipRequirement,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (id, idForVendor, children, ownershipRequirement),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_children(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::OculusStudios::Platform::Core::IProductDefinition,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::OculusStudios::Platform::Core::IProductDefinition,
                                >,
                            >,
                        >,
                        0usize,
                    >("get_children")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_children", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::OculusStudios::Platform::Core::IProductDefinition,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_id(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_id")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "get_id",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_idForVendor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::AYellowpaper::SerializedCollections::SerializedDictionary_2<
                crate::OculusStudios::Platform::Core::Vendor,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::AYellowpaper::SerializedCollections::SerializedDictionary_2<
                                crate::OculusStudios::Platform::Core::Vendor,
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppString,
                                >,
                            >,
                        >,
                        0usize,
                    >("get_idForVendor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_idForVendor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::AYellowpaper::SerializedCollections::SerializedDictionary_2<
                crate::OculusStudios::Platform::Core::Vendor,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ownershipRequirement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::OculusStudios::Platform::Core::OwnershipRequirement,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::OculusStudios::Platform::Core::OwnershipRequirement,
                        0usize,
                    >("get_ownershipRequirement")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_ownershipRequirement", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::OculusStudios::Platform::Core::OwnershipRequirement = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OculusStudios+Platform+Core+ProductDefinition")]
impl quest_hook::libil2cpp::ObjectType
for crate::OculusStudios::Platform::Core::ProductDefinition {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OculusStudios+Platform+Core+ProductDefinition")]
impl AsRef<crate::OculusStudios::Platform::Core::IProductDefinition>
for crate::OculusStudios::Platform::Core::ProductDefinition {
    fn as_ref(&self) -> &crate::OculusStudios::Platform::Core::IProductDefinition {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusStudios+Platform+Core+ProductDefinition")]
impl AsMut<crate::OculusStudios::Platform::Core::IProductDefinition>
for crate::OculusStudios::Platform::Core::ProductDefinition {
    fn as_mut(
        &mut self,
    ) -> &mut crate::OculusStudios::Platform::Core::IProductDefinition {
        unsafe { std::mem::transmute(self) }
    }
}
