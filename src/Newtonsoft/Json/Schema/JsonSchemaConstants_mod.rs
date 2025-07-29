#[cfg(feature = "cordl_class_Newtonsoft+Json+Schema+JsonSchemaConstants")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonSchemaConstants {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Newtonsoft+Json+Schema+JsonSchemaConstants")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Schema::JsonSchemaConstants {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Schema";
    const CLASS_NAME: &'static str = "JsonSchemaConstants";
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
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaConstants")]
impl std::ops::Deref for crate::Newtonsoft::Json::Schema::JsonSchemaConstants {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaConstants")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Schema::JsonSchemaConstants {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaConstants")]
impl crate::Newtonsoft::Json::Schema::JsonSchemaConstants {
    pub const AdditionalItemsPropertyName: &'static str = "additionalItems";
    pub const AdditionalPropertiesPropertyName: &'static str = "additionalProperties";
    pub const DefaultPropertyName: &'static str = "default";
    pub const DescriptionPropertyName: &'static str = "description";
    pub const DisallowPropertyName: &'static str = "disallow";
    pub const DivisibleByPropertyName: &'static str = "divisibleBy";
    pub const EnumPropertyName: &'static str = "enum";
    pub const ExclusiveMaximumPropertyName: &'static str = "exclusiveMaximum";
    pub const ExclusiveMinimumPropertyName: &'static str = "exclusiveMinimum";
    pub const ExtendsPropertyName: &'static str = "extends";
    pub const FormatPropertyName: &'static str = "format";
    pub const HiddenPropertyName: &'static str = "hidden";
    pub const IdPropertyName: &'static str = "id";
    pub const ItemsPropertyName: &'static str = "items";
    pub const MaximumItemsPropertyName: &'static str = "maxItems";
    pub const MaximumLengthPropertyName: &'static str = "maxLength";
    pub const MaximumPropertyName: &'static str = "maximum";
    pub const MinimumItemsPropertyName: &'static str = "minItems";
    pub const MinimumLengthPropertyName: &'static str = "minLength";
    pub const MinimumPropertyName: &'static str = "minimum";
    pub const OptionLabelPropertyName: &'static str = "label";
    pub const OptionValuePropertyName: &'static str = "value";
    pub const PatternPropertiesPropertyName: &'static str = "patternProperties";
    pub const PatternPropertyName: &'static str = "pattern";
    pub const PropertiesPropertyName: &'static str = "properties";
    pub const ReadOnlyPropertyName: &'static str = "readonly";
    pub const RequiredPropertyName: &'static str = "required";
    pub const RequiresPropertyName: &'static str = "requires";
    pub const TitlePropertyName: &'static str = "title";
    pub const TransientPropertyName: &'static str = "transient";
    pub const TypePropertyName: &'static str = "type";
    pub const UniqueItemsPropertyName: &'static str = "uniqueItems";
}
#[cfg(feature = "cordl_class_Newtonsoft+Json+Schema+JsonSchemaConstants")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Schema::JsonSchemaConstants {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
