#[cfg(feature = "System+ComponentModel+ReflectTypeDescriptionProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectTypeDescriptionProvider {
    __cordl_parent: crate::System::ComponentModel::TypeDescriptionProvider,
    pub _typeData: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
}
#[cfg(feature = "System+ComponentModel+ReflectTypeDescriptionProvider")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::ComponentModel::ReflectTypeDescriptionProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.ComponentModel";
    const CLASS_NAME: &'static str = "ReflectTypeDescriptionProvider";
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
#[cfg(feature = "System+ComponentModel+ReflectTypeDescriptionProvider")]
impl std::ops::Deref for crate::System::ComponentModel::ReflectTypeDescriptionProvider {
    type Target = crate::System::ComponentModel::TypeDescriptionProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ReflectTypeDescriptionProvider")]
impl std::ops::DerefMut
for crate::System::ComponentModel::ReflectTypeDescriptionProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ReflectTypeDescriptionProvider")]
impl crate::System::ComponentModel::ReflectTypeDescriptionProvider {
    #[cfg(
        feature = "System+ComponentModel+ReflectTypeDescriptionProvider+ReflectedTypeData"
    )]
    pub type ReflectedTypeData = crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData;
    pub fn CreateInstance_IServiceProvider_Il2CppArray_Il2CppArray0(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IServiceProvider>,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        argTypes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::IServiceProvider>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                4usize,
            >("CreateInstance")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "CreateInstance", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe {
            method.invoke_unchecked(self, (provider, objectType, argTypes, args))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateInstance_Type1(
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        callingType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                2usize,
            >("CreateInstance")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "CreateInstance", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (objectType, callingType))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributes(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::AttributeCollection>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::AttributeCollection,
                >,
                1usize,
            >("GetAttributes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "GetAttributes", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::AttributeCollection,
        > = unsafe { method.invoke_unchecked(self, (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCache(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
                1usize,
            >("GetCache")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "GetCache", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IDictionary,
        > = unsafe { method.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetClassName(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetClassName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "GetClassName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetComponentName(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("GetComponentName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "GetComponentName", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (_cordl_type, instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetConverter(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::TypeConverter>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::ComponentModel::TypeConverter>,
                2usize,
            >("GetConverter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "GetConverter", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::TypeConverter,
        > = unsafe { method.invoke_unchecked(self, (_cordl_type, instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultEvent(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::EventDescriptor>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::EventDescriptor,
                >,
                2usize,
            >("GetDefaultEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "GetDefaultEvent", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptor,
        > = unsafe { method.invoke_unchecked(self, (_cordl_type, instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultProperty(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::PropertyDescriptor>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::PropertyDescriptor,
                >,
                2usize,
            >("GetDefaultProperty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "GetDefaultProperty", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptor,
        > = unsafe { method.invoke_unchecked(self, (_cordl_type, instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetEditor(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        editorBaseType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                3usize,
            >("GetEditor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "GetEditor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe {
            method.invoke_unchecked(self, (_cordl_type, instance, editorBaseType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetEditorTable(
        editorBaseType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
                1usize,
            >("GetEditorTable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "GetEditorTable", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Hashtable,
        > = unsafe { method.invoke_unchecked((), (editorBaseType))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetEvents(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptorCollection,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::EventDescriptorCollection,
                >,
                1usize,
            >("GetEvents")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "GetEvents", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptorCollection,
        > = unsafe { method.invoke_unchecked(self, (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetExtendedAttributes(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::AttributeCollection>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::AttributeCollection,
                >,
                1usize,
            >("GetExtendedAttributes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "GetExtendedAttributes",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::AttributeCollection,
        > = unsafe { method.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetExtendedClassName(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetExtendedClassName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "GetExtendedClassName",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetExtendedComponentName(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetExtendedComponentName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "GetExtendedComponentName",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetExtendedConverter(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::TypeConverter>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<crate::System::ComponentModel::TypeConverter>,
                1usize,
            >("GetExtendedConverter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "GetExtendedConverter",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::TypeConverter,
        > = unsafe { method.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetExtendedDefaultEvent(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::EventDescriptor>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::EventDescriptor,
                >,
                1usize,
            >("GetExtendedDefaultEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "GetExtendedDefaultEvent",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptor,
        > = unsafe { method.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetExtendedDefaultProperty(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::PropertyDescriptor>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::PropertyDescriptor,
                >,
                1usize,
            >("GetExtendedDefaultProperty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetExtendedDefaultProperty", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptor,
        > = unsafe { method.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetExtendedEditor(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        editorBaseType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                2usize,
            >("GetExtendedEditor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "GetExtendedEditor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, (instance, editorBaseType))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetExtendedEvents(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptorCollection,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::EventDescriptorCollection,
                >,
                1usize,
            >("GetExtendedEvents")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "GetExtendedEvents", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptorCollection,
        > = unsafe { method.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetExtendedProperties(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::PropertyDescriptorCollection,
                >,
                1usize,
            >("GetExtendedProperties")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "GetExtendedProperties",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        > = unsafe { method.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetExtendedPropertyOwner(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        pd: quest_hook::libil2cpp::Gc<crate::System::ComponentModel::PropertyDescriptor>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::ComponentModel::PropertyDescriptor,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                2usize,
            >("GetExtendedPropertyOwner")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "GetExtendedPropertyOwner",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, (instance, pd))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetExtendedTypeDescriptor(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::ICustomTypeDescriptor>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::ICustomTypeDescriptor,
                >,
                1usize,
            >("GetExtendedTypeDescriptor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "GetExtendedTypeDescriptor",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ICustomTypeDescriptor,
        > = unsafe { method.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetExtenderProviders(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::IExtenderProvider,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<
                            crate::System::ComponentModel::IExtenderProvider,
                        >,
                    >,
                >,
                1usize,
            >("GetExtenderProviders")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "GetExtenderProviders",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::IExtenderProvider,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetExtenders(
        components: quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cache: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::IExtenderProvider,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
                ),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<
                            crate::System::ComponentModel::IExtenderProvider,
                        >,
                    >,
                >,
                3usize,
            >("GetExtenders")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "GetExtenders", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::IExtenderProvider,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), (components, instance, cache))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetProperties(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::PropertyDescriptorCollection,
                >,
                1usize,
            >("GetProperties")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "GetProperties", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        > = unsafe { method.invoke_unchecked(self, (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyOwner(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        pd: quest_hook::libil2cpp::Gc<crate::System::ComponentModel::PropertyDescriptor>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::ComponentModel::PropertyDescriptor,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                3usize,
            >("GetPropertyOwner")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "GetPropertyOwner", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, (_cordl_type, instance, pd))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetReflectionType(
        &mut self,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                2usize,
            >("GetReflectionType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "GetReflectionType", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            method.invoke_unchecked(self, (objectType, instance))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeData(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        createIfNeeded: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>, bool),
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData,
                >,
                2usize,
            >("GetTypeData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "GetTypeData", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData,
        > = unsafe { method.invoke_unchecked(self, (_cordl_type, createIfNeeded))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeDescriptor(
        &mut self,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::ICustomTypeDescriptor>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::ICustomTypeDescriptor,
                >,
                2usize,
            >("GetTypeDescriptor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "GetTypeDescriptor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ICustomTypeDescriptor,
        > = unsafe { method.invoke_unchecked(self, (objectType, instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeFromName(
        typeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                1usize,
            >("GetTypeFromName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "GetTypeFromName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            method.invoke_unchecked((), (typeName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsPopulated(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                bool,
                1usize,
            >("IsPopulated")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "IsPopulated", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReflectGetAttributes_MemberInfo1(
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::System::Attribute>,
                    >,
                >,
                1usize,
            >("ReflectGetAttributes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "ReflectGetAttributes",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        > = unsafe { method.invoke_unchecked((), (member))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReflectGetAttributes_Type0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::System::Attribute>,
                    >,
                >,
                1usize,
            >("ReflectGetAttributes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "ReflectGetAttributes",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        > = unsafe { method.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReflectGetEvents(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::ComponentModel::EventDescriptor>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<
                            crate::System::ComponentModel::EventDescriptor,
                        >,
                    >,
                >,
                1usize,
            >("ReflectGetEvents")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "ReflectGetEvents", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::ComponentModel::EventDescriptor>,
            >,
        > = unsafe { method.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReflectGetExtendedProperties(
        provider: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::IExtenderProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::PropertyDescriptor,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::IExtenderProvider,
                >),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<
                            crate::System::ComponentModel::PropertyDescriptor,
                        >,
                    >,
                >,
                1usize,
            >("ReflectGetExtendedProperties")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(),
                    "ReflectGetExtendedProperties", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::PropertyDescriptor,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), (provider))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReflectGetProperties(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::PropertyDescriptor,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<
                            crate::System::ComponentModel::PropertyDescriptor,
                        >,
                    >,
                >,
                1usize,
            >("ReflectGetProperties")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "ReflectGetProperties",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::PropertyDescriptor,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn Refresh(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Refresh")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "Refresh", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (_cordl_type))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SearchIntrinsicTable(
        table: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
        callingType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                2usize,
            >("SearchIntrinsicTable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), "SearchIntrinsicTable",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (table, callingType))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IntrinsicTypeConverters() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
                0usize,
            >("get_IntrinsicTypeConverters")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ComponentModel::ReflectTypeDescriptionProvider as
                    quest_hook::libil2cpp::Type > ::class(),
                    "get_IntrinsicTypeConverters", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Hashtable,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ComponentModel+ReflectTypeDescriptionProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::ReflectTypeDescriptionProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "System+ComponentModel+ReflectTypeDescriptionProvider+ReflectedTypeData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectTypeDescriptionProvider_ReflectedTypeData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub _attributes: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::AttributeCollection,
    >,
    pub _events: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::EventDescriptorCollection,
    >,
    pub _properties: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::PropertyDescriptorCollection,
    >,
    pub _converter: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::TypeConverter,
    >,
    pub _editors: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    >,
    pub _editorTypes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
        >,
    >,
    pub _editorCount: i32,
}
#[cfg(
    feature = "System+ComponentModel+ReflectTypeDescriptionProvider+ReflectedTypeData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.ComponentModel";
    const CLASS_NAME: &'static str = "ReflectTypeDescriptionProvider/ReflectedTypeData";
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
#[cfg(
    feature = "System+ComponentModel+ReflectTypeDescriptionProvider+ReflectedTypeData"
)]
impl std::ops::Deref
for crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+ComponentModel+ReflectTypeDescriptionProvider+ReflectedTypeData"
)]
impl std::ops::DerefMut
for crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+ComponentModel+ReflectTypeDescriptionProvider+ReflectedTypeData"
)]
impl crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData {
    pub fn GetAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::AttributeCollection>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::AttributeCollection,
                >,
                0usize,
            >("GetAttributes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData
                    as quest_hook::libil2cpp::Type > ::class(), "GetAttributes", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::AttributeCollection,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetClassName(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetClassName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData
                    as quest_hook::libil2cpp::Type > ::class(), "GetClassName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetComponentName(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetComponentName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData
                    as quest_hook::libil2cpp::Type > ::class(), "GetComponentName",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetConverter(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::TypeConverter>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<crate::System::ComponentModel::TypeConverter>,
                1usize,
            >("GetConverter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData
                    as quest_hook::libil2cpp::Type > ::class(), "GetConverter", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::TypeConverter,
        > = unsafe { method.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultEvent(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::EventDescriptor>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::EventDescriptor,
                >,
                1usize,
            >("GetDefaultEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData
                    as quest_hook::libil2cpp::Type > ::class(), "GetDefaultEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptor,
        > = unsafe { method.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultProperty(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::PropertyDescriptor>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::PropertyDescriptor,
                >,
                1usize,
            >("GetDefaultProperty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData
                    as quest_hook::libil2cpp::Type > ::class(), "GetDefaultProperty",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptor,
        > = unsafe { method.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetEditor(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        editorBaseType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                2usize,
            >("GetEditor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData
                    as quest_hook::libil2cpp::Type > ::class(), "GetEditor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, (instance, editorBaseType))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetEditorAttribute(
        attributes: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::AttributeCollection,
        >,
        editorBaseType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::EditorAttribute>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::ComponentModel::AttributeCollection,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::EditorAttribute,
                >,
                2usize,
            >("GetEditorAttribute")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData
                    as quest_hook::libil2cpp::Type > ::class(), "GetEditorAttribute",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EditorAttribute,
        > = unsafe { method.invoke_unchecked((), (attributes, editorBaseType))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetEvents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptorCollection,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::EventDescriptorCollection,
                >,
                0usize,
            >("GetEvents")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData
                    as quest_hook::libil2cpp::Type > ::class(), "GetEvents", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptorCollection,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetProperties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::PropertyDescriptorCollection,
                >,
                0usize,
            >("GetProperties")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData
                    as quest_hook::libil2cpp::Type > ::class(), "GetProperties", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeFromName(
        &mut self,
        typeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                1usize,
            >("GetTypeFromName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData
                    as quest_hook::libil2cpp::Type > ::class(), "GetTypeFromName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            method.invoke_unchecked(self, (typeName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type))?;
        Ok(__cordl_object.into())
    }
    pub fn Refresh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Refresh")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData
                    as quest_hook::libil2cpp::Type > ::class(), "Refresh", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (_cordl_type))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsPopulated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_IsPopulated")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData
                    as quest_hook::libil2cpp::Type > ::class(), "get_IsPopulated", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "System+ComponentModel+ReflectTypeDescriptionProvider+ReflectedTypeData"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
