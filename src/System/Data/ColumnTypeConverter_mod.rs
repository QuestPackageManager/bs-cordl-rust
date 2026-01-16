#[cfg(feature = "cordl_class_System+Data+ColumnTypeConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct ColumnTypeConverter {
    __cordl_parent: crate::System::ComponentModel::TypeConverter,
    pub _values: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::TypeConverter_StandardValuesCollection,
    >,
}
#[cfg(feature = "cordl_class_System+Data+ColumnTypeConverter")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Data::ColumnTypeConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Data";
    const CLASS_NAME: &'static str = "ColumnTypeConverter";
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
#[cfg(feature = "System+Data+ColumnTypeConverter")]
impl std::ops::Deref for crate::System::Data::ColumnTypeConverter {
    type Target = crate::System::ComponentModel::TypeConverter;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+ColumnTypeConverter")]
impl std::ops::DerefMut for crate::System::Data::ColumnTypeConverter {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+ColumnTypeConverter")]
impl crate::System::Data::ColumnTypeConverter {
    pub fn CanConvertFrom(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::System::ComponentModel::ITypeDescriptorContext>,
        sourceType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::ComponentModel::ITypeDescriptorContext,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                    ), bool, 2usize>("CanConvertFrom")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CanConvertFrom",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (context, sourceType))? };
        Ok(__cordl_ret.into())
    }
    pub fn CanConvertTo(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::System::ComponentModel::ITypeDescriptorContext>,
        destinationType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::ComponentModel::ITypeDescriptorContext,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                    ), bool, 2usize>("CanConvertTo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CanConvertTo",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (context, destinationType))? };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertFrom(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::System::ComponentModel::ITypeDescriptorContext>,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::ComponentModel::ITypeDescriptorContext,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, 3usize>(
                        "ConvertFrom",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ConvertFrom",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject> =
            unsafe { cordl_method_info.invoke_unchecked(self, (context, culture, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertTo(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::System::ComponentModel::ITypeDescriptorContext>,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destinationType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::ComponentModel::ITypeDescriptorContext,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                    ), quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, 4usize>(
                        "ConvertTo",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ConvertTo",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject> = unsafe {
            cordl_method_info.invoke_unchecked(self, (context, culture, value, destinationType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetStandardValues(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::System::ComponentModel::ITypeDescriptorContext>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::TypeConverter_StandardValuesCollection,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::ComponentModel::ITypeDescriptorContext,
                    >), quest_hook::libil2cpp::Gc<
                        crate::System::ComponentModel::TypeConverter_StandardValuesCollection,
                    >, 1usize>("GetStandardValues")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetStandardValues",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::TypeConverter_StandardValuesCollection,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (context))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetStandardValuesExclusive(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::System::ComponentModel::ITypeDescriptorContext>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::ComponentModel::ITypeDescriptorContext,
                    >), bool, 1usize>("GetStandardValuesExclusive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetStandardValuesExclusive",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (context))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetStandardValuesSupported(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::System::ComponentModel::ITypeDescriptorContext>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::ComponentModel::ITypeDescriptorContext,
                    >), bool, 1usize>("GetStandardValuesSupported")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetStandardValuesSupported",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (context))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
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
#[cfg(feature = "cordl_class_System+Data+ColumnTypeConverter")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::ColumnTypeConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
