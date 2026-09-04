#[macro_export]
macro_rules! impl_app_desktop_methods {
    ($wrapper:ident, $bound:path) => {
        impl<T> std::ops::Deref for $wrapper<T>
        where
            T: $bound,
        {
            type Target = Desktop;
            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                &self.base
            }
        }

        impl<T> std::ops::DerefMut for $wrapper<T>
        where
            T: $bound,
        {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.base
            }
        }
        $crate::impl_app_desktop_methods!(@empty $wrapper, $bound;
            Control,
            DesktopControl,
            OnWindowRegistered,
            OnDefaultAction,
            OnFocus,
            OnExpand,
            OnSiblingSelected,
            OnThemeChanged,
            ButtonEvents,
            CheckBoxEvents,
            WindowEvents,
            GenericCommandBarEvents,
            GenericMenuEvents,
            ToolBarEvents,
            ColorPickerEvents,
            ThreeStateBoxEvents,
            RadioBoxEvents,
            PasswordEvents,
            KeySelectorEvents,
            TextFieldEvents,
            CustomEvents,
            GenericSelectorEvents,
            ComboBoxEvents,
            GenericDropDownListEvents,
            GenericNumericSelectorEvents,
            DatePickerEvents,
            ListBoxEvents,
            GenericListViewEvents,
            ToggleButtonEvents,
            PathFinderEvents,
            GenericTreeViewEvents,
            MarkdownEvents,
            GenericBackgroundTaskEvents,
            AccordionEvents,
            TabEvents,
            CharPickerEvents,
            GenericGraphViewEvents,
            AppBarEvents,
            TimePickerEvents,
            RichTextFieldEvents,
            GenericBufferViewEvents,
            HyperLinkEvents,
            GenericHSliderEvents,
            EditorEvents,
        );
    };
    (@empty $wrapper:ident, $bound:path; $($trait_name:path),* $(,)?) => {
        $(
            impl<T> $trait_name for $wrapper<T> where T: $bound {}
        )*
    };
}

pub(crate) use impl_app_desktop_methods;