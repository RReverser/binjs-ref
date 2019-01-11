#[derive(Default, Display, Add, AddAssign, Into, From, Clone, Copy)]
pub struct Bytes(usize);

impl std::iter::Sum for Bytes {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Bytes>,
    {
        iter.fold(Default::default(), std::ops::Add::add)
    }
}

/// A newtype for `usize` used to count the number of instances of some item.
#[derive(Default, Display, Serialize, Deserialize, From, Into, Add, AddAssign, Clone, Copy)]
pub struct Instances(usize);

impl std::iter::Sum for Instances {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Instances>,
    {
        iter.fold(Default::default(), std::ops::Add::add)
    }
}

pub struct BytesAndInstances {
    bytes: Bytes,
    instances: Instances,
}
impl BytesAndInstances {
    pub fn new(bytes: Bytes, instances: Instances) -> Self {
        BytesAndInstances { bytes, instances }
    }
}
/// A container for information associated with a type of data we write to the stream
/// as part of the content (i.e. not the header).
///
/// Typically used to collect/display the number of bytes written in each category.
#[derive(Debug, Default, Add, Clone, AddAssign)]
pub struct ContentInfo<T> {
    pub bools: T,
    pub floats: T,
    pub unsigned_longs: T,
    pub string_enums: T,
    pub property_keys: T,
    pub identifier_names: T,
    pub interface_names: T,
    pub string_literals: T,
    pub list_lengths: T,
}
impl<T> ContentInfo<T> {
    /// Initialize a new `ContentInfo`.
    pub fn with<F>(f: F) -> Self
    where
        F: Fn(&str) -> T,
    {
        ContentInfo {
            bools: f("bools"),
            floats: f("floats"),
            unsigned_longs: f("unsigned_longs"),
            string_enums: f("string_enums"),
            property_keys: f("property_keys"),
            identifier_names: f("identifier_names"),
            interface_names: f("interface_names"),
            string_literals: f("string_literals"),
            list_lengths: f("list_lengths"),
        }
    }

    /// Convert a `ContentInfo` into another one.
    pub fn into_with<F, U>(self, f: F) -> ContentInfo<U>
    where
        F: Fn(&str, T) -> U,
    {
        ContentInfo {
            bools: f("bools", self.bools),
            floats: f("floats", self.floats),
            unsigned_longs: f("unsigned_longs", self.unsigned_longs),
            string_enums: f("string_enums", self.string_enums),
            property_keys: f("property_keys", self.property_keys),
            identifier_names: f("identifier_names", self.identifier_names),
            interface_names: f("interface_names", self.interface_names),
            string_literals: f("string_literals", self.string_literals),
            list_lengths: f("list_lengths", self.list_lengths),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &T)> {
        vec![
            ("bools", &self.bools),
            ("floats", &self.floats),
            ("unsigned_longs", &self.unsigned_longs),
            ("string_enums", &self.string_enums),
            ("property_keys", &self.property_keys),
            ("identifier_names", &self.identifier_names),
            ("interface_names", &self.interface_names),
            ("string_literals", &self.string_literals),
            ("list_lengths", &self.list_lengths),
        ]
        .into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&'static str, &mut T)> {
        vec![
            ("bools", &mut self.bools),
            ("floats", &mut self.floats),
            ("unsigned_longs", &mut self.unsigned_longs),
            ("string_enums", &mut self.string_enums),
            ("property_keys", &mut self.property_keys),
            ("identifier_names", &mut self.identifier_names),
            ("interface_names", &mut self.interface_names),
            ("string_literals", &mut self.string_literals),
            ("list_lengths", &mut self.list_lengths),
        ]
        .into_iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item = (&'static str, T)> {
        vec![
            ("bools", self.bools),
            ("floats", self.floats),
            ("unsigned_longs", self.unsigned_longs),
            ("string_enums", self.string_enums),
            ("property_keys", self.property_keys),
            ("identifier_names", self.identifier_names),
            ("interface_names", self.interface_names),
            ("string_literals", self.string_literals),
            ("list_lengths", self.list_lengths),
        ]
        .into_iter()
    }
}

pub trait DisplayWith<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter, data: &T) -> Result<(), std::fmt::Error>;
}

impl DisplayWith</* Total */ BytesAndInstances> for BytesAndInstances {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter,
        total: &BytesAndInstances,
    ) -> Result<(), std::fmt::Error> {
        let bytes = Into::<usize>::into(self.bytes);
        let symbols = Into::<usize>::into(self.instances);
        let total_bytes = Into::<usize>::into(total.bytes);
        let total_symbols = Into::<usize>::into(total.instances);
        write!(formatter, "symbols {symbols} = {symbols_percent:.2}, bytes {bytes} = {bytes_percent:.2} ({bits_per_symbol:.2} bits/symbol)",
            symbols = symbols,
            bytes = bytes,
            symbols_percent = 100.* symbols as f64 / total_symbols as f64,
            bytes_percent = 100.* bytes as f64 / total_bytes as f64,
            bits_per_symbol = 8. * bytes as f64 / symbols as f64,
        )
    }
}

impl std::fmt::Display for ContentInfo<BytesAndInstances> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let total = BytesAndInstances {
            bytes: self.iter().map(|(_, data)| data.bytes.clone()).sum(),
            instances: self.iter().map(|(_, data)| data.instances.clone()).sum(),
        };

        write!(formatter, "Content:\n  Fixed:\n")?;
        for (field, name) in &[
            (&self.bools, "bools"),
            (&self.string_enums, "string enums"),
            (&self.interface_names, "interface names"),
        ] {
            write!(formatter, "    {name}: ", name = name)?;
            field.fmt(formatter, &total)?;
            write!(formatter, "\n")?;
        }
        write!(formatter, "  User-extensible:\n")?;
        for (field, name) in &[
            (&self.property_keys, "property_keys"),
            (&self.identifier_names, "identifier names"),
            (&self.string_literals, "string literals"),
            (&self.floats, "floats"),
            (&self.unsigned_longs, "unsigned longs"),
            (&self.list_lengths, "list lengths"),
        ] {
            write!(formatter, "    {name}: ", name = name)?;
            field.fmt(formatter, &total)?;
            write!(formatter, "\n")?;
        }
        write!(
            formatter,
            "Total: {} bytes, {} symbols",
            total.bytes, total.instances
        )?;
        Ok(())
    }
}
