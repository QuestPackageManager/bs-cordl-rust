#[cfg(feature = "System+ComponentModel+ListChangedEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct ListChangedEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub _ListChangedType_k__BackingField: crate::System::ComponentModel::ListChangedType,
    pub _NewIndex_k__BackingField: i32,
    pub _OldIndex_k__BackingField: i32,
    pub _PropertyDescriptor_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::PropertyDescriptor,
    >,
}
#[cfg(feature = "System+ComponentModel+ListChangedEventArgs")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::ComponentModel::ListChangedEventArgs {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.ComponentModel";
    const CLASS_NAME: &'static str = "ListChangedEventArgs";
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
#[cfg(feature = "System+ComponentModel+ListChangedEventArgs")]
impl std::ops::Deref for crate::System::ComponentModel::ListChangedEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ListChangedEventArgs")]
impl std::ops::DerefMut for crate::System::ComponentModel::ListChangedEventArgs {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ListChangedEventArgs")]
impl crate::System::ComponentModel::ListChangedEventArgs {
    pub fn New_PropertyDescriptor2(
        listChangedType: crate::System::ComponentModel::ListChangedType,
        propDesc: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (listChangedType, propDesc))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_0(
        listChangedType: crate::System::ComponentModel::ListChangedType,
        newIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (listChangedType, newIndex))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_PropertyDescriptor1(
        listChangedType: crate::System::ComponentModel::ListChangedType,
        newIndex: i32,
        propDesc: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (listChangedType, newIndex, propDesc))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_i32_3(
        listChangedType: crate::System::ComponentModel::ListChangedType,
        newIndex: i32,
        oldIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (listChangedType, newIndex, oldIndex))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_PropertyDescriptor2(
        &mut self,
        listChangedType: crate::System::ComponentModel::ListChangedType,
        propDesc: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::System::ComponentModel::ListChangedType,
                            quest_hook::libil2cpp::Gc<
                                crate::System::ComponentModel::PropertyDescriptor,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (listChangedType, propDesc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_0(
        &mut self,
        listChangedType: crate::System::ComponentModel::ListChangedType,
        newIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::ComponentModel::ListChangedType, i32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (listChangedType, newIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_PropertyDescriptor1(
        &mut self,
        listChangedType: crate::System::ComponentModel::ListChangedType,
        newIndex: i32,
        propDesc: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::System::ComponentModel::ListChangedType,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::ComponentModel::PropertyDescriptor,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (listChangedType, newIndex, propDesc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_3(
        &mut self,
        listChangedType: crate::System::ComponentModel::ListChangedType,
        newIndex: i32,
        oldIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::ComponentModel::ListChangedType, i32, i32),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (listChangedType, newIndex, oldIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ListChangedType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::ComponentModel::ListChangedType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::System::ComponentModel::ListChangedType,
                        0usize,
                    >("get_ListChangedType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_ListChangedType", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ComponentModel::ListChangedType = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_NewIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_NewIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_NewIndex", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_OldIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_OldIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_OldIndex", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ComponentModel+ListChangedEventArgs")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::ListChangedEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
