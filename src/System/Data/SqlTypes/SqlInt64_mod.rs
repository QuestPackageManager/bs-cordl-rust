#[cfg(feature = "cordl_class_System+Data+SqlTypes+SqlInt64")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct SqlInt64 {
    pub m_fNotNull: bool,
    pub m_value: i64,
}
#[cfg(feature = "cordl_class_System+Data+SqlTypes+SqlInt64")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Data::SqlTypes::SqlInt64 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Data.SqlTypes";
    const CLASS_NAME: &'static str = "SqlInt64";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_System+Data+SqlTypes+SqlInt64")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::Data::SqlTypes::SqlInt64 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+Data+SqlTypes+SqlInt64")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::Data::SqlTypes::SqlInt64 {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_System+Data+SqlTypes+SqlInt64")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::Data::SqlTypes::SqlInt64 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_System+Data+SqlTypes+SqlInt64")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::Data::SqlTypes::SqlInt64 {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_System+Data+SqlTypes+SqlInt64")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::Data::SqlTypes::SqlInt64 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlInt64")]
impl crate::System::Data::SqlTypes::SqlInt64 {
    pub fn CompareTo_Il2CppObject0(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        i32,
                        1usize,
                    >("CompareTo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CompareTo", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_SqlInt64_1(
        &mut self,
        value: crate::System::Data::SqlTypes::SqlInt64,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::System::Data::SqlTypes::SqlInt64), i32, 1usize>(
                        "CompareTo",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CompareTo",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetHashCode",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetXsdType(
        schemaSet: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSet>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaSet,
                        >),
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
                        1usize,
                    >("GetXsdType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetXsdType", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName> =
            unsafe { cordl_method_info.invoke_unchecked((), (schemaSet))? };
        Ok(__cordl_ret.into())
    }
    pub fn GreaterThan(
        x: crate::System::Data::SqlTypes::SqlInt64,
        y: crate::System::Data::SqlTypes::SqlInt64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBoolean> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::Data::SqlTypes::SqlInt64,
                        crate::System::Data::SqlTypes::SqlInt64,
                    ), crate::System::Data::SqlTypes::SqlBoolean, 2usize>(
                        "GreaterThan"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GreaterThan",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBoolean =
            unsafe { cordl_method_info.invoke_unchecked((), (x, y))? };
        Ok(__cordl_ret.into())
    }
    pub fn LessThan(
        x: crate::System::Data::SqlTypes::SqlInt64,
        y: crate::System::Data::SqlTypes::SqlInt64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBoolean> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::Data::SqlTypes::SqlInt64,
                        crate::System::Data::SqlTypes::SqlInt64,
                    ), crate::System::Data::SqlTypes::SqlBoolean, 2usize>(
                        "LessThan"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LessThan",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBoolean =
            unsafe { cordl_method_info.invoke_unchecked((), (x, y))? };
        Ok(__cordl_ret.into())
    }
    pub fn SameSignLong(x: i64, y: i64) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i64, i64), bool, 2usize>("SameSignLong")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SameSignLong",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (x, y))? };
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_Serialization_IXmlSerializable_GetSchema(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
                        0usize,
                    >("System.Xml.Serialization.IXmlSerializable.GetSchema")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.Xml.Serialization.IXmlSerializable.GetSchema", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_Serialization_IXmlSerializable_ReadXml(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("System.Xml.Serialization.IXmlSerializable.ReadXml")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.Xml.Serialization.IXmlSerializable.ReadXml", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (reader))? };
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_Serialization_IXmlSerializable_WriteXml(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("System.Xml.Serialization.IXmlSerializable.WriteXml")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.Xml.Serialization.IXmlSerializable.WriteXml", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (writer))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToSqlByte(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlByte> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::System::Data::SqlTypes::SqlByte, 0usize>("ToSqlByte")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ToSqlByte",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlByte =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ToSqlDecimal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDecimal> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::System::Data::SqlTypes::SqlDecimal, 0usize>(
                        "ToSqlDecimal",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ToSqlDecimal",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDecimal =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ToSqlDouble(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDouble> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::System::Data::SqlTypes::SqlDouble, 0usize>(
                        "ToSqlDouble",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ToSqlDouble",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDouble =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ToSqlInt16(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlInt16> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::System::Data::SqlTypes::SqlInt16, 0usize>(
                        "ToSqlInt16",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ToSqlInt16",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlInt16 =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ToSqlInt32(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlInt32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::System::Data::SqlTypes::SqlInt32, 0usize>(
                        "ToSqlInt32",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ToSqlInt32",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlInt32 =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool0(
        &mut self,
        fNull: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (fNull))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i64_1(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i64), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsNull(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_IsNull")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_IsNull",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i64, 0usize>("get_Value")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_Value",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i64 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition(
        x: crate::System::Data::SqlTypes::SqlInt64,
        y: crate::System::Data::SqlTypes::SqlInt64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlInt64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::Data::SqlTypes::SqlInt64,
                        crate::System::Data::SqlTypes::SqlInt64,
                    ), crate::System::Data::SqlTypes::SqlInt64, 2usize>(
                        "op_Addition"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Addition",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlInt64 =
            unsafe { cordl_method_info.invoke_unchecked((), (x, y))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Division(
        x: crate::System::Data::SqlTypes::SqlInt64,
        y: crate::System::Data::SqlTypes::SqlInt64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlInt64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::Data::SqlTypes::SqlInt64,
                        crate::System::Data::SqlTypes::SqlInt64,
                    ), crate::System::Data::SqlTypes::SqlInt64, 2usize>(
                        "op_Division"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Division",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlInt64 =
            unsafe { cordl_method_info.invoke_unchecked((), (x, y))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        x: crate::System::Data::SqlTypes::SqlInt64,
        y: crate::System::Data::SqlTypes::SqlInt64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBoolean> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::Data::SqlTypes::SqlInt64,
                        crate::System::Data::SqlTypes::SqlInt64,
                    ), crate::System::Data::SqlTypes::SqlBoolean, 2usize>(
                        "op_Equality"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Equality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBoolean =
            unsafe { cordl_method_info.invoke_unchecked((), (x, y))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit(
        x: crate::System::Data::SqlTypes::SqlDecimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlInt64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::Data::SqlTypes::SqlDecimal),
                        crate::System::Data::SqlTypes::SqlInt64,
                        1usize,
                    >("op_Explicit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_Explicit", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlInt64 =
            unsafe { cordl_method_info.invoke_unchecked((), (x))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan(
        x: crate::System::Data::SqlTypes::SqlInt64,
        y: crate::System::Data::SqlTypes::SqlInt64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBoolean> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::Data::SqlTypes::SqlInt64,
                        crate::System::Data::SqlTypes::SqlInt64,
                    ), crate::System::Data::SqlTypes::SqlBoolean, 2usize>(
                        "op_GreaterThan"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_GreaterThan",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBoolean =
            unsafe { cordl_method_info.invoke_unchecked((), (x, y))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_SqlByte1(
        x: crate::System::Data::SqlTypes::SqlByte,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlInt64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::Data::SqlTypes::SqlByte),
                        crate::System::Data::SqlTypes::SqlInt64,
                        1usize,
                    >("op_Implicit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_Implicit", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlInt64 =
            unsafe { cordl_method_info.invoke_unchecked((), (x))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_SqlInt16_2(
        x: crate::System::Data::SqlTypes::SqlInt16,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlInt64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::Data::SqlTypes::SqlInt16),
                        crate::System::Data::SqlTypes::SqlInt64,
                        1usize,
                    >("op_Implicit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_Implicit", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlInt64 =
            unsafe { cordl_method_info.invoke_unchecked((), (x))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_SqlInt32_3(
        x: crate::System::Data::SqlTypes::SqlInt32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlInt64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::Data::SqlTypes::SqlInt32),
                        crate::System::Data::SqlTypes::SqlInt64,
                        1usize,
                    >("op_Implicit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_Implicit", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlInt64 =
            unsafe { cordl_method_info.invoke_unchecked((), (x))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_i64_0(
        x: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlInt64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i64), crate::System::Data::SqlTypes::SqlInt64, 1usize>(
                        "op_Implicit",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Implicit",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlInt64 =
            unsafe { cordl_method_info.invoke_unchecked((), (x))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan(
        x: crate::System::Data::SqlTypes::SqlInt64,
        y: crate::System::Data::SqlTypes::SqlInt64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBoolean> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::Data::SqlTypes::SqlInt64,
                        crate::System::Data::SqlTypes::SqlInt64,
                    ), crate::System::Data::SqlTypes::SqlBoolean, 2usize>(
                        "op_LessThan"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_LessThan",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBoolean =
            unsafe { cordl_method_info.invoke_unchecked((), (x, y))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus(
        x: crate::System::Data::SqlTypes::SqlInt64,
        y: crate::System::Data::SqlTypes::SqlInt64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlInt64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::Data::SqlTypes::SqlInt64,
                        crate::System::Data::SqlTypes::SqlInt64,
                    ), crate::System::Data::SqlTypes::SqlInt64, 2usize>(
                        "op_Modulus"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Modulus",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlInt64 =
            unsafe { cordl_method_info.invoke_unchecked((), (x, y))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply(
        x: crate::System::Data::SqlTypes::SqlInt64,
        y: crate::System::Data::SqlTypes::SqlInt64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlInt64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::Data::SqlTypes::SqlInt64,
                        crate::System::Data::SqlTypes::SqlInt64,
                    ), crate::System::Data::SqlTypes::SqlInt64, 2usize>(
                        "op_Multiply"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Multiply",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlInt64 =
            unsafe { cordl_method_info.invoke_unchecked((), (x, y))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction(
        x: crate::System::Data::SqlTypes::SqlInt64,
        y: crate::System::Data::SqlTypes::SqlInt64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlInt64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::Data::SqlTypes::SqlInt64,
                        crate::System::Data::SqlTypes::SqlInt64,
                    ), crate::System::Data::SqlTypes::SqlInt64, 2usize>(
                        "op_Subtraction"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Subtraction",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlInt64 =
            unsafe { cordl_method_info.invoke_unchecked((), (x, y))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryNegation(
        x: crate::System::Data::SqlTypes::SqlInt64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlInt64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::Data::SqlTypes::SqlInt64),
                        crate::System::Data::SqlTypes::SqlInt64,
                        1usize,
                    >("op_UnaryNegation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_UnaryNegation", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlInt64 =
            unsafe { cordl_method_info.invoke_unchecked((), (x))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlInt64")]
impl AsRef<crate::System::Data::SqlTypes::INullable> for crate::System::Data::SqlTypes::SqlInt64 {
    fn as_ref(&self) -> &crate::System::Data::SqlTypes::INullable {
        todo!()
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlInt64")]
impl AsMut<crate::System::Data::SqlTypes::INullable> for crate::System::Data::SqlTypes::SqlInt64 {
    fn as_mut(&mut self) -> &mut crate::System::Data::SqlTypes::INullable {
        todo!()
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlInt64")]
impl AsRef<crate::System::IComparable> for crate::System::Data::SqlTypes::SqlInt64 {
    fn as_ref(&self) -> &crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlInt64")]
impl AsMut<crate::System::IComparable> for crate::System::Data::SqlTypes::SqlInt64 {
    fn as_mut(&mut self) -> &mut crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlInt64")]
impl AsRef<crate::System::Xml::Serialization::IXmlSerializable>
    for crate::System::Data::SqlTypes::SqlInt64
{
    fn as_ref(&self) -> &crate::System::Xml::Serialization::IXmlSerializable {
        todo!()
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlInt64")]
impl AsMut<crate::System::Xml::Serialization::IXmlSerializable>
    for crate::System::Data::SqlTypes::SqlInt64
{
    fn as_mut(&mut self) -> &mut crate::System::Xml::Serialization::IXmlSerializable {
        todo!()
    }
}
