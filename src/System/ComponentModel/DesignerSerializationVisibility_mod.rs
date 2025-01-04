#[cfg(feature = "System+ComponentModel+DesignerSerializationVisibility")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DesignerSerializationVisibility {
    #[default]
    Content = 2i32,
    Hidden = 0i32,
    Visible = 1i32,
}
#[cfg(feature = "System+ComponentModel+DesignerSerializationVisibility")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::DesignerSerializationVisibility => "System.ComponentModel"
    ."DesignerSerializationVisibility"
);
