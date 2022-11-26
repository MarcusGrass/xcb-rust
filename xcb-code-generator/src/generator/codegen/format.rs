pub trait FormatBytes {
    fn dynamic_get_len(&self, len_var: &str) -> String;
}
