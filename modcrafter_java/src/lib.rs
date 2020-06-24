use std::fmt::{self, Display, Formatter};

pub enum JavaType<'class> {
    Int,
    Float,
    Double,
    Byte,
    Long,
    Short,
    Instance { class_name: &'class str },
    Array { typ: Box<JavaType<'class>> },
    Bool,
    Char,
}

pub enum JavaAccessLevel {
    Public,
    Private,
    Protected,
}

pub struct JavaField<'class> {
    identifier: &'class str,
    value: &'class str,
    typ: JavaType<'class>,
    access_level: JavaAccessLevel,
}

pub struct JavaMethod<'class> {
    identifier: &'class str,
    return_type: JavaType<'class>,
    access_level: JavaAccessLevel,
    parameter: Vec<(JavaType<'class>, &'class str)>,
    body: &'class str,
}

pub struct JavaClass<'class> {
    package: &'class str,
    class_name: &'class str,
    super_class: Option<&'class str>,
    interfaces: Vec<&'class str>,
    class_members: Vec<JavaField<'class>>,
    class_methods: Vec<JavaMethod<'class>>,
}

impl<'class> JavaClass<'class> {
    pub fn new(
        package: &'class str,
        class_name: &'class str,
        super_class: Option<&'class str>,
        interfaces: Vec<&'class str>,
        class_members: Vec<JavaField<'class>>,
        class_methods: Vec<JavaMethod<'class>>,
    ) -> Self {
        JavaClass {
            package,
            class_name,
            super_class,
            interfaces,
            class_members,
            class_methods,
        }
    }
}

impl<'class> JavaField<'class> {
    pub fn new(
        identifier: &'class str,
        value: &'class str,
        typ: JavaType<'class>,
        access_level: JavaAccessLevel,
    ) -> Self {
        JavaField {
            identifier,
            value,
            typ,
            access_level,
        }
    }
}

impl<'class> Display for JavaType<'class> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            JavaType::Float => write!(f, "float"),
            JavaType::Bool => write!(f, "bool"),
            _ => write!(f, "undefined"),
        }
    }
}

impl<'class> Display for JavaAccessLevel {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            JavaAccessLevel::Public => write!(f, "public"),
            JavaAccessLevel::Private => write!(f, "private"),
            JavaAccessLevel::Protected => write!(f, "protected"),
        }
    }
}

impl<'class> Display for JavaField<'class> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {} = {};",
            self.access_level, self.typ, self.identifier, self.value
        )
    }
}

impl<'class> Display for JavaClass<'class> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "package {};", self.package)?;
        writeln!(f, "")?;
        writeln!(f, "import none;")?;
        writeln!(f, "")?;
        write!(f, "public class {} ", self.class_name)?;
        if let Some(super_class) = self.super_class {
            write!(f, "extends {} ", super_class)?;
        }

        if self.interfaces.len() > 0 {
            write!(f, "implements {} ", self.interfaces.join(", "))?;
        }
        writeln!(f, "{{")?;

        for var in &self.class_members {
            writeln!(f, "    {}", var)?;
        }

        writeln!(f, "}}")
    }
}
