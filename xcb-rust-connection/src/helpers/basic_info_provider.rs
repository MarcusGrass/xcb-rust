use xcb_rust_protocol::util::{ExtensionInfoProvider, ExtensionInformation};

#[derive(Default, Debug)]
pub struct BasicExtensionInfoProvider {
    pub(crate) extensions: alloc::vec::Vec<(&'static str, ExtensionInformation)>,
}

impl ExtensionInfoProvider for BasicExtensionInfoProvider {
    #[inline]
    fn get_by_name(&self, name: &str) -> Option<ExtensionInformation> {
        self.extensions
            .iter()
            .find_map(|(ext_name, ext)| (*ext_name == name).then_some(*ext))
    }

    #[inline]
    fn get_from_major_opcode(
        &self,
        major_opcode: u8,
    ) -> Option<(&'static str, ExtensionInformation)> {
        for (name, ext_info) in &self.extensions {
            if ext_info.major_opcode == major_opcode {
                return Some((name, *ext_info));
            }
        }
        None
    }

    #[inline]
    fn get_from_event_code(&self, event_code: u8) -> Option<(&'static str, ExtensionInformation)> {
        self.extensions
            .iter()
            .filter_map(|(name, ext_info)| {
                (ext_info.first_event <= event_code).then_some((*name, *ext_info))
            })
            .max_by_key(|i| i.1.first_event)
    }

    #[inline]
    fn get_from_error_code(&self, error_code: u8) -> Option<(&'static str, ExtensionInformation)> {
        self.extensions
            .iter()
            .filter_map(|(name, ext_info)| {
                (ext_info.first_error <= error_code).then_some((*name, *ext_info))
            })
            .max_by_key(|i| i.1.first_error)
    }
}
